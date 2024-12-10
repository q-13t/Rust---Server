use crate::http::{http_cookies::*, http_headers::*, http_methods::*};
use crate::server;
use crate::utils::logger::Logger;
use std::fmt::Display;

/// Represents a request
/// # Arguments
/// * `method` - The method of the request : HttpMethod
/// * `path` - The path of the request : String
/// * `path_variables` - The path variables of the request : Vec<String>
/// * `headers` - The headers of the request : Vec<Header>
/// * `cookies` - The cookies of the request : Vec<Cookie>
/// * `data` - The data of the request : String
///
/// # Example
/// ```
/// let request = Request::new(
///     HttpMethod::GET,
///     "/".to_string(),
///     Vec::new(),
///     Vec::new(),
///     Vec::new(),
///     "Hello world".to_string(),
/// );
/// ```
pub struct Request {
    pub method: HttpMethod,
    pub path: String,
    pub path_variables: Vec<String>,
    pub headers: Vec<Header>,
    pub cookies: Vec<Cookie>,
    pub data: String,
}

#[allow(unused)]
impl Request {
    pub fn new(
        method: HttpMethod,
        path: String,
        path_variables: Vec<String>,
        headers: Vec<Header>,
        cookies: Vec<Cookie>,
        data: String,
    ) -> Request {
        Request {
            method,
            path,
            path_variables,
            headers,
            cookies,
            data,
        }
    }

    /// Parse a request from a string
    /// # Arguments
    /// * `payload` - The payload of the request : String
    /// # Returns
    /// * `Request` - The request
    /// # Example
    /// ```
    /// let request = Request::parse("GET / HTTP/1.1\r\nHost: localhost\r\n\r\n".to_string());
    /// ```
    pub fn parse(payload: String) -> Request {
        let logger: Logger = Logger {
            c_name: "Request",
            level: server::get_log_level(),
        };
        logger.debug(&["split", payload.as_str()]);
        let mut lines = payload.lines();

        let request_line = lines.next().unwrap_or_default();
        let mut parts = request_line.split_whitespace();
        let method = HttpMethod::get_method(parts.next().unwrap_or_default());
        let path = parts.next().unwrap_or_default().to_string();

        let path_variables = path
            .split('/')
            .filter(|segment| !segment.is_empty() && segment.starts_with(':'))
            .map(|segment| segment.trim_start_matches(':').to_string())
            .collect();

        let mut headers = Vec::new();
        let mut cookies = Vec::new();
        let mut is_header_section = true;
        let mut data = String::new();

        for line in lines {
            if line.is_empty() {
                is_header_section = false;
                continue;
            }

            if is_header_section {
                if let Some((key, value)) = line.split_once(": ") {
                    let key = key.trim().to_string();
                    let value = value.trim().to_string();

                    if key.eq_ignore_ascii_case("Cookie") {
                        for cookie in value.split("; ") {
                            if let Some((cookie_key, cookie_value)) = cookie.split_once('=') {
                                cookies.push(Cookie {
                                    key: cookie_key.to_string(),
                                    value: cookie_value.to_string(),
                                });
                            }
                        }
                    } else {
                        headers.push(Header { key, value });
                    }
                }
            } else {
                data.push_str(line);
                data.push('\n');
            }
        }

        // Create the Request struct
        Request {
            method,
            path,
            path_variables,
            headers,
            cookies,
            data: data.trim_end().to_string(),
        }
    }
    /// Create an empty request
    /// # Returns
    /// * `Request` - The empty request
    pub fn get_empty() -> Request {
        Request::new(
            HttpMethod::GET,
            "".to_string(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            "".to_string(),
        )
    }

    /// Set the path variables
    /// # Arguments
    /// * `path_variables` - The path variables of the request : Vec<String>
    /// # Returns
    /// * `Request` - The request
    pub fn set_path_variables(&mut self, path_variables: Vec<String>) -> &mut Self {
        self.path_variables = path_variables;
        self
    }
    /// Get the path variables
    /// # Returns
    /// * `Vec<String>` - The path variables of the request : Vec<String>
    pub fn get_path_variables(&self) -> Vec<String> {
        self.path_variables.clone()
    }
    /// Get a header from the request
    /// # Arguments
    /// * `key` - The key of the header : &str
    /// # Returns
    /// * `Option<Header>` - The header : Option<Header>
    pub fn get_header(&self, key: &str) -> Option<Header> {
        for header in &self.headers {
            if header.key == key {
                return Some(header.clone());
            }
        }
        None
    }
    /// Get a cookie from the request
    /// # Arguments
    /// * `key` - The key of the cookie : &str
    /// # Returns
    /// * `Option<Cookie>` - The cookie : Option<Cookie>
    pub fn get_cookie(&self, key: &str) -> Option<Cookie> {
        for cookie in &self.cookies {
            if cookie.key == key {
                return Some(cookie.clone());
            }
        }
        None
    }

    /// Set the method of the request
    /// # Arguments
    /// * `method` - The method of the request : HttpMethod
    /// # Returns
    /// * `Request` - The request
    pub fn set_method(&mut self, method: HttpMethod) -> &mut Self {
        self.method = method;
        self
    }
    /// Set the path of the request
    /// # Arguments
    /// * `path` - The path of the request : String
    /// # Returns
    /// * `Request` - The request
    pub fn set_path(&mut self, path: String) -> &mut Self {
        self.path = path;
        self
    }
    /// Set the headers of the request
    ///     
    /// # Arguments
    /// * `headers` - The headers of the request : Vec<Header>
    /// # Returns
    ///     * `Request` - The request
    pub fn set_headers(&mut self, headers: Vec<Header>) -> &mut Self {
        self.headers = headers;
        self
    }
    /// Add a header to the request
    /// # Arguments
    /// * `header` - The header to add : Header
    /// # Returns
    /// * `Request` - The request
    pub fn add_header(&mut self, header: Header) -> &mut Self {
        self.headers.push(header);
        self
    }

    /// Set the cookies of the request
    /// # Arguments
    /// * `cookies` - The cookies of the request : Vec<Cookie>
    /// # Returns
    /// * `Request` - The request
    pub fn set_cookies(&mut self, cookies: Vec<Cookie>) -> &mut Self {
        self.cookies = cookies;
        self
    }

    /// Add a cookie to the request
    /// # Arguments
    /// * `cookie` - The cookie to add : Cookie
    /// # Returns
    /// * `Request` - The request
    pub fn add_cookie(&mut self, cookie: Cookie) -> &mut Self {
        self.cookies.push(cookie);
        self
    }

    /// Set the data of the request
    /// # Arguments
    /// * `data` - The data of the request : String
    /// # Returns
    /// * `Request` - The request
    pub fn set_data(&mut self, data: String) -> &mut Self {
        self.data = data;
        self
    }
}

impl Display for Request {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cookies_str = self
            .cookies
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
            .join("\n");
        let headers_str = self
            .headers
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
            .join("\n");

        return write!(
            f,
            "{}\n{}\n{}\n{}\n{}",
            self.method, self.path, headers_str, cookies_str, self.data
        );
    }
}
