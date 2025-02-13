use std::{collections::{BTreeMap, HashMap}, fmt::Display, str::FromStr};

// =========================================================
// ====================== HttpError ========================
// =========================================================
#[derive(Debug, PartialEq)]
pub struct HttpError {
    pub status: HttpStatus,
    message: String,
}

impl HttpError {
    pub fn new<T: Into<String>>(status: HttpStatus, message: T) -> Self {
        Self {
            status: status,
            message: message.into(),
        }
    }
}

impl Clone for HttpError {
    fn clone(&self) -> Self {
        Self {
            status: self.status,
            message: self.message.clone(),
        }
    }
}

impl Display for HttpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HttpError: {}", self.message)
    }
}

// =========================================================
// ==================== HttpProtocol =======================
// =========================================================
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum HttpProtocol {
    Http1,
    Http1_1,
}

impl Display for HttpProtocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                HttpProtocol::Http1 => "HTTP/1",
                HttpProtocol::Http1_1 => "HTTP/1.1",
            }
        )
    }
}

impl FromStr for HttpProtocol {
    type Err = HttpError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "HTTP/1" => Ok(HttpProtocol::Http1),
            "HTTP/1.1" => Ok(HttpProtocol::Http1_1),
            _ => Err(HttpError::new(
                HttpStatus::BadRequest,
                format!("unsupported http version: {}", s),
            )),
        }
    }
}

// =========================================================
// ================= HttpMethod Section ====================
// =========================================================
#[derive(Eq, PartialOrd, PartialEq, Copy, Clone, Debug, Hash)]
pub enum HttpMethod {
    DELETE,
    GET,
    PATCH,
    POST,
    PUT,
}

impl FromStr for HttpMethod {
    type Err = HttpError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(HttpMethod::GET),
            "POST" => Ok(HttpMethod::POST),
            "PUT" => Ok(HttpMethod::PUT),
            "DELETE" => Ok(HttpMethod::DELETE),
            "PATCH" => Ok(HttpMethod::PATCH),
            _ => Err(HttpError::new(
                HttpStatus::BadRequest,
                format!("cannot parse {} as request method", s),
            )),
        }
    }
}

impl Display for HttpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            HttpMethod::POST => "POST",
            HttpMethod::PATCH => "PATCH",
            HttpMethod::PUT => "PUT",
            HttpMethod::GET => "GET",
            HttpMethod::DELETE => "DELETE",
        };
        write!(f, "{}", str)
    }
}

// =========================================================
// ================= HttpStatus Section ====================
// =========================================================

#[derive(Copy, Clone)]
pub enum HttpStatusType {
    Success,
    Redirect,
    ClientError,
    ServerError,
    Unknown,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum HttpStatus {
    // 2xx Range
    Ok = 200,
    Created = 201,
    Accepted = 202,
    NonAuthoritativeInformation = 203,
    NoContent = 204,
    ResetContent = 205,
    PartialContent = 206,
    // 3xx Range
    MultipleChoices = 300,
    MovedPermanently = 301,
    Found = 302,
    SeeOther = 303,
    NotModified = 304,
    UseProxy = 305,
    TemporaryRedirect = 307,
    PermanentRedirect = 308,
    // 4xx Range
    BadRequest = 400,
    Unauthorized = 401,
    PaymentRequired = 402,
    Forbidden = 403,
    NotFound = 404,
    MethodNotAllowed = 405,
    NotAcceptable = 406,
    ProxyAuthenticationRequired = 407,
    RequestTimeout = 408,
    Conflict = 409,
    Gone = 410,
    LengthRequired = 411,
    PreconditionFailed = 412,
    PayloadTooLarge = 413,
    URITooLong = 414,
    UnsupportedMediaType = 415,
    RangeNotSatisfiable = 416,
    ExpectationFailed = 417,
    ImATeapot = 418,
    MisdirectedRequest = 421,
    UnprocessableEntity = 422,
    Locked = 423,
    FailedDependency = 424,
    TooEarly = 425,
    UpgradeRequired = 426,
    PreconditionRequired = 428,
    TooManyRequests = 429,
    RequestHeaderFieldsTooLarge = 431,
    UnavailableForLegalReasons = 451,
    // 5xx Range
    InternalServerError = 500,
    NotImplemented = 501,
    BadGateway = 502,
    ServiceUnavailable = 503,
    GatewayTimeout = 504,
    HTTPVersionNotSupported = 505,
    VariantAlsoNegotiates = 506,
    InsufficientStorage = 507,
    LoopDetected = 508,
    NotExtended = 510,
    NetworkAuthenticationRequired = 511,
}

impl Display for HttpStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", *self as usize, self.message())
    }
}

impl HttpStatus {
    pub fn from_code(code: u16) -> HttpStatus {
        match code {
            200 => HttpStatus::Ok,
            201 => HttpStatus::Created,
            202 => HttpStatus::Accepted,
            203 => HttpStatus::NonAuthoritativeInformation,
            204 => HttpStatus::NoContent,
            205 => HttpStatus::ResetContent,
            206 => HttpStatus::PartialContent,
            300 => HttpStatus::MultipleChoices,
            301 => HttpStatus::MovedPermanently,
            302 => HttpStatus::Found,
            303 => HttpStatus::SeeOther,
            304 => HttpStatus::NotModified,
            305 => HttpStatus::UseProxy,
            307 => HttpStatus::TemporaryRedirect,
            308 => HttpStatus::PermanentRedirect,
            400 => HttpStatus::BadRequest,
            401 => HttpStatus::Unauthorized,
            402 => HttpStatus::PaymentRequired,
            403 => HttpStatus::Forbidden,
            404 => HttpStatus::NotFound,
            405 => HttpStatus::MethodNotAllowed,
            406 => HttpStatus::NotAcceptable,
            407 => HttpStatus::ProxyAuthenticationRequired,
            408 => HttpStatus::RequestTimeout,
            409 => HttpStatus::Conflict,
            410 => HttpStatus::Gone,
            411 => HttpStatus::LengthRequired,
            412 => HttpStatus::PreconditionFailed,
            413 => HttpStatus::PayloadTooLarge,
            414 => HttpStatus::URITooLong,
            415 => HttpStatus::UnsupportedMediaType,
            416 => HttpStatus::RangeNotSatisfiable,
            417 => HttpStatus::ExpectationFailed,
            418 => HttpStatus::ImATeapot,
            421 => HttpStatus::MisdirectedRequest,
            422 => HttpStatus::UnprocessableEntity,
            423 => HttpStatus::Locked,
            424 => HttpStatus::FailedDependency,
            425 => HttpStatus::TooEarly,
            426 => HttpStatus::UpgradeRequired,
            428 => HttpStatus::PreconditionRequired,
            429 => HttpStatus::TooManyRequests,
            431 => HttpStatus::RequestHeaderFieldsTooLarge,
            451 => HttpStatus::UnavailableForLegalReasons,
            500 => HttpStatus::InternalServerError,
            501 => HttpStatus::NotImplemented,
            502 => HttpStatus::BadGateway,
            503 => HttpStatus::ServiceUnavailable,
            504 => HttpStatus::GatewayTimeout,
            505 => HttpStatus::HTTPVersionNotSupported,
            506 => HttpStatus::VariantAlsoNegotiates,
            507 => HttpStatus::InsufficientStorage,
            508 => HttpStatus::LoopDetected,
            510 => HttpStatus::NotExtended,
            511 => HttpStatus::NetworkAuthenticationRequired,
            // If we cannot parse the http status code, its a server error
            _ => HttpStatus::InternalServerError,
        }
    }

    fn status_type(&self) -> HttpStatusType {
        match *self as u16 {
            200..300 => HttpStatusType::Success,
            300..400 => HttpStatusType::Redirect,
            400..500 => HttpStatusType::ClientError,
            500..600 => HttpStatusType::ServerError,
            _ => HttpStatusType::Unknown,
        }
    }
    fn message(&self) -> &'static str {
        match self {
            HttpStatus::Ok => "OK",
            HttpStatus::Created => "Created",
            HttpStatus::Accepted => "Accepted",
            HttpStatus::NonAuthoritativeInformation => "Non-Authoritative Information",
            HttpStatus::NoContent => "No Content",
            HttpStatus::ResetContent => "Reset Content",
            HttpStatus::PartialContent => "Partial Content",
            HttpStatus::MultipleChoices => "Multiple Choices",
            HttpStatus::MovedPermanently => "Moved Permanently",
            HttpStatus::Found => "Found",
            HttpStatus::SeeOther => "See Other",
            HttpStatus::NotModified => "Not Modified",
            HttpStatus::UseProxy => "Use Proxy",
            HttpStatus::TemporaryRedirect => "Temporary Redirect",
            HttpStatus::PermanentRedirect => "Permanent Redirect",
            HttpStatus::BadRequest => "Bad Request",
            HttpStatus::Unauthorized => "Unauthorized",
            HttpStatus::PaymentRequired => "Payment Required",
            HttpStatus::Forbidden => "Forbidden",
            HttpStatus::NotFound => "Not Found",
            HttpStatus::MethodNotAllowed => "Method Not Allowed",
            HttpStatus::NotAcceptable => "Not Acceptable",
            HttpStatus::ProxyAuthenticationRequired => "Proxy Authentication Required",
            HttpStatus::RequestTimeout => "Request Timeout",
            HttpStatus::Conflict => "Conflict",
            HttpStatus::Gone => "Gone",
            HttpStatus::LengthRequired => "Length Required",
            HttpStatus::PreconditionFailed => "Precondition Failed",
            HttpStatus::PayloadTooLarge => "Payload Too Large",
            HttpStatus::URITooLong => "URI Too Long",
            HttpStatus::UnsupportedMediaType => "Unsupported Media Type",
            HttpStatus::RangeNotSatisfiable => "Range Not Satisfiable",
            HttpStatus::ExpectationFailed => "Expectation Failed",
            HttpStatus::ImATeapot => "I'm a teapot",
            HttpStatus::MisdirectedRequest => "Misdirected Request",
            HttpStatus::UnprocessableEntity => "Unprocessable Entity",
            HttpStatus::Locked => "Locked",
            HttpStatus::FailedDependency => "Failed Dependency",
            HttpStatus::TooEarly => "Too Early",
            HttpStatus::UpgradeRequired => "Upgrade Required",
            HttpStatus::PreconditionRequired => "Precondition Required",
            HttpStatus::TooManyRequests => "Too Many Requests",
            HttpStatus::RequestHeaderFieldsTooLarge => "Request Header Fields Too Large",
            HttpStatus::UnavailableForLegalReasons => "Unavailable For Legal Reasons",
            HttpStatus::InternalServerError => "Internal Server Error",
            HttpStatus::NotImplemented => "Not Implemented",
            HttpStatus::BadGateway => "Bad Gateway",
            HttpStatus::ServiceUnavailable => "Service Unavailable",
            HttpStatus::GatewayTimeout => "Gateway Timeout",
            HttpStatus::HTTPVersionNotSupported => "HTTP Version Not Supported",
            HttpStatus::VariantAlsoNegotiates => "Variant Also Negotiates",
            HttpStatus::InsufficientStorage => "Insufficient Storage",
            HttpStatus::LoopDetected => "Loop Detected",
            HttpStatus::NotExtended => "Not Extended",
            HttpStatus::NetworkAuthenticationRequired => "Network Authentication Required",
        }
    }
}


pub type HttpHeaders = BTreeMap<String, String>;
pub type HttpBody = Option<String>;
pub type HttpServerContext = HashMap<String, String>;


// ########################################################################
// ############################### Tests ##################################
// ########################################################################
#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================
    // ================= HttpProtocol Tests ====================
    // =========================================================
    #[test]
    fn test_http_protocol_from_str() {
        assert_eq!(
            "HTTP/1".parse::<HttpProtocol>().unwrap(),
            HttpProtocol::Http1
        );
        assert_eq!(
            "HTTP/1.1".parse::<HttpProtocol>().unwrap(),
            HttpProtocol::Http1_1
        );
        assert!("HTTP/2".parse::<HttpProtocol>().is_err());
        assert!("HTTP/3".parse::<HttpProtocol>().is_err());
        assert!("HTTP/1.0".parse::<HttpProtocol>().is_err());
    }
    #[test]
    fn test_http_protocol_display() {
        assert_eq!(format!("{}", HttpProtocol::Http1), "HTTP/1");
        assert_eq!(format!("{}", HttpProtocol::Http1_1), "HTTP/1.1");
    }
    // =========================================================
    // =================== HttpError Tests =====================
    // =========================================================
    #[test]
    fn test_http_status_new() {
        let error = HttpError::new(HttpStatus::BadRequest, "dummy error");
        assert_eq!(error.message, "dummy error");
        assert_eq!(error.status, HttpStatus::BadRequest);
    }
    #[test]
    fn test_http_error_display() {
        let error = HttpError::new(HttpStatus::BadRequest, "dummy error");
        assert_eq!(format!("{}", error), "HttpError: dummy error");
    }

    // =========================================================
    // ================== HttpMethod Tests =====================
    // =========================================================
    #[test]
    fn test_http_method_parse_ok() {
        let correct_methods = ["GET", "POST", "PUT", "DELETE"];
        assert!(correct_methods
            .iter()
            .all(|x| HttpMethod::from_str(x).is_ok()));
    }

    #[test]
    fn test_http_method_parse_error() {
        let incorrect_methods = ["Get", "pOst", "puT", "dELETE"];
        assert!(incorrect_methods
            .iter()
            .all(|x| HttpMethod::from_str(x).is_err()));

        let not_recognized_methods = ["method1", "foo", "bar", "HTTP", "/item/uri/path"];
        assert!(not_recognized_methods
            .iter()
            .all(|x| HttpMethod::from_str(x).is_err()));
    }
    #[test]
    fn test_http_method_display() {
        assert_eq!(format!("{}", HttpMethod::DELETE), "DELETE");
        assert_eq!(format!("{}", HttpMethod::GET), "GET");
        assert_eq!(format!("{}", HttpMethod::PATCH), "PATCH");
        assert_eq!(format!("{}", HttpMethod::POST), "POST");
        assert_eq!(format!("{}", HttpMethod::PUT), "PUT");
    }

    #[test]
    fn test_http_status_display() {
        for status in [HttpStatus::Ok, HttpStatus::GatewayTimeout] {
            let (status_code, message) = (status as u16, status.message());
            assert_eq!(
                format!("{}", status),
                format!("{} {}", status_code, message)
            )
        }
    }
}
