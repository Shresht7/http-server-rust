use std::collections::HashMap;

use crate::http::constants::CRLF;

// -------
// HEADERS
// -------

/// Represents the headers of an HTTP response, which are key-value pairs that provide additional information about the response.
/// Example: `Content-Type: text/html` and `Content-Length: 123`
#[derive(Default)]
pub struct Headers(HashMap<String, String>);

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
