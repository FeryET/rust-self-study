use std::{
    collections::BTreeMap,
    fmt::Display,
    io::{BufRead, BufReader, Read},
    str::FromStr,
};

use crate::common::{HttpError, HttpMethod, HttpProtocol, HttpStatus};

pub type HttpHeaders = BTreeMap<String, String>;
pub type HttpBody = Option<String>;

#[derive(Clone)]
pub struct HttpRequestMetaData {
    pub protocol: HttpProtocol,
    pub uri: String,
    pub method: HttpMethod,
    pub headers: HttpHeaders,
}

impl Display for HttpRequestMetaData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let headers_str = self
            .headers
            .iter()
            .map(|item| format!("{}: {}\r\n", item.0, item.1))
            .collect::<String>();
        write!(
            f,
            "{} {} {}\r\n{}",
            self.method, self.uri, self.protocol, headers_str
        )
    }
}

#[derive(Clone)]
pub struct HttpRequest {
    pub metadata: HttpRequestMetaData,
    pub body: HttpBody,
}

impl Display for HttpRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let body_str = match self.body.clone() {
            Some(b) => b,
            None => "".to_string(),
        };
        write!(f, "{}\r\n{}", self.metadata, body_str)
    }
}

impl HttpRequestMetaData {
    fn parse_info_line(first_line: &str) -> Result<(HttpMethod, HttpProtocol, String), HttpError> {
        let first_line_split = first_line.split(' ').collect::<Vec<&str>>();
        if first_line_split.len() != 3 {
            return Err(HttpError::new(HttpStatus::BadRequest, ""));
        }
        let (method, protocol) = match (
            HttpMethod::from_str(first_line_split[0]),
            HttpProtocol::from_str(first_line_split[2]),
        ) {
            (Ok(m), Ok(p)) => (m, p),
            _ => return Err(HttpError::new(HttpStatus::BadRequest, "")),
        };
        let uri = first_line_split[1].to_string();
        Ok((method, protocol, uri))
    }
    fn parse_headers(lines: &[String]) -> Result<HttpHeaders, HttpError> {
        lines
            .iter()
            .map(|x: &String| -> Result<(String, String), HttpError> {
                let vec: Vec<&str> = x.split(": ").collect();
                match vec.as_slice() {
                    [first, second] => Ok((first.to_string(), second.to_string())),
                    _ => Err(HttpError::new(HttpStatus::BadRequest, "")),
                }
            })
            .collect::<Result<HttpHeaders, HttpError>>()
    }

    /// Parses a raw http request from a readable buffer and returns the metadata.
    ///
    /// # Errors    HttpError
    ///
    /// This function will return an error if there are any issues when reading the information line, and
    /// the header lines.
    pub fn parse(raw: &str) -> Result<HttpRequestMetaData, HttpError> {
        let metadata_lines = raw.lines().map(String::from).collect::<Vec<String>>();
        if metadata_lines.len() == 0 {
            return Err(HttpError::new(HttpStatus::BadRequest, ""));
        }
        let (method, protocol, uri) = match HttpRequestMetaData::parse_info_line(&metadata_lines[0])
        {
            Ok((m, p, uri)) => (m, p, uri),
            Err(e) => return Err(e),
        };

        let request_headers = if metadata_lines.len() > 2 {
            match HttpRequestMetaData::parse_headers(&metadata_lines[1..]) {
                Ok(h) => h,
                Err(e) => return Err(e),
            }
        } else {
            HttpHeaders::new()
        };
        Ok(HttpRequestMetaData {
            method: method,
            uri: uri,
            protocol: protocol,
            headers: request_headers,
        })
    }
    pub fn content_length(self: &Self) -> Result<usize, HttpError> {
        let zero_content_length = "0".to_string();
        let x = self
            .headers
            .get("Content-Length")
            .unwrap_or(&zero_content_length);
        match x.parse::<u32>() {
            Ok(s) => Ok(s as usize),
            Err(_) => Err(HttpError::new(
                HttpStatus::BadRequest,
                "bad 'Content-Length'",
            )),
        }
    }
}

impl HttpRequest {
    pub fn parse_request_body<R: Read>(
        buffer: &mut BufReader<R>,
        content_length: usize,
    ) -> HttpBody {
        if content_length == 0 {
            None
        } else {
            Some(
                buffer
                    .bytes()
                    .take(content_length)
                    .map(|x| x.ok().unwrap() as char)
                    .collect::<String>(),
            )
        }
    }
}

pub fn parse_http_request<R: Read>(buffer: &mut BufReader<R>) -> Result<HttpRequest, HttpError> {
    let metadata_str = buffer
        .lines()
        .map(|x| x.unwrap())
        .take_while(|x| !x.is_empty())
        .collect::<Vec<String>>()
        .join("\n");
    let metadata = match HttpRequestMetaData::parse(metadata_str.as_str()) {
        Ok(m) => m,
        Err(e) => return Err(e),
    };
    let body = match metadata.content_length() {
        Ok(c) => HttpRequest::parse_request_body(buffer, c),
        Err(e) => return Err(e),
    };
    Ok(HttpRequest {
        metadata: metadata,
        body: body,
    })
}
// =========================================================
// ==================== Http Response ======================
// =========================================================

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
pub fn create_http_response_message(
    protocol: HttpProtocol,
    status: HttpStatus,
    headers: HttpHeaders,
    body: HttpBody,
) -> HttpResponse {
    HttpResponse {
        metadata: HttpResponseMetaData {
            protocol: protocol,
            status: status,
            headers: headers,
        },
        body: body,
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    fn collect_into_lines(raw: &str) -> Vec<String> {
        raw.lines().map(|s| s.to_string()).collect::<Vec<String>>()
    }

    #[test]
    fn test_parse_request_info() {
        assert!(HttpRequestMetaData::parse_info_line("POST /uri/path HTTP/1.1").is_ok());
        assert!(HttpRequestMetaData::parse_info_line("POST/uri/path HTTP/1.1").is_err());
        // 4 elements in the first line should throw an error
        assert!(
            HttpRequestMetaData::parse_info_line("POST /uri/path some_other_item HTTP/1.1")
                .is_err()
        );
        // without URI should throw error
        assert!(HttpRequestMetaData::parse_info_line("POST HTTP/1.1").is_err());
    }

    #[test]
    fn test_parse_headers_ok_for_correct_headers() {
        let correct_headers = "Host: localhost\n\
                                        Content-Type: application/json\n\
                                        Content-Length: 50";
        let headers = HttpRequestMetaData::parse_headers(&collect_into_lines(&correct_headers));
        assert!(headers.is_ok());
    }
    #[test]
    fn test_parse_headers_correct_key_value_for_correct_headers() {
        let correct_headers = "Host: localhost\n\
                                        Content-Type: application/json\n\
                                        Content-Length: 50";
        let headers =
            HttpRequestMetaData::parse_headers(&collect_into_lines(&correct_headers)).unwrap();
        assert_eq!(headers.len(), 3);
        assert_eq!(headers["Host"], "localhost");
        assert_eq!(headers["Content-Type"], "application/json");
        assert_eq!(headers["Content-Length"], "50");
    }
    #[test]
    fn test_parse_headers_returns_error_for_incorrect_headers() {
        let incorrect_headers = "Host localhost\n\
                                           Content-Type: content-type\n
                                           Header:    value";
        assert!(
            HttpRequestMetaData::parse_headers(&collect_into_lines(&incorrect_headers)).is_err()
        )
    }

    #[test]
    fn test_parse_body_request_body_none_when_content_length_zero() {
        let correct_body = "This is a body".to_string();
        assert!(
            HttpRequest::parse_request_body(&mut BufReader::new(correct_body.as_bytes()), 0)
                .is_none()
        );
    }
    #[test]
    fn test_parse_request_body_correct_when_content_length_is_non_zero() {
        let correct_body = "This is a multiline\nBody format";
        let parsed_body = HttpRequest::parse_request_body(
            &mut BufReader::new(correct_body.as_bytes()),
            correct_body.len(),
        );
        assert!(parsed_body.is_some());
        assert_eq!(parsed_body.unwrap(), correct_body)
    }

    #[test]
    fn test_parse_request_correct_raw_message_is_ok() {
        let raw_request = "\
        POST /uri/path HTTP/1.1\n\
        Host: localhost\n\
        Content-Type: application/json\n\
        Content-Length: 4\n\
        \n\
        body";
        assert!(parse_http_request(&mut BufReader::new(raw_request.as_bytes())).is_ok());
    }
    #[test]
    fn test_parse_request_correct_raw_message_correct_headers() {
        let raw_request = "POST /uri/path HTTP/1.1\n\
        Host: localhost\n\
        Content-Type: application/json\n\
        Content-Length: 23\n\
        \n\
        body-line-1\n\
        body-line-2";
        let h = parse_http_request(&mut BufReader::new(raw_request.as_bytes()))
            .unwrap()
            .metadata
            .headers;
        assert_eq!(h.len(), 3);
        assert_eq!(h["Content-Type"], "application/json");
        assert_eq!(h["Content-Length"], "23");
        assert_eq!(h["Host"], "localhost");
    }
    #[test]
    fn test_parse_request_correct_raw_message_correct_body() {
        let raw_request = "POST /uri/path HTTP/1.1\n\
        Host: localhost\n\
        Content-Type: application/json\n\
        Content-Length: 23\n\
        \n\
        body-line-1\n\
        body-line-2";
        let b = parse_http_request(&mut BufReader::new(raw_request.as_bytes()))
            .unwrap()
            .body;
        assert_eq!(b.unwrap(), "body-line-1\nbody-line-2")
    }
    #[test]
    fn test_parse_request_incorrect_raw_message_info_line_incorrect_should_return_err() {
        let raw_request = "Host: localhost\n\
        Content-Type: application/json\n\
        Content-Length: 5\n\
        \n\
        body";
        // if the first line is actually headers throw error
        assert!(parse_http_request(&mut BufReader::new(raw_request.as_bytes())).is_err());
        // If the first line is empty throw error
        assert!(parse_http_request(&mut BufReader::new(
            ("\n".to_owned() + raw_request).as_bytes()
        ))
        .is_err());
    }
    #[test]
    fn test_parse_request_correct_parse_body_starting_with_line_break() {
        let raw_request = "POST /uri/path HTTP/1.1\n\
        Host: localhost\n\
        Content-Type: application/json\n\
        Content-Length: 5\n\
        \n\
        \n\
        body";
        let b = parse_http_request(&mut BufReader::new(raw_request.as_bytes()))
            .unwrap()
            .body;
        assert_eq!(b.unwrap(), "\nbody")
    }

    #[test]
    fn test_parse_request_body_matches_content_length() {
        let body = "body";
        let raw_request = format!(
            "POST /uri/path HTTP/1.1\n\
        Host: localhost\n\
        Content-Type: application/json\n\
        Content-Length: {}\n\
        \n\
        {}",
            body.len(),
            body
        );
        let r = parse_http_request(&mut BufReader::new(raw_request.as_bytes())).unwrap();
        assert_eq!(r.body.unwrap(), body);
        assert_eq!(
            r.metadata.headers["Content-Length"]
                .parse::<usize>()
                .unwrap(),
            body.len()
        );
    }
    #[test]
    fn test_create_response_display_string() {
        let mut h: HttpHeaders = HttpHeaders::new();
        h.insert("Host".to_string(), "localhost".to_string());
        h.insert("Content-Length".to_string(), "4".to_string());
        let body = "body";
        let r = create_http_response_message(
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
        let m = create_http_response_message(
            HttpProtocol::Http1_1,
            HttpStatus::Ok,
            h,
            Some("foo".to_string()),
        );
        assert_eq!(raw_reponse, format!("{m}"));
    }
}
