use core::time;
use std::thread;

mod common;
mod request;
mod server;

fn main() {
    let (host, port): (&str, u16) = ("127.0.0.1", 18000);
    let http_server =
        server::HttpServer::bind(host, port).expect("Cannot bind to the address and port given!");
    loop {
        for stream in http_server.tcp_listener.incoming() {
            let _ = match stream {
                Ok(s) => http_server.handle(s),
                Err(e) => Err(e),
            };
            thread::sleep(time::Duration::from_millis(10));
        }
    }
}
