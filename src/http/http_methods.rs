use std::fmt::Display;

const METHODS: &[(u16, &str)] = &[
    (0, "GET"),
    (1, "POST"),
    (2, "PUT"),
    (3, "DELETE"),
    (4, "PATCH"),
    (5, "HEAD"),
    (6, "OPTIONS"),
    (7, "CONNECT"),
    (8, "TRACE"),
];
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
/// HTTP methods
/// # Example
/// ```rust
/// use http::http_methods::HttpMethod;
/// use http::request::Request;
/// use http::response::Response;
/// use route::Route;
///
/// let route = Route::new("/", index, HttpMethod::GET);
///
/// fn index(request: Request) -> Response {
///     Response::new(
///         StatusCode::Ok,
///         ContentType::TextHtmlCharsetUtf8,
///         Vec::new(),
///         Vec::new(),
///         fs::read_to_string("./static/index.html").unwrap(),
///     )
/// }
/// ```
pub enum HttpMethod {
    GET = 0,
    POST = 1,
    PUT = 2,
    DELETE = 3,
    PATCH = 4,
    HEAD = 5,
    OPTIONS = 6,
    CONNECT = 7,
    TRACE = 8,
}

impl Display for HttpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", HttpMethod::get_method_str(*self))
    }
}

impl HttpMethod {
    pub fn get_method_str(method: HttpMethod) -> String {
        METHODS
            .binary_search_by(|&(code, _)| code.cmp(&(method as u16)))
            .map(|idx| METHODS[idx].1)
            .unwrap()
            .to_string()
    }

    pub fn get_method(method: &str) -> HttpMethod {
        match method.to_uppercase().as_str() {
            "GET" => HttpMethod::GET,
            "POST" => HttpMethod::POST,
            "PUT" => HttpMethod::PUT,
            "DELETE" => HttpMethod::DELETE,
            "PATCH" => HttpMethod::PATCH,
            "HEAD" => HttpMethod::HEAD,
            "OPTIONS" => HttpMethod::OPTIONS,
            "CONNECT" => HttpMethod::CONNECT,
            "TRACE" => HttpMethod::TRACE,
            _ => HttpMethod::GET,
        }
    }
}
