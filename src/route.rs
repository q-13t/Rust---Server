use crate::{HttpMethod, Request, Response};

use std::fmt::Display;

/// Represents a route
///
/// # Arguments
/// * `path` - The path of the route
/// * `handler` - The handler of the route
/// * `method` - The method of the route
///
/// # Example
/// ```
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
pub struct Route {
    pub path: String,
    pub handler: Box<dyn Fn(Request) -> Response + Send + Sync + 'static>,
    pub method: HttpMethod,
}

impl Route {
    pub fn new(
        path: &str,
        handler: impl Fn(Request) -> Response + Send + Sync + 'static,
        method: HttpMethod,
    ) -> Route {
        Route {
            path: path.to_string(),
            handler: Box::new(handler),
            method,
        }
    }
    pub fn call(&self, data: Request) -> Response {
        (self.handler)(data)
    }
}

impl Display for Route {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {} - <function>", self.method, self.path)
    }
}
