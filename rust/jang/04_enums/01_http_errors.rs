#[derive(Debug)]
pub enum HttpClientError {
    BadRequest,
    Unauthorized,
    PaymentRequired,
    Forbidden,
    NotFound,
    MethodNotAllowed,
    NotAcceptable,
    ProxyAuthenticationRequired,
    RequestTimeout,
    Conflict,
    Gone,
    LengthRequired,
    PreconditionFailed,
    PayloadTooLarge,
    UriTooLong,
    UnsupportedMediaType,
    RangeNotSatisfiable,
    ExpectationFailed,
    MisdirectedRequest,
    UnprocessableEntity,
    Locked,
    FailedDependency,
    UpgradeRequired,
    PreconditionRequired,
    TooManyRequests,
    RequestHeaderFieldsTooLarge,
    UnavailableForLegalReasons,
}

impl HttpClientError {
    pub fn status_code(&self) -> u16 {
        match self {
            HttpClientError::BadRequest => 400,
            HttpClientError::Unauthorized => 401,
            HttpClientError::PaymentRequired => 402,
            HttpClientError::Forbidden => 403,
            HttpClientError::NotFound => 404,
            HttpClientError::MethodNotAllowed => 405,
            HttpClientError::NotAcceptable => 406,
            HttpClientError::ProxyAuthenticationRequired => 407,
            HttpClientError::RequestTimeout => 408,
            HttpClientError::Conflict => 409,
            HttpClientError::Gone => 410,
            HttpClientError::LengthRequired => 411,
            HttpClientError::PreconditionFailed => 412,
            HttpClientError::PayloadTooLarge => 413,
            HttpClientError::UriTooLong => 414,
            HttpClientError::UnsupportedMediaType => 415,
            HttpClientError::RangeNotSatisfiable => 416,
            HttpClientError::ExpectationFailed => 417,
            HttpClientError::MisdirectedRequest => 421,
            HttpClientError::UnprocessableEntity => 422,
            HttpClientError::Locked => 423,
            HttpClientError::FailedDependency => 424,
            HttpClientError::UpgradeRequired => 426,
            HttpClientError::PreconditionRequired => 428,
            HttpClientError::TooManyRequests => 429,
            HttpClientError::RequestHeaderFieldsTooLarge => 431,
            HttpClientError::UnavailableForLegalReasons => 451,
        }
    }

    pub fn reason_phrase(&self) -> &'static str {
        match self {
            HttpClientError::BadRequest => "Bad Request",
            HttpClientError::Unauthorized => "Unauthorized",
            HttpClientError::PaymentRequired => "Payment Required",
            HttpClientError::Forbidden => "Forbidden",
            HttpClientError::NotFound => "Not Found",
            HttpClientError::MethodNotAllowed => "Method Not Allowed",
            HttpClientError::NotAcceptable => "Not Acceptable",
            HttpClientError::ProxyAuthenticationRequired => "Proxy Authentication Required",
            HttpClientError::RequestTimeout => "Request Timeout",
            HttpClientError::Conflict => "Conflict",
            HttpClientError::Gone => "Gone",
            HttpClientError::LengthRequired => "Length Required",
            HttpClientError::PreconditionFailed => "Precondition Failed",
            HttpClientError::PayloadTooLarge => "Payload Too Large",
            HttpClientError::UriTooLong => "URI Too Long",
            HttpClientError::UnsupportedMediaType => "Unsupported Media Type",
            HttpClientError::RangeNotSatisfiable => "Range Not Satisfiable",
            HttpClientError::ExpectationFailed => "Expectation Failed",
            HttpClientError::MisdirectedRequest => "Misdirected Request",
            HttpClientError::UnprocessableEntity => "Unprocessable Entity",
            HttpClientError::Locked => "Locked",
            HttpClientError::FailedDependency => "Failed Dependency",
            HttpClientError::UpgradeRequired => "Upgrade Required",
            HttpClientError::PreconditionRequired => "Precondition Required",
            HttpClientError::TooManyRequests => "Too Many Requests",
            HttpClientError::RequestHeaderFieldsTooLarge => "Request Header Fields Too Large",
            HttpClientError::UnavailableForLegalReasons => "Unavailable For Legal Reasons",
        }
    }
}

fn main() {
    let error = HttpClientError::NotFound;
    println!(
        "Error: {:?}, Status Code: {}, Reason: {}",
        error,
        error.status_code(),
        error.reason_phrase()
    );
}
