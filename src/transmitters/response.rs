use std::fmt::Display;

use crate::http::{
    http_codes::*, http_content_types::ContentType, http_cookies::*, http_headers::*,
};

pub struct Response {
    pub status: StatusCode,
    pub content_type: ContentType,
    pub headers: Vec<Header>,
    pub cookies: Vec<Cookie>,
    pub data: String,
}
#[allow(unused)]
impl Response {
    pub fn new(
        status: StatusCode,
        resp_type: ContentType,
        headers: Vec<Header>,
        cookies: Vec<Cookie>,
        data: String,
    ) -> Response {
        Response {
            status,
            content_type: resp_type,
            headers,
            cookies,
            data,
        }
    }

    pub fn new_from_tuple(
        tuple: (StatusCode, ContentType, Vec<Header>, Vec<Cookie>, String),
    ) -> Response {
        Response {
            content_type: tuple.1,
            status: tuple.0,
            headers: tuple.2,
            cookies: tuple.3,
            data: tuple.4,
        }
    }
    pub fn get_empty() -> Response {
        Response::new(
            StatusCode::Ok,
            ContentType::TextPlainCharsetUtf8,
            Vec::new(),
            Vec::new(),
            "".to_string(),
        )
    }

    pub fn set_content_type(&mut self, content_type: ContentType) -> &mut Self {
        self.content_type = content_type;
        self
    }
    pub fn get_content_type(&self) -> ContentType {
        self.content_type
    }

    pub fn set_status(&mut self, status: StatusCode) -> &mut Self {
        self.status = status;
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

    pub fn prepare(&self) -> String {
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

        let status_line = get_status_line(self.status);

        format!(
            "{}\n{}\n{}\n{}\n{}",
            status_line, self.content_type, headers_str, cookies_str, self.data
        )
    }
}

impl Default for Response {
    fn default() -> Self {
        Response {
            status: StatusCode::Ok,
            content_type: ContentType::TextPlainCharsetUtf8,
            headers: Vec::new(),
            cookies: Vec::new(),
            data: "".to_string(),
        }
    }
}

impl Display for Response {
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
        let status_line = get_status_line(self.status);

        write!(
            f,
            "{}\n{}\n{}\n{}",
            status_line, headers_str, cookies_str, self.data
        )
    }
}
