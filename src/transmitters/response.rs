use std::fmt::Display;

use crate::http::{
    http_codes::*, http_content_types::ContentType, http_cookies::*, http_headers::*,
};

/// Represents a response
/// # Arguments
/// * `status` - The status of the response
/// * `resp_type` - The type of the response
/// * `headers` - The headers of the response
/// * `cookies` - The cookies of the response
/// * `data` - The data of the response
///
/// # Returns
/// * `Response` - The response object
///
/// # Example
/// ```
/// use http::http_codes::StatusCode;
/// use http::http_content_types::ContentType;  
/// use http::http_headers::Header;
///     
/// let response = Response::new(
///     StatusCode::Ok,
///     ContentType::ApplicationJson,
///     Vec::new(),
///     Vec::new(),
///     '{"token": true"}'.to_string(),
/// );
/// ```
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

    /// Sets the content type of the response
    /// # Arguments
    /// * `content_type` - The content type of the response : ContentType
    pub fn set_content_type(&mut self, content_type: ContentType) -> &mut Self {
        self.content_type = content_type;
        self
    }
    /// Gets the content type of the response
    /// # Returns
    /// * `ContentType` - The content type of the response: ContentType
    pub fn get_content_type(&self) -> ContentType {
        self.content_type
    }
    /// Sets the status of the response
    /// # Arguments
    /// * `status` - The status of the response : StatusCode
    /// # Returns
    /// * `response` - The response object: Response
    pub fn set_status(&mut self, status: StatusCode) -> &mut Self {
        self.status = status;
        self
    }
    /// Gets the status of the response
    /// # Arguments
    /// * `headers` - The status of the response : Vec<Header>
    /// # Returns
    /// * `response` - The status of the response: Response
    pub fn set_headers(&mut self, headers: Vec<Header>) -> &mut Self {
        self.headers = headers;
        self
    }
    /// Adds a header to the response
    /// # Arguments
    /// * `header` - The header to add : Header
    /// # Returns
    /// * `response` - The response object: Response
    pub fn add_header(&mut self, header: Header) -> &mut Self {
        self.headers.push(header);
        self
    }
    /// Sets the cookies of the response
    /// # Arguments
    /// * `cookies` - The cookies of the response : Vec<Cookie>
    /// # Returns
    /// * `response` - The response object: Response
    pub fn set_cookies(&mut self, cookies: Vec<Cookie>) -> &mut Self {
        self.cookies = cookies;
        self
    }
    /// Adds a cookie to the response
    /// # Arguments
    /// * `cookie` - The cookie to add : Cookie
    /// # Returns
    /// * `response` - The response object: Response
    pub fn add_cookie(&mut self, cookie: Cookie) -> &mut Self {
        self.cookies.push(cookie);
        self
    }
    /// Sets the data of the response
    /// # Arguments
    /// * `data` - The data of the response : String
    /// # Returns
    /// * `response` - The response object: Response
    pub fn set_data(&mut self, data: String) -> &mut Self {
        self.data = data;
        self
    }
    /// Gets a header from the response
    /// # Arguments
    /// * `key` - The key of the header : &str
    /// # Returns
    /// * `Option<Header>` - The header: Option<Header>
    pub fn get_header(&self, key: &str) -> Option<Header> {
        for header in &self.headers {
            if header.key == key {
                return Some(header.clone());
            }
        }
        None
    }
    /// Gets a cookie from the response
    /// # Arguments
    /// * `key` - The key of the cookie : &str
    /// # Returns
    /// * `Option<Cookie>` - The cookie: Option<Cookie>
    pub fn get_cookie(&self, key: &str) -> Option<Cookie> {
        for cookie in &self.cookies {
            if cookie.key == key {
                return Some(cookie.clone());
            }
        }
        None
    }
    /// Prepares the response:
    /// <br>
    /// Packs the status, headers, cookies and data into a string and converts the response into a string to be sended
    /// # Returns
    /// * `String` - The response: String
    pub fn prepare(&self) -> String {
        let cookies_str = self
            .cookies
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
            .join("\n");

        let mut pre_response_headers = self.headers.clone();
        pre_response_headers.push(Header::new(
            "Access-Control-Allow-Origin".to_string(),
            "*".to_string(),
        ));
        pre_response_headers.push(Header::new(
            "Access-Control-Allow-Credentials".to_string(),
            "true".to_string(),
        ));
        pre_response_headers.push(Header::new(
            "Access-Control-Allow-Methods".to_string(),
            "POST, GET, OPTIONS, PUT, DELETE, PATCH".to_string(),
        ));
        pre_response_headers.push(Header::new(
            "Access-Control-Allow-Headers".to_string(),
            "Content-Type, Authorization".to_string(),
        ));

        let headers_str = pre_response_headers
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
