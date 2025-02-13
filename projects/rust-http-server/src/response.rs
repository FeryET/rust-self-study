use std::fmt::Display;

use crate::common::{HttpBody, HttpError, HttpHeaders, HttpProtocol, HttpStatus};

#[derive(Clone)]
pub struct HttpResponseMetaData {
    pub protocol: HttpProtocol,
    pub status: HttpStatus,
    pub headers: HttpHeaders,
}

impl Display for HttpResponseMetaData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let headers_str = self
            .headers
            .iter()
            .map(|item| format!("{}: {}\r\n", item.0, item.1))
            .collect::<String>();
        write!(f, "{} {}\r\n{}", self.protocol, self.status, headers_str)
    }
}
pub struct HttpResponse {
    pub metadata: HttpResponseMetaData,
    pub body: HttpBody,
}

impl Display for HttpResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let body_str = match self.body.clone() {
            Some(b) => b,
            None => "".to_string(),
        };
        write!(f, "{}\r\n{}", self.metadata, body_str)
    }
}

impl HttpResponse {
    pub fn new(
        protocol: HttpProtocol,
        status: HttpStatus,
        headers: HttpHeaders,
        body: HttpBody,
    ) -> Self {
        HttpResponse {
            metadata: HttpResponseMetaData {
                protocol: protocol,
                status: status,
                headers: headers,
            },
            body: body,
        }
    }

    pub fn from_err(err: HttpError, protocol: Option<HttpProtocol>) -> HttpResponse {
        HttpResponse {
            metadata: HttpResponseMetaData {
                protocol: protocol.unwrap_or(HttpProtocol::Http1_1),
                status: err.status,
                headers: HttpHeaders::new(),
            },
            body: None,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    fn collect_into_lines(raw: &str) -> Vec<String> {
        raw.lines().map(|s| s.to_string()).collect::<Vec<String>>()
    }

    #[test]
    fn test_create_response_display_string() {
        let mut h: HttpHeaders = HttpHeaders::new();
        h.insert("Host".to_string(), "localhost".to_string());
        h.insert("Content-Length".to_string(), "4".to_string());
        let body = "body";
        let r = HttpResponse::new(
            HttpProtocol::Http1_1,
            HttpStatus::Accepted,
            h,
            Some(body.to_string()),
        );
        let expected = "HTTP/1.1 202 Accepted\r\n\
        Content-Length: 4\r\n\
        Host: localhost\r\n\
        \r\n\
        body";
        assert_eq!(expected, format!("{r}"));
    }
    // =========================================================
    // ================= Http Response Test ====================
    // =========================================================
    #[test]
    fn test_response_display() {
        let raw_reponse = "\
        HTTP/1.1 200 OK\r\n\
        Content-Length: 3\r\n\
        Host: localhost\r\n\
        \r\n\
        foo";
        let mut h = HttpHeaders::new();
        h.insert("Host".to_string(), "localhost".to_string());
        h.insert("Content-Length".to_string(), "3".to_string());
        let m = HttpResponse::new(
            HttpProtocol::Http1_1,
            HttpStatus::Ok,
            h,
            Some("foo".to_string()),
        );
        assert_eq!(raw_reponse, format!("{m}"));
    }
}
