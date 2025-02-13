use common::{HttpError, HttpHeaders, HttpMethod, HttpServerContext};
use log::{debug, error, info};
use request::HttpRequest;
use response::HttpResponse;
use router::HttpRouterBuilder;
use server::HttpServer;
use std::{
    fs::File,
    io::{BufReader, Read},
    net::TcpListener,
};

mod common;
mod request;
mod response;
mod router;
mod server;

fn handle_static_content(
    r: &HttpRequest,
    c: &HttpServerContext,
) -> Result<HttpResponse, HttpError> {
    debug!("handle_static_content: called!");
    if r.metadata.method != HttpMethod::GET {
        error!("handle_static_content: method not allowed");
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
            Ok(HttpResponse::new(
                r.metadata.protocol,
                common::HttpStatus::Ok,
                HttpHeaders::new(),
                Some(body),
            ))
        }
        Err(e) => {
            error!("Static Content Handler: error accessing resource {e}");
            Err(HttpError::new(common::HttpStatus::NotFound, ""))
        }
    }
}

fn handle_pics(r: &HttpRequest, c: &HttpServerContext) -> Result<HttpResponse, HttpError> {
    debug!("handle_pics: called");
    if r.metadata.method != HttpMethod::GET {
        error!("handle_pics: method not allowed");
        return Err(HttpError::new(common::HttpStatus::MethodNotAllowed, ""));
    }
    let pic_path = c.get("pic").unwrap();
    match File::open(format!("pics/{pic_path}")) {
        Ok(f) => {
            let body = BufReader::new(f)
                .bytes()
                .map(|x| x.unwrap() as char)
                .map(String::from)
                .collect::<String>();
            Ok(HttpResponse::new(
                r.metadata.protocol,
                common::HttpStatus::Ok,
                HttpHeaders::new(),
                Some(body),
            ))
        }
        Err(e) => {
            error!("handle_pics: error accessing resource {e}");
            Err(HttpError::new(common::HttpStatus::NotFound, ""))
        }
    }
}

fn start_server() {
    let server = HttpServer::new(
        HttpRouterBuilder::new()
            .add_route(HttpMethod::GET, "/*", handle_static_content)
            .add_route(HttpMethod::GET, "/pics/:pic", handle_pics)
            .build(),
    );
    let listener =
        TcpListener::bind("127.0.0.1:18000").expect("binding address was in use, could not bind.");
    server.serve(&listener);
}

fn main() {
    env_logger::init();
    start_server();
}
