use std::{fmt::Display, str::FromStr};

#[derive(Debug, PartialEq)]

pub struct HttpError {
    message: String,
}
#[derive(Debug, PartialEq)]
pub enum HttpProtocol {
    Http1,
    Http1_1,
    Http2,
}

impl HttpError {
    pub fn new<T: Into<String>>(message: T) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl Display for HttpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HttpError: {}", self.message)
    }
}

impl FromStr for HttpProtocol {
    type Err = HttpError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "HTTP/1" => Ok(HttpProtocol::Http1),
            "HTTP/1.1" => Ok(HttpProtocol::Http1_1),
            "HTTP/2" => Ok(HttpProtocol::Http2),
            _ => Err(HttpError::new(format!("unsupported http version: {}", s))),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_version_from_str() {
        assert_eq!("HTTP/1".parse::<HttpProtocol>().unwrap(), HttpProtocol::Http1);
        assert_eq!(
            "HTTP/1.1".parse::<HttpProtocol>().unwrap(),
            HttpProtocol::Http1_1
        );
        assert_eq!("HTTP/2".parse::<HttpProtocol>().unwrap(), HttpProtocol::Http2);
        assert!("HTTP/3".parse::<HttpProtocol>().is_err());
        assert!("HTTP/1.0".parse::<HttpProtocol>().is_err());
    }

    #[test]
    fn test_http_error_display() {
        let error = HttpError::new("dummy error");
        assert_eq!(
            format!("{}", error),
            "HttpError: dummy error"
        );
    }
}
