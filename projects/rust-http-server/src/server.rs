use std::{
    collections::HashMap,
    io::{self, BufReader, BufWriter, Error, Write},
    net::{TcpListener, TcpStream},
    thread,
};

use crate::{
    common::{HttpError, HttpMethod, HttpStatus},
    message::{
        create_http_response_message, parse_http_request, HttpHeaders, HttpRequest, HttpResponse,
    },
    router::{HttpRouter, HttpRouterFunc},
};

pub struct HttpServer {
    pub tcp_listener: TcpListener,
    pub router: HttpRouter,
}

impl HttpServer {
    pub fn bind(host: &str, port: u16, router: HttpRouter) -> Result<Self, std::io::Error> {
        match TcpListener::bind(format!("{host}:{port}")) {
            Ok(listener) => Ok(HttpServer {
                tcp_listener: listener,
                router: router,
            }),
            Err(e) => Err(e),
        }
    }
    pub fn serve(self: &Self) {
        let _ = for stream in self.tcp_listener.incoming() {
            let _ = match stream {
                Ok(s) => {
                    todo!()
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
    use crate::router::HttpRouterBuilder;

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
        let server = HttpServer::bind(HOST, PORT, HttpRouterBuilder::new().build());
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
}
