use std::collections::HashMap;

use crate::http::constants::CRLF;

// -------
// HEADERS
// -------

/// Represents the headers of an HTTP response, which are key-value pairs that provide additional information about the response.
/// Example: `Content-Type: text/html` and `Content-Length: 123`
#[derive(Debug, Default)]
pub struct Headers(HashMap<String, String>);

impl Headers {
    /// Creates a new `Headers` instance.
    pub fn new() -> Self {
        Headers(HashMap::new())
    }

    pub fn parse_line<T: AsRef<str>>(&mut self, line: T) -> Result<(), ParseHeadersError> {
        let line = line.as_ref();
        if let Some((key, value)) = line.split_once(':') {
            self.insert(key.trim().to_string(), value.trim().to_string());
            Ok(())
        } else {
            Err(ParseHeadersError(line.to_string()))
        }
    }
}

impl std::ops::Deref for Headers {
    type Target = HashMap<String, String>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Headers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::fmt::Display for Headers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (key, value) in &self.0 {
            write!(f, "{}: {}{}", key, value, CRLF)?;
        }
        write!(f, "{}", CRLF)?;
        Ok(())
    }
}

impl std::str::FromStr for Headers {
    type Err = ParseHeadersError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut headers = Headers::new();
        for line in s.lines() {
            if line.is_empty() {
                break;
            }
            headers.parse_line(line)?;
        }
        Ok(headers)
    }
}

// ------
// ERRORS
// ------

#[derive(Debug)]
pub struct ParseHeadersError(String);

impl std::fmt::Display for ParseHeadersError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to parse header: {}", self.0)
    }
}

impl std::error::Error for ParseHeadersError {}

// -----
// TESTS
// -----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_behave_like_a_hashmap() {
        let mut headers = Headers::new();
        headers.insert("Content-Type".to_string(), "text/html".to_string());
        headers.insert("Content-Length".to_string(), "123".to_string());
        assert_eq!(headers.get("Content-Type"), Some(&"text/html".to_string()));
        assert_eq!(headers.get("Content-Length"), Some(&"123".to_string()));
    }

    #[test]
    fn should_parse_single_line() {
        let mut headers = Headers::new();
        headers.parse_line("Content-Type: text/html").unwrap();
        assert_eq!(headers.get("Content-Type"), Some(&"text/html".to_string()));
    }

    #[test]
    fn should_fail_to_parse_invalid_line() {
        let mut headers = Headers::new();
        let err = headers.parse_line("InvalidHeader").unwrap_err();
        assert!(matches!(err, ParseHeadersError(_)));
    }

    #[test]
    fn should_parse_multiple_lines() {
        let input = "Content-Type: text/html\r\nContent-Length: 123\r\n\r\n";
        let headers: Headers = input.parse().unwrap();
        assert_eq!(headers.get("Content-Type"), Some(&"text/html".to_string()));
        assert_eq!(headers.get("Content-Length"), Some(&"123".to_string()));
    }
}
