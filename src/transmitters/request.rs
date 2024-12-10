use crate::http::{http_cookies::*, http_headers::*, http_methods::*};
use crate::server;
use crate::utils::logger::Logger;
use std::fmt::Display;

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

    pub fn set_path_variables(&mut self, path_variables: Vec<String>) -> &mut Self {
        self.path_variables = path_variables;
        self
    }
    pub fn get_path_variables(&self) -> Vec<String> {
        self.path_variables.clone()
    }
    pub fn get_header(&self, key: &str) -> Option<Header> {
        for header in &self.headers {
            if header.key == key {
                return Some(header.clone());
            }
        }
        None
    }
    pub fn get_cookie(&self, key: &str) -> Option<Cookie> {
        for cookie in &self.cookies {
            if cookie.key == key {
                return Some(cookie.clone());
            }
        }
        None
    }
    pub fn set_method(&mut self, method: HttpMethod) -> &mut Self {
        self.method = method;
        self
    }
    pub fn set_path(&mut self, path: String) -> &mut Self {
        self.path = path;
        self
    }
    pub fn set_headers(&mut self, headers: Vec<Header>) -> &mut Self {
        self.headers = headers;
        self
    }
    pub fn add_header(&mut self, header: Header) -> &mut Self {
        self.headers.push(header);
        self
    }
    pub fn set_cookies(&mut self, cookies: Vec<Cookie>) -> &mut Self {
        self.cookies = cookies;
        self
    }
    pub fn add_cookie(&mut self, cookie: Cookie) -> &mut Self {
        self.cookies.push(cookie);
        self
    }
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
