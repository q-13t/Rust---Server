use std::fmt::Display;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cookie {
    pub key: String,
    pub value: String,
}
#[allow(unused)]
impl Cookie {
    pub fn new(key: String, value: String) -> Cookie {
        Cookie { key, value }
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

impl Display for Cookie {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}", self.key, self.value)
    }
}
