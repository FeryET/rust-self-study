use std::{
    io::Read,
    net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream},
    str::FromStr,
};

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
        let request: String = stream
            .bytes()
            .map(|x| x.expect("cannot parse byte") as char)
            .collect::<String>();
        print!("{}", request);
        Ok(true)
    }
}
