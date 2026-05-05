use super::query_params::ParseQueryParamError;

// ------
// ERRORS
// ------

/// Errors that can occur during URI parsing.
#[derive(Debug)]
pub enum ParseUriError {
    EmptyPath,
    QueryParam(ParseQueryParamError),
}

impl std::fmt::Display for ParseUriError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseUriError::EmptyPath => write!(f, "Empty URI path"),
            ParseUriError::QueryParam(e) => write!(f, "Failed to parse query parameters: {}", e),
        }
    }
}

impl std::error::Error for ParseUriError {}
