use std::collections::HashMap;

use crate::{
    common::{HttpError, HttpMethod, HttpServerContext, HttpStatus},
    request::HttpRequest,
    response::HttpResponse,
};
use log::{debug, info};
use path_tree::PathTree;

pub type HttpRouterFunc = fn(&HttpRequest, &HttpServerContext) -> Result<HttpResponse, HttpError>;

type HttpMethodRouter = PathTree<HttpRouterFunc>;
type HttpRouterMap = HashMap<HttpMethod, HttpMethodRouter>;
#[derive(Clone)]
pub struct HttpRouter {
    router_map: HttpRouterMap,
}

impl HttpRouter {
    pub fn parse_request_route(
        self: &Self,
        req: &HttpRequest,
    ) -> Option<(&HttpRouterFunc, HttpServerContext)> {
        debug!(
            "HttpRouter: parsing request route protocol: {} # method: {} # uri: {}",
            req.metadata.protocol, req.metadata.method, req.metadata.uri
        );
        let item = match self.router_map.get(&req.metadata.method) {
            Some(router) => router.find(&req.metadata.uri),
            None => None,
        };
        match item {
            Some((func, path)) => Some((
                func,
                path.params_iter()
                    .map(|(x, y)| (x.to_string(), y.to_string()))
                    .collect(),
            )),
            None => None,
        }
    }
    pub fn route(self: &Self, req: &HttpRequest) -> Result<HttpResponse, HttpError> {
        let (handler, params) = match self.parse_request_route(req) {
            Some(tuple) => tuple,
            None => return Err(HttpError::new(HttpStatus::MethodNotAllowed, "")),
        };
        (handler)(req, &params)
    }
}

pub struct HttpRouterBuilder {
    routers: HashMap<(HttpMethod, String), HttpRouterFunc>,
}

impl HttpRouterBuilder {
    pub fn new() -> Self {
        HttpRouterBuilder {
            routers: HashMap::new(),
        }
    }
    pub fn add_route(
        self: &mut Self,
        method: HttpMethod,
        path: &str,
        func: HttpRouterFunc,
    ) -> &mut Self {
        if self.routers.contains_key(&(method, path.to_string())) {
            panic!("dupliate endpoint decleration method: {method} path:{path})")
        }
        self.routers.insert((method, path.to_string()), func);
        self
    }
    pub fn build(self: &Self) -> HttpRouter {
        let mut router_map: HttpRouterMap = HttpRouterMap::new();
        self.routers.iter().for_each(|((m, p), f)| -> () {
            if !router_map.contains_key(m) {
                router_map.insert(*m, PathTree::new());
            };
            let _ = router_map.get_mut(m).unwrap().insert(p, *f);
        });
        return HttpRouter { router_map };
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::common::{HttpHeaders, HttpProtocol};
    use crate::request::HttpRequestMetaData;
    use crate::response::HttpResponseMetaData;

    #[test]
    fn test_http_router_builder_new_pass() {
        let builder = HttpRouterBuilder::new();
        assert_eq!(builder.routers.capacity(), 0);
    }

    fn emit_error(_: &HttpRequest, _: &HttpServerContext) -> Result<HttpResponse, HttpError> {
        Err(HttpError::new(HttpStatus::BadRequest, "undefined"))
    }

    fn emit_success_response(
        _: &HttpRequest,
        _: &HttpServerContext,
    ) -> Result<HttpResponse, HttpError> {
        Ok(HttpResponse {
            body: None,
            metadata: HttpResponseMetaData {
                status: HttpStatus::Ok,
                protocol: HttpProtocol::Http1_1,
                headers: HttpHeaders::new(),
            },
        })
    }

    fn test_request() -> HttpRequest {
        HttpRequest {
            metadata: HttpRequestMetaData::parse("GET / HTTP/1.1").unwrap(),
            body: None,
        }
    }

    #[test]
    fn test_http_router_builder_add_router_pass() {
        let mut builder = HttpRouterBuilder::new();
        builder.add_route(HttpMethod::GET, "/", emit_error);
        builder
            .routers
            .get(&(HttpMethod::GET, "/".to_string()))
            .unwrap();
    }
    #[test]
    #[should_panic]
    fn test_http_router_builder_add_router_twice_fail() {
        let mut builder = HttpRouterBuilder::new();
        builder.add_route(HttpMethod::GET, "/", emit_error);
        builder.add_route(HttpMethod::GET, "/", emit_error);
    }
    #[test]
    fn test_http_router_builder_add_router_twice_with_different_methods_pass() {
        let mut builder = HttpRouterBuilder::new();
        builder.add_route(HttpMethod::GET, "/", emit_error);
        builder.add_route(HttpMethod::POST, "/", emit_error);
        builder.add_route(HttpMethod::DELETE, "/", emit_error);
        builder.add_route(HttpMethod::PUT, "/", emit_error);
        builder.add_route(HttpMethod::PATCH, "/", emit_error);
    }

    #[test]
    fn test_http_router_builder_build_successful() {
        let mut builder = HttpRouterBuilder::new();
        builder.add_route(HttpMethod::GET, "/", emit_error);
        builder.add_route(HttpMethod::POST, "/", emit_error);
        builder.add_route(HttpMethod::DELETE, "/", emit_error);
        builder.add_route(HttpMethod::PUT, "/", emit_error);
        builder.add_route(HttpMethod::PATCH, "/", emit_error);

        let _ = builder.build();
    }

    #[test]
    fn test_http_router_parse_request_route_pass() {
        let router = HttpRouterBuilder::new()
            .add_route(HttpMethod::GET, "/", emit_success_response)
            .build();
        let r = test_request();
        let (handler, params) = router.parse_request_route(&r).unwrap();
        assert_eq!(params.capacity(), 0);
        assert!(handler(&r, &params).is_ok());
        assert_eq!(
            handler(&r, &params).unwrap().metadata.status,
            HttpStatus::Ok
        );
    }
}
