use crate::http;
use crate::http::constants::CRLF;

// ------------
// REQUEST LINE
// ------------

/// Represents the request line of an HTTP request, containing the HTTP method, URI, and version.
/// Example: `GET /index.html HTTP/1.1`
pub struct RequestLine {
    method: http::Method,
    uri: String,
    version: http::Version,
}

impl std::fmt::Display for RequestLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}{}", self.method, self.uri, self.version, CRLF)
    }
}

pub struct Request {}
