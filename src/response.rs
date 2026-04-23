//! # HTTP Response
//!
//! A HTTP Response is the message sent by a server in response to an HTTP request.
//!
//! HTTP Response is made up of three parts, each separated by a [CRLF](https://developer.mozilla.org/en-US/docs/Glossary/CRLF) (`\r\n`):
//! 1. Status Line: Contains the HTTP version, status code, and reason phrase. Example: `HTTP/1.1 200 OK`
//! 2. Headers: Key-value pairs that provide additional information about the response. Example: `Content-Type: text/html`
//! 3. Body: (Optional) The actual content of the response, which can be HTML, JSON, or any other data format. Example: `<html><body><h1>Hello, World!</h1></body></html>`

use std::collections::HashMap;

use super::constants::CRLF;

// -----------
// STATUS LINE
// -----------

/// The default HTTP version to use in the status line if not specified
const DEFAULT_HTTP_VERSION: &str = "HTTP/1.1";

/// Represents the status line of an HTTP response, containing the HTTP version, status code, and reason phrase.
/// Example: `HTTP/1.1 200 OK`
pub struct StatusLine {
    version: String,
    status_code: u16,
    reason_phrase: String,
}

impl Default for StatusLine {
    fn default() -> Self {
        Self {
            version: DEFAULT_HTTP_VERSION.to_string(),
            status_code: 200,
            reason_phrase: "OK".to_string(),
        }
    }
}

impl StatusLine {
    /// Creates a new `StatusLine` with the specified HTTP version, status code, and reason phrase.
    fn new(version: &str, status_code: u16, reason_phrase: &str) -> Self {
        Self {
            version: version.to_string(),
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
        writeln!(
            f,
            "{} {} {}",
            self.version, self.status_code, self.reason_phrase
        )
    }
}

// -------
// HEADERS
// -------

/// Represents the headers of an HTTP response, which are key-value pairs that provide additional information about the response.
/// Example: `Content-Type: text/html` and `Content-Length: 123`
#[derive(Default)]
pub struct Headers(HashMap<String, String>);

impl Headers {
    /// Adds a header to the response with the specified key and value.
    pub fn add(&mut self, key: &str, value: &str) -> &mut Self {
        self.0.insert(key.to_string(), value.to_string());
        self
    }
}

impl std::fmt::Display for Headers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (key, value) in &self.0 {
            writeln!(f, "{}: {}", key, value)?;
        }
        Ok(())
    }
}

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
        write!(f, "{}", self.0)
    }
}

// --------
// RESPONSE
// --------

#[derive(Default)]
pub struct Response {
    pub status_line: StatusLine,
    pub headers: Headers,
    pub body: Body,
}

impl Response {
    /// Creates a new `Response` instance with the specified status line, headers, and body.
    pub fn status(mut self, code: u16, reason: &str) -> Self {
        self.status_line.with(code, reason);
        self
    }

    /// Adds a header to the response with the specified key and value.
    pub fn header(mut self, key: &str, value: &str) -> Self {
        self.headers.add(key, value);
        self
    }

    pub fn body(mut self, content: &str) -> Self {
        self.body = Body::new(content);
        self
    }

    /// Converts the `Response` into a byte vector that can be sent over a network stream.
    pub fn as_bytes(&self) -> Vec<u8> {
        self.to_string().into_bytes()
    }
}

impl std::fmt::Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}{}{}",
            self.status_line, CRLF, self.headers, CRLF, self.body
        )
    }
}
