use crate::http::constants::CRLF;

// ----
// BODY
// ----

/// Represents the body of an HTTP response, which is the actual content of the response. It can be HTML, JSON, or any other data format.
#[derive(Default)]
pub struct Body(String);

impl Body {
    /// Creates a new `Body` instance with the specified content.
    pub fn new(content: &str) -> Self {
        Self(content.to_string())
    }
}

impl std::fmt::Display for Body {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.0, CRLF)
    }
}
