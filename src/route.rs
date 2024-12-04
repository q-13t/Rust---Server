use crate::{HttpMethod, Request, Response};

use std::fmt::Display;

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
