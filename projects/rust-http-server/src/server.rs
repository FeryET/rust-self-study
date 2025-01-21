use std::{
    collections::HashMap,
    io::{self, BufReader, BufWriter, Error, Write},
    net::{TcpListener, TcpStream},
    thread,
};

use crate::{
    common::{HttpError, HttpStatus},
    message::{
        create_http_response_message, parse_http_request, HttpHeaders, HttpRequest, HttpResponse,
    },
};

type HandlerFunc = fn(&HttpRequest) -> Result<HttpResponse, HttpError>;
pub struct HttpServer {
    pub tcp_listener: TcpListener,
    pub handlers: HashMap<String, HandlerFunc>,
}

impl HttpServer {
    pub fn bind(host: &str, port: u16) -> Result<Self, std::io::Error> {
        match TcpListener::bind(format!("{host}:{port}")) {
            Ok(listener) => Ok(HttpServer {
                tcp_listener: listener,
                handlers: HashMap::new(),
            }),
            Err(e) => Err(e),
        }
    }
    fn choose_handler<'a>(
        handlers: &'a HashMap<String, HandlerFunc>,
        uri: &String,
    ) -> Result<&'a HandlerFunc, HttpError> {
        let handler = handlers
            .iter()
            .filter_map(|x| {
                if uri.starts_with(x.0) {
                    Some(x.1)
                } else {
                    None
                }
            })
            .last();
        if let Some(h) = handler {
            Ok(h)
        } else {
            Err(HttpError::new(HttpStatus::NotFound, ""))
        }
    }
    fn write_response(
        response: &HttpResponse,
        writer: &mut BufWriter<&TcpStream>,
    ) -> io::Result<usize> {
        let bytes = format!("{}", response).bytes().collect::<Vec<u8>>();
        writer.write(&bytes)
    }
    fn handle(handlers: &HashMap<String, HandlerFunc>, stream: &TcpStream) -> Result<(), ()> {
        let request = parse_http_request(&mut BufReader::new(stream));
        let handler = match request.as_ref() {
            Ok(r) => HttpServer::choose_handler(handlers, &r.metadata.uri),
            Err(e) => {
                println!("request parse err: {e}");
                let err_response = create_http_response_message(
                    crate::common::HttpProtocol::Http1_1,
                    e.status,
                    HttpHeaders::new(),
                    None,
                );
                let _ = Self::write_response(&err_response, &mut BufWriter::new(&stream));
                return Err(());
            }
        };
        let request = request.unwrap();
        let protocol: crate::common::HttpProtocol = request.metadata.protocol;
        let response = match handler {
            Ok(h) => (h)(&request),
            Err(e) => {
                println!("handler err: {e}");
                let err_response =
                    create_http_response_message(protocol, e.status, HttpHeaders::new(), None);
                let _ = Self::write_response(&err_response, &mut BufWriter::new(&stream));
                return Err(());
            }
        };
        match response {
            Ok(r) => {
                let _ = Self::write_response(&r, &mut BufWriter::new(&stream));
            }
            Err(e) => {
                let err_response =
                    create_http_response_message(protocol, e.status, HttpHeaders::new(), None);
                let _ = Self::write_response(&err_response, &mut BufWriter::new(&stream));
                return Err(());
            }
        }
        return Ok(());
    }
    pub fn serve(self: &Self) {
        let _ = for stream in self.tcp_listener.incoming() {
            let _ = match stream {
                Ok(s) => {
                    let handlers = self.handlers.clone();
                    let _ = thread::spawn(move || Self::handle(&handlers, &s));
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
    use crate::common::HttpProtocol;

    use super::*;
    use serial_test::serial;
    use std::io::Write;
    use std::thread;

    const HOST: &str = "127.0.0.1";
    const PORT: u16 = 38080;
    const BIND_ADDRESS: &str = "127.0.0.1:38080";

    #[test]
    #[serial]
    fn test_bind() {
        let server = HttpServer::bind(HOST, PORT);
        assert!(server.is_ok());
    }

    fn _test_root_handler(_: &HttpRequest) -> Result<HttpResponse, HttpError> {
        Ok(create_http_response_message(
            HttpProtocol::Http1,
            HttpStatus::Ok,
            HttpHeaders::new(),
            None,
        ))
    }

    #[test]
    #[serial]
    fn test_handle_requests() {
        let listener = TcpListener::bind(BIND_ADDRESS).unwrap();
        let root_path = "/".to_string();
        let mut handlers: HashMap<String, fn(&HttpRequest) -> Result<HttpResponse, HttpError>> =
            HashMap::new();
        handlers.insert(root_path, _test_root_handler);
        let server = HttpServer {
            tcp_listener: listener,
            handlers: handlers,
        };

        thread::spawn(move || {
            let mut stream = TcpStream::connect(BIND_ADDRESS).unwrap();
            let request = "GET / HTTP/1.1\r\nHost: localhost\r\n\r\n";
            stream.write_all(request.as_bytes()).unwrap();
        });

        for mut stream in server.tcp_listener.incoming() {
            let result = HttpServer::handle(&server.handlers, &mut stream.unwrap());
            assert!(result.is_ok());
            break;
        }
    }
}
