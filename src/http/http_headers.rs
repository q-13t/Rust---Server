use std::fmt::Display;

#[derive(Clone, Debug, PartialEq, Eq)]
/// HTTP headers
///
/// Header is a tuple of key and value
///
/// # Example
/// ``` rust
/// use http::http_headers::Header;
/// let header = Header::new("Content-Type".to_string(), "application/json".to_string());
/// assert_eq!(header.key(), "Content-Type");
/// assert_eq!(header.value(), "application/json");
/// ```
pub struct Header {
    pub key: String,
    pub value: String,
}
#[allow(unused)]
impl Header {
    pub fn new(key: String, value: String) -> Header {
        Header { key, value }
    }
    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

impl Display for Header {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.key, self.value)
    }
}
