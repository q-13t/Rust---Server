use std::fmt::Display;

const STATUSES: &[(u16, &str)] = &[
    (100, "HTTP/1.1 100 Continue"),
    (101, "HTTP/1.1 101 Switching Protocols"),
    (102, "HTTP/1.1 102 Processing"),
    (103, "HTTP/1.1 103 Early Hints"),
    (200, "HTTP/1.1 200 OK"),
    (201, "HTTP/1.1 201 Created"),
    (202, "HTTP/1.1 202 Accepted"),
    (203, "HTTP/1.1 203 Non-Authoritative Information"),
    (204, "HTTP/1.1 204 No Content"),
    (205, "HTTP/1.1 205 Reset Content"),
    (206, "HTTP/1.1 206 Partial Content"),
    (207, "HTTP/1.1 207 Multi-Status"),
    (208, "HTTP/1.1 208 Already Reported"),
    (226, "HTTP/1.1 226 IM Used"),
    (300, "HTTP/1.1 300 Multiple Choices"),
    (301, "HTTP/1.1 301 Moved Permanently"),
    (302, "HTTP/1.1 302 Found"),
    (303, "HTTP/1.1 303 See Other"),
    (304, "HTTP/1.1 304 Not Modified"),
    (305, "HTTP/1.1 305 Use Proxy"),
    (307, "HTTP/1.1 307 Temporary Redirect"),
    (308, "HTTP/1.1 308 Permanent Redirect"),
    (400, "HTTP/1.1 400 Bad Request"),
    (401, "HTTP/1.1 401 Unauthorized"),
    (402, "HTTP/1.1 402 Payment Required"),
    (403, "HTTP/1.1 403 Forbidden"),
    (404, "HTTP/1.1 404 Not Found"),
    (405, "HTTP/1.1 405 Method Not Allowed"),
    (406, "HTTP/1.1 406 Not Acceptable"),
    (407, "HTTP/1.1 407 Proxy Authentication Required"),
    (408, "HTTP/1.1 408 Request Timeout"),
    (409, "HTTP/1.1 409 Conflict"),
    (410, "HTTP/1.1 410 Gone"),
    (411, "HTTP/1.1 411 Length Required"),
    (412, "HTTP/1.1 412 Precondition Failed"),
    (413, "HTTP/1.1 413 Payload Too Large"),
    (414, "HTTP/1.1 414 URI Too Long"),
    (415, "HTTP/1.1 415 Unsupported Media Type"),
    (416, "HTTP/1.1 416 Range Not Satisfiable"),
    (417, "HTTP/1.1 417 Expectation Failed"),
    (418, "HTTP/1.1 418 I'm a teapot"),
    (421, "HTTP/1.1 421 Misdirected Request"),
    (422, "HTTP/1.1 422 Unprocessable Entity"),
    (423, "HTTP/1.1 423 Locked"),
    (424, "HTTP/1.1 424 Failed Dependency"),
    (426, "HTTP/1.1 426 Upgrade Required"),
    (428, "HTTP/1.1 428 Precondition Required"),
    (429, "HTTP/1.1 429 Too Many Requests"),
    (431, "HTTP/1.1 431 Request Header Fields Too Large"),
    (451, "HTTP/1.1 451 Unavailable For Legal Reasons"),
    (500, "HTTP/1.1 500 Internal Server Error"),
    (501, "HTTP/1.1 501 Not Implemented"),
    (502, "HTTP/1.1 502 Bad Gateway"),
    (503, "HTTP/1.1 503 Service Unavailable"),
    (504, "HTTP/1.1 504 Gateway Timeout"),
    (505, "HTTP/1.1 505 HTTP Version Not Supported"),
    (506, "HTTP/1.1 506 Variant Also Negotiates"),
    (507, "HTTP/1.1 507 Insufficient Storage"),
    (508, "HTTP/1.1 508 Loop Detected"),
    (510, "HTTP/1.1 510 Not Extended"),
    (511, "HTTP/1.1 511 Network Authentication Required"),
];

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[allow(unused)]
/// HTTP status codes
/// # Example
/// ``` rust
/// use http::http_codes::StatusCode;
/// let status_code = StatusCode::Ok;
/// assert_eq!(status_code, StatusCode::Ok);
/// ```
pub enum StatusCode {
    Continue = 100,
    SwitchingProtocols = 101,
    Processing = 102,
    EarlyHints = 103,
    Ok = 200,
    Created = 201,
    Accepted = 202,
    NonAuthoritativeInformation = 203,
    NoContent = 204,
    ResetContent = 205,
    PartialContent = 206,
    MultiStatus = 207,
    AlreadyReported = 208,
    ImUsed = 226,
    MultipleChoices = 300,
    MovedPermanently = 301,
    Found = 302,
    SeeOther = 303,
    NotModified = 304,
    UseProxy = 305,
    TemporaryRedirect = 307,
    PermanentRedirect = 308,
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
    UriTooLong = 414,
    UnsupportedMediaType = 415,
    RangeNotSatisfiable = 416,
    ExpectationFailed = 417,
    ImATeapot = 418,
    MisdirectedRequest = 421,
    UnprocessableEntity = 422,
    Locked = 423,
    FailedDependency = 424,
    UpgradeRequired = 426,
    PreconditionRequired = 428,
    TooManyRequests = 429,
    RequestHeaderFieldsTooLarge = 431,
    UnavailableForLegalReasons = 451,
    InternalServerError = 500,
    NotImplemented = 501,
    BadGateway = 502,
    ServiceUnavailable = 503,
    GatewayTimeout = 504,
    HttpVersionNotSupported = 505,
    VariantAlsoNegotiates = 506,
    InsufficientStorage = 507,
    LoopDetected = 508,
    NotExtended = 510,
    NetworkAuthenticationRequired = 511,
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

/// Returns the status line for a given status code
/// # Arguments
/// * `status` - The status code : StatusCode
/// # Returns
/// * `String` - The status line : String
pub fn get_status_line(status: StatusCode) -> String {
    STATUSES
        .binary_search_by(|&(code, _)| code.cmp(&(status as u16)))
        .map(|idx| STATUSES[idx].1)
        .unwrap()
        .to_string()
}
