use common::{HttpError, HttpMethod};
use message::{create_http_response_message, HttpHeaders, HttpRequest, HttpResponse};
use std::{
    fs::File,
    io::{BufReader, Read},
};

mod common;
mod message;
mod router;
mod server;

fn handle_root(r: &HttpRequest) -> Result<HttpResponse, HttpError> {
    println!("root handler called!");
    if r.metadata.method != HttpMethod::GET {
        return Err(HttpError::new(common::HttpStatus::MethodNotAllowed, ""));
    }
    let uri = match r.metadata.uri.strip_prefix("/") {
        Some(h) => h,
        None => r.metadata.uri.as_str(),
    };
    match File::open(uri) {
        Ok(f) => {
            let body = BufReader::new(f)
                .bytes()
                .map(|x| x.unwrap() as char)
                .map(String::from)
                .collect::<String>();
            Ok(create_http_response_message(
                r.metadata.protocol,
                common::HttpStatus::Ok,
                HttpHeaders::new(),
                Some(body),
            ))
        }
        Err(e) => {
            println!("Error accessing resource: {e}");
            Err(HttpError::new(common::HttpStatus::NotFound, ""))
        }
    }
}

fn handle_pics(r: &HttpRequest) -> Result<HttpResponse, HttpError> {
    println!("pic handler called!");
    if r.metadata.method != HttpMethod::GET {
        return Err(HttpError::new(common::HttpStatus::MethodNotAllowed, ""));
    }
    let uri = match r.metadata.uri.strip_prefix("/pics/") {
        Some(h) => h,
        None => r.metadata.uri.as_str(),
    };
    match File::open(format!("pics/{uri}")) {
        Ok(f) => {
            let body = BufReader::new(f)
                .bytes()
                .map(|x| x.unwrap() as char)
                .map(String::from)
                .collect::<String>();
            Ok(create_http_response_message(
                r.metadata.protocol,
                common::HttpStatus::Ok,
                HttpHeaders::new(),
                Some(body),
            ))
        }
        Err(e) => {
            println!("Error accessing resource: {e}");
            Err(HttpError::new(common::HttpStatus::NotFound, ""))
        }
    }
}

fn start_server() {
    let (host, port): (&str, u16) = ("127.0.0.1", 18000);
    let mut http_server: server::HttpServer =
        server::HttpServer::bind(host, port).expect("Cannot bind to the address and port given!");
    http_server.handlers.insert("/".to_string(), handle_root);
    http_server
        .handlers
        .insert("/pics".to_string(), handle_pics);
    http_server.serve();
}

fn main() {
    start_server();
}
