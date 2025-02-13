use std::{
    io::{BufReader, BufWriter, Write},
    net::{TcpListener, TcpStream},
    sync::{Arc, RwLock},
    thread,
};

use log::error;

use crate::{
    common::{HttpError, HttpStatus},
    request::parse_http_request,
    response::HttpResponse,
    router::HttpRouter,
};

pub struct HttpServer {
    router: HttpRouter,
}

impl HttpServer {
    pub fn new(router: HttpRouter) -> HttpServer {
        HttpServer { router: router }
    }
    fn write_response_to_stream(stream: &TcpStream, response: HttpResponse) {
        let mut writer = BufWriter::new(stream);
        let bytes = format!("{}", response).bytes().collect::<Vec<u8>>();
        let _ = writer.write(&bytes);
    }
    fn handle_incoming_stream(router: Arc<RwLock<HttpRouter>>, s: &TcpStream) {
        let request = match parse_http_request(&mut BufReader::new(s)) {
            Ok(r) => r,
            Err(e) => {
                error!("HttpServer: parse request error: {e}");
                Self::write_response_to_stream(&s, HttpResponse::from_err(e, None));
                return;
            }
        };
        let router = router.read().unwrap();
        let result = match router.parse_request_route(&request) {
            Some((handler, params)) => (handler)(&request, &params),
            None => {
                error!(
                    "HttpServer: request routing error # method: {} # uri: {}",
                    request.metadata.method, request.metadata.uri
                );
                Err(HttpError::new(HttpStatus::MethodNotAllowed, ""))
            }
        };
        Self::write_response_to_stream(
            &s,
            result.unwrap_or_else(|e| HttpResponse::from_err(e, Some(request.metadata.protocol))),
        );
    }
    pub fn serve(self: &Self, tcp_listener: &TcpListener) {
        let router = Arc::new(RwLock::new(self.router.clone()));
        let _ = for stream in tcp_listener.incoming() {
            let _ = match stream {
                Ok(s) => {
                    let router = Arc::clone(&router);
                    let _ = thread::spawn(move || {
                        Self::handle_incoming_stream(router, &s);
                    });
                }
                Err(e) => {
                    println!("critical: cannot read the tcp stream -> error: {e}");
                    return;
                }
            };
        };
    }
}

#[cfg(test)]
mod tests {
    use std::io::Read;

    use crate::{
        common::{HttpHeaders, HttpMethod, HttpProtocol, HttpServerContext},
        request::HttpRequest,
        router::HttpRouterBuilder,
    };

    use super::*;
    use serial_test::serial;

    const BIND_ADDRESS: &'static str = "127.0.0.1:38080";

    fn bind_tcp_listener() -> Result<TcpListener, Error> {
        TcpListener::bind(BIND_ADDRESS)
    }

    #[test]
    #[serial]
    fn test_tcp_listener() {
        assert!(bind_tcp_listener().is_ok())
    }

    #[test]
    #[serial]
    fn test_http_server_new() {
        HttpServer::new(HttpRouterBuilder::new().build());
    }

    #[test]
    #[serial]
    fn test_http_server_write_response() {
        let expected = HttpResponse::new(
            HttpProtocol::Http1,
            HttpStatus::Accepted,
            HttpHeaders::new(),
            None,
        );
        let expected_len = format!("{}", expected).bytes().len();
        let listener = bind_tcp_listener().unwrap();
        thread::spawn(|| {
            let stream = TcpStream::connect(BIND_ADDRESS).unwrap();
            HttpServer::write_response_to_stream(&stream, expected);
        });
        let (s, _) = listener.accept().unwrap();
        let bytes: Vec<u8> = BufReader::new(s)
            .bytes()
            .map(|x| x.unwrap())
            .collect::<Vec<u8>>();
        assert_eq!(bytes.len(), expected_len);
    }

    #[test]
    #[serial]
    fn test_http_server_handle_stream() {
        let request = "POST / HTTP/1.1\r\n\
                Header: Value\r\n\
                \r\n\
                Body";
        let listener = bind_tcp_listener();
        thread::spawn(|| {
            let stream = TcpStream::connect(BIND_ADDRESS).unwrap();
            BufWriter::new(stream)
                .write(&request.bytes().collect::<Vec<u8>>())
                .unwrap();
        });
        let stream = listener.unwrap().accept().unwrap().0;
        let router = HttpRouterBuilder::new()
            .add_route(
                HttpMethod::POST,
                "/",
                |x: &HttpRequest, y: &HttpServerContext| -> Result<HttpResponse, HttpError> {
                    Err(HttpError::new(HttpStatus::BadRequest, ""))
                },
            )
            .build();
        HttpServer::handle_incoming_stream(Arc::new(RwLock::new(router)), &stream);
    }
}
