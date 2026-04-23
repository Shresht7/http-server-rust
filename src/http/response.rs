//! # HTTP Response
//!
//! A HTTP Response is the message sent by a server in response to an HTTP request.
//!
//! HTTP Response is made up of three parts, each separated by a [CRLF](https://developer.mozilla.org/en-US/docs/Glossary/CRLF) (`\r\n`):
//! 1. Status Line: Contains the HTTP version, status code, and reason phrase. Example: `HTTP/1.1 200 OK`
//! 2. Headers: Key-value pairs that provide additional information about the response. Example: `Content-Type: text/html`
//! 3. Body: (Optional) The actual content of the response, which can be HTML, JSON, or any other data format. Example: `<html><body><h1>Hello, World!</h1></body></html>`

use crate::http;
use crate::http::constants::CRLF;

// -----------
// STATUS LINE
// -----------

/// Represents the status line of an HTTP response, containing the HTTP version, status code, and reason phrase.
/// Example: `HTTP/1.1 200 OK`
pub struct StatusLine {
    version: http::Version,
    status_code: u16,
    reason_phrase: String,
}

impl Default for StatusLine {
    fn default() -> Self {
        Self {
            version: http::Version::default(),
            status_code: 200,
            reason_phrase: "OK".to_string(),
        }
    }
}

impl StatusLine {
    /// Creates a new `StatusLine` with the specified HTTP version, status code, and reason phrase.
    fn new(version: http::Version, status_code: u16, reason_phrase: &str) -> Self {
        Self {
            version,
            status_code,
            reason_phrase: reason_phrase.to_string(),
        }
    }

    /// Updates the status code and reason phrase of the status line.
    fn with(&mut self, status_code: u16, reason_phrase: &str) -> &mut Self {
        self.status_code = status_code;
        self.reason_phrase = reason_phrase.to_string();
        self
    }
}

impl std::fmt::Display for StatusLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}{}",
            self.version, self.status_code, self.reason_phrase, CRLF
        )
    }
}

// --------
// RESPONSE
// --------

#[derive(Default)]
pub struct Response {
    pub status_line: StatusLine,
    pub headers: http::Headers,
    pub body: http::Body,
}

impl Response {
    /// Creates a new `Response` instance with the specified status line, headers, and body.
    pub fn status(mut self, code: u16, reason: &str) -> Self {
        self.status_line.with(code, reason);
        self
    }

    /// Adds a header to the response with the specified key and value.
    pub fn header<T: AsRef<str>>(mut self, key: T, value: T) -> Self {
        self.headers
            .insert(key.as_ref().to_string(), value.as_ref().to_string());
        self
    }

    pub fn body(mut self, content: &str) -> Self {
        self.body = http::Body::new(content);
        self
    }

    /// Converts the `Response` into a byte vector that can be sent over a network stream.
    pub fn as_bytes(&self) -> Vec<u8> {
        self.to_string().into_bytes()
    }
}

impl std::fmt::Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{}", self.status_line, self.headers, self.body)
    }
}
