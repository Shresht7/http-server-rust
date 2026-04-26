//! # HTTP Request
//!
//! A HTTP Request is a message sent by a client to a server, requesting a resource or an action.
//!
//! A HTTP Request is made up of three parts, each separated by a [CRLF] (`\r\n`):
//! 1. Request Line: Contains the HTTP method, URI, and version. Example: `GET /index.html HTTP/1.1`
//! 2. Headers: Key-value pairs that provide additional information about the request. Example: `Host: www.example.com`
//! 3. Body: (Optional) The actual content of the request, which can be form data, JSON, or any other data format. Example: `{"name": "John", "age": 30}`
//! Note: The body is only present in certain types of requests, such as POST and PUT.

use crate::http;
use crate::http::constants::CRLF;

use crate::http::headers::ParseHeadersError;
use crate::http::version::ParseVersionError;

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

impl std::str::FromStr for RequestLine {
    type Err = ParseRequestLineError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Split the request line into parts based on whitespace
        let parts: Vec<&str> = s.split_whitespace().collect();

        // Check if the request line has exactly 3 parts: method, URI, and version
        if parts.len() != 3 {
            return Err(ParseRequestLineError::InvalidFormat(s.to_string()));
        }

        // HTTP method
        let method = http::Method::from(parts[0]);

        // URI
        let uri = parts[1].to_string();

        // HTTP version
        let version = http::Version::from_str(parts[2]).map_err(ParseRequestLineError::Version)?;

        Ok(Self {
            method,
            uri,
            version,
        })
    }
}

impl std::fmt::Display for RequestLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}{}", self.method, self.uri, self.version, CRLF)
    }
}

pub struct Request {
    pub request_line: RequestLine,
    pub headers: http::Headers,
    pub body: http::Body,
}

impl std::str::FromStr for Request {
    type Err = ParseRequestError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        // Parse the request line
        let request_line_str = lines.next().ok_or(ParseRequestError::EmptyRequest)?;

        // Parse the request line
        let request_line = request_line_str
            .parse::<RequestLine>()
            .map_err(ParseRequestError::RequestLine)?;

        // Parse headers
        let mut headers = http::Headers::default();
        while let Some(line) = lines.next()
            && !line.is_empty()
        {
            headers
                .parse_line(line)
                .map_err(ParseRequestError::Headers)?;
        }

        // Parse body
        let body = lines.collect::<Vec<&str>>().join(CRLF);
        let body = http::Body::new(&body);

        Ok(Self {
            request_line,
            headers,
            body,
        })
    }
}

// ------
// ERRORS
// ------

#[derive(Debug)]
pub enum ParseRequestError {
    EmptyRequest,
    RequestLine(ParseRequestLineError),
    Headers(ParseHeadersError),
}

impl std::fmt::Display for ParseRequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseRequestError::EmptyRequest => write!(f, "Empty request"),
            ParseRequestError::RequestLine(e) => write!(f, "Failed to parse request line: '{}'", e),
            ParseRequestError::Headers(e) => write!(f, "Failed to parse headers: '{}'", e),
        }
    }
}

impl std::error::Error for ParseRequestError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ParseRequestError::RequestLine(e) => Some(e),
            ParseRequestError::Headers(e) => Some(e),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum ParseRequestLineError {
    InvalidFormat(String),
    Version(ParseVersionError),
}

impl std::fmt::Display for ParseRequestLineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseRequestLineError::InvalidFormat(s) => {
                write!(f, "Invalid request line format: '{}'", s)
            }
            ParseRequestLineError::Version(e) => {
                write!(f, "Failed to parse HTTP version: '{}'", e)
            }
        }
    }
}

impl std::error::Error for ParseRequestLineError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ParseRequestLineError::Version(e) => Some(e),
            _ => None,
        }
    }
}
