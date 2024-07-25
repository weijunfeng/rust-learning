use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Method {
    Get,
    Post,
    Undefined,
}

impl From<&str> for Method {
    fn from(value: &str) -> Self {
        match value {
            "GET" => Method::Get,
            "POST" => Method::Post,
            _ => Method::Undefined
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Version {
    V1_1,
    V2_0,
    Undefined,
}

impl From<&str> for Version {
    fn from(value: &str) -> Self {
        match value {
            "http/1.1" => Version::V1_1,
            _ => Version::Undefined
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Resource {
    Path(String),
}

#[derive(Debug, PartialEq)]
pub struct HttpRequest {
    pub method: Method,
    pub version: Version,
    pub resource: Resource,
    pub headers: HashMap<String, String>,
    pub msg_body: String,
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_http() {
        assert_eq!(super::Method::Get, "GET".into());
        assert_eq!(super::Version::V1_1, "http/1.1".into())
    }
}