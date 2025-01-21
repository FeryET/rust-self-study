use std::{
    io::{Error, Read},
    net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream},
    str::FromStr,
};

use crate::request::HttpRequest;

pub struct HttpServer {
    pub tcp_listener: TcpListener,
}

impl HttpServer {
    pub fn bind(host: &str, port: u16) -> Result<Self, std::io::Error> {
        let socket = SocketAddr::new(
            IpAddr::V4(Ipv4Addr::from_str(host).expect("Cannot parse ip address")),
            port,
        );
        match TcpListener::bind(socket) {
            Ok(listener) => Ok(HttpServer {
                tcp_listener: listener,
            }),
            Err(e) => Err(e),
        }
    }
    pub fn handle(self: &Self, stream: TcpStream) -> Result<bool, std::io::Error> {
        let raw_request = stream
            .bytes()
            .map(|x| match x {
                Ok(u) => Ok(u as char),
                Err(e) => Err(e),
            })
            .collect::<Result<String, Error>>();
        match raw_request {
            Ok(r) => {
                match HttpRequest::parse(&r) {
                    Ok(h) => {
                        println!("Http Request Message: {:?}", h);
                    }
                    Err(e) => {
                        println!("{}", e);
                    }
                };
                Ok(true)
            }
            Err(e) => Err(e),
        }
    }
}
