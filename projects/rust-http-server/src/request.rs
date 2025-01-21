use std::{collections::HashMap, fmt::Display, str::FromStr};

use crate::common::{HttpError, HttpProtocol};

#[derive(Debug, PartialEq)]
pub enum RequestMethod {
    DELETE,
    GET,
    PATCH,
    POST,
    PUT,
}
#[derive(Debug)]
pub struct HttpRequest<'a> {
    uri: String,
    protocol: HttpProtocol,
    method: RequestMethod,
    headers: Option<HashMap<&'a str, &'a str>>,
    body: Option<String>,
}

impl FromStr for RequestMethod {
    type Err = HttpError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(RequestMethod::GET),
            "POST" => Ok(RequestMethod::POST),
            "PUT" => Ok(RequestMethod::PUT),
            "DELETE" => Ok(RequestMethod::DELETE),
            "PATCH" => Ok(RequestMethod::PATCH),
            _ => Err(HttpError::new(format!(
                "cannot parse {} as request method",
                s
            ))),
        }
    }
}

impl Display for RequestMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl HttpRequest<'_> {
    pub fn parse(raw_request: &str) -> Result<HttpRequest, HttpError> {
        let lines = raw_request.lines().collect::<Vec<&str>>();
        // HTTP request should at least be one line.
        // first line is the METHOD URI HTTP_VERSION
        if lines.len() == 0 {
            return Err(HttpError::new("empty http request message"));
        }
        let method = match lines[0].split(' ').nth(0) {
            Some(s) => match RequestMethod::from_str(s) {
                Ok(method) => method,
                Err(e) => return Err(e),
            },
            None => return Err(HttpError::new("empty http request method")),
        };
        let uri: &str = match lines[0].split(' ').nth(1) {
            Some(m) => m,
            None => return Err(HttpError::new("empty http request uri")),
        };
        let http_version: HttpProtocol = match lines[0].split(' ').nth(2) {
            Some(s) => match HttpProtocol::from_str(s) {
                Ok(v) => v,
                Err(e) => return Err(e),
            },
            None => return Err(HttpError::new("empty http request protocol")),
        };
        // When there are more than on lines, we expect headers/body
        if lines.len() > 2 {
            let body_seperator_index = match lines.iter().position(|x| x.is_empty()) {
                Some(x) => x,
                None => return Err(HttpError::new("http body/header seperator not found")),
            };
            if body_seperator_index != lines.len() - 1 {
                match method {
                    RequestMethod::PATCH | RequestMethod::POST | RequestMethod::PUT => (),
                    s => {
                        return Err(HttpError::new(format!(
                            "method {s} does not support having http body",
                        )))
                    }
                }
            }
            let (header_lines, body_lines) = (
                &lines[1..body_seperator_index],
                &lines[body_seperator_index + 1..],
            );
            // Collect Headers
            let headers = header_lines
                .iter()
                .map(|x: &&str| -> Result<(&str, &str), HttpError> {
                    let vec = x.split(": ").collect::<Vec<&str>>();
                    match &vec[..] {
                        [first, second] => Ok((first, second)),
                        _ => Err(HttpError::new(format!("Could not parse http header {x}",))),
                    }
                })
                .collect::<Result<HashMap<&str, &str>, HttpError>>();
            if headers.is_err() {
                return Err(headers.err().unwrap());
            }
            //Body
            let body = body_lines.join("\n");
            Ok(HttpRequest {
                method: method,
                protocol: http_version,
                uri: uri.to_string(),
                headers: Some(headers.unwrap()),
                body: Some(body),
            })
        } else {
            Ok(HttpRequest {
                method: method,
                protocol: http_version,
                uri: uri.to_string(),
                headers: None,
                body: None,
            })
        }
    }
}

impl Display for HttpRequest<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Method: {:?}\nURI: {}\nHeaders: {:?}\nBody: {}",
            self.method,
            self.uri,
            self.headers.as_ref().unwrap_or(&HashMap::new()),
            self.body.as_ref().unwrap_or(&String::new())
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_method_parse_ok() {
        let correct_methods = ["GET", "POST", "PUT", "DELETE"];
        assert!(correct_methods
            .iter()
            .all(|x| RequestMethod::from_str(x).is_ok()));
    }

    #[test]
    fn test_http_method_parse_error() {
        let incorrect_methods = ["Get", "pOst", "puT", "dELETE"];
        assert!(incorrect_methods
            .iter()
            .all(|x| RequestMethod::from_str(x).is_err()));

        let not_recognized_methods = ["method1", "foo", "bar", "HTTP", "/item/uri/path"];
        assert!(not_recognized_methods
            .iter()
            .all(|x| RequestMethod::from_str(x).is_err()));
    }
    #[test]
    fn test_http_request_parse_ok() {
        let correct_raw_http_request = "POST /uri/path HTTP/1.1\n\
        Host: localhost\n\
        Content-Type: application/json\n\
        Content-Length: 50\n\
        \n\
        body";
        match HttpRequest::parse(correct_raw_http_request) {
            Ok(h) => {
                // Asserting first line objects
                assert_eq!(h.method, RequestMethod::POST);
                assert_eq!(h.uri, "/uri/path");
                assert_eq!(h.protocol, HttpProtocol::Http1_1);
                // Asserting headers
                let headers = h.headers.unwrap();
                assert_eq!(headers["Host"], "localhost");
                assert_eq!(headers["Content-Type"], "application/json");
                assert_eq!(headers["Content-Length"], "50");
                // Asserting body
                let body = h.body.unwrap();
                assert_eq!(body, "body");
            }
            Err(e) => panic!(
                "Correct http request could not be parsed! error: {}\nrequest body: {}",
                e, correct_raw_http_request
            ),
        };
    }

    // HEADERS Tests
    #[test]
    fn test_http_request_parse_empty_headers_correct() {
        let correct_raw_http_request = "POST /uri/path HTTP/1.1\n\
        \n\
        body";
        match HttpRequest::parse(correct_raw_http_request) {
            Ok(h) => {
                // Asserting first line objects
                assert_eq!(h.method, RequestMethod::POST);
                assert_eq!(h.uri, "/uri/path");
                assert_eq!(h.protocol, HttpProtocol::Http1_1);
                // Asserting headers
                assert_eq!(h.headers.unwrap().len(), 0);
                // Asserting body
                assert_eq!(h.body.unwrap(), "body");
            }
            Err(e) => panic!(
                "Correct http request could not be parsed! error: {}\nrequest body: {}",
                e, correct_raw_http_request
            ),
        };
    }
    #[test]
    fn test_http_request_parse_bad_headers_error() {
        let bad_headers_http_request = "POST /uri/path HTTP/1.1\n\
        Host localhost\n\
        \n";
        match HttpRequest::parse(bad_headers_http_request) {
            Ok(h) => {
                panic!("Bad headers were given but the test case was ok!: {}", h)
            }
            Err(_) => (),
        };
    }

    // Body Test
    #[test]
    fn test_http_request_parse_empty_body_correct() {
        let correct_raw_http_request = "POST /uri/path HTTP/1.1\n\
        Host: localhost\n\
        Content-Type: application/json\n\
        Content-Length: 50\n\
        \n";
        match HttpRequest::parse(correct_raw_http_request) {
            Ok(h) => {
                // Asserting first line objects
                assert_eq!(h.method, RequestMethod::POST);
                assert_eq!(h.uri, "/uri/path");
                assert_eq!(h.protocol, HttpProtocol::Http1_1);
                // Asserting headers
                let headers = h.headers.unwrap();
                assert_eq!(headers["Host"], "localhost");
                assert_eq!(headers["Content-Type"], "application/json");
                assert_eq!(headers["Content-Length"], "50");
                // Asserting body
                assert_eq!(h.body.unwrap(), "");
            }
            Err(e) => panic!(
                "Correct http request could not be parsed! error: {}\nrequest body: {}",
                e, correct_raw_http_request
            ),
        };
    }
    #[test]
    fn test_http_request_body_multiline_correct() {
        let correct_raw_http_request = "POST /uri/path HTTP/1.1\n\
        Host: localhost\n\
        Content-Type: application/json\n\
        Content-Length: 50\n\
        \n\
        body-line-1\n\
        body-line-2";
        match HttpRequest::parse(correct_raw_http_request) {
            Ok(h) => {
                // Asserting first line objects
                assert_eq!(h.method, RequestMethod::POST);
                assert_eq!(h.uri, "/uri/path");
                assert_eq!(h.protocol, HttpProtocol::Http1_1);
                // Asserting headers
                let headers = h.headers.unwrap();
                assert_eq!(headers["Host"], "localhost");
                assert_eq!(headers["Content-Type"], "application/json");
                assert_eq!(headers["Content-Length"], "50");
                // Asserting body
                assert_eq!(h.body.unwrap(), "body-line-1\nbody-line-2");
            }
            Err(e) => panic!(
                "Correct http request could not be parsed! error: {}\nrequest body: {}",
                e, correct_raw_http_request
            ),
        };
    }
    #[test]
    fn test_http_request_body_with_incorrect_method() {
        let partial_raw_http_request = "/uri/path HTTP/1.1\n\
        Host: localhost\n\
        Content-Type: application/json\n\
        Content-Length: 50\n\
        \n\
        body";
        for item in [RequestMethod::GET, RequestMethod::DELETE] {
            match HttpRequest::parse(
                (format!("{:?} ", item).to_owned() + partial_raw_http_request).as_str(),
            ) {
                Ok(_) => {
                    panic!("Request Method: {:?} should not accept any body!", item)
                }
                Err(_) => (),
            };
        }
    }
}
