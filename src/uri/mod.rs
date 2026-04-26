mod query_params;

use query_params::{ParseQueryParamError, QueryParams};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Uri {
    path: String,
    query_params: QueryParams,
    fragment: Option<String>,
}

impl Default for Uri {
    fn default() -> Self {
        Self {
            path: "/".to_string(),
            query_params: QueryParams::new(),
            fragment: None,
        }
    }
}

impl std::str::FromStr for Uri {
    type Err = ParseUriError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(ParseUriError::EmptyPath);
        }

        let mut path = s;
        let mut query_params = QueryParams::new();
        let mut fragment = None;

        // First, check for the presence of a fragment identifier (indicated by '#') and extract it if present
        if let Some(hash_index) = s.find('#') {
            fragment = Some(s[hash_index + 1..].to_string());
            path = &s[..hash_index];
        }

        // Next, check for the presence of query parameters (indicated by '?') and extract them if present
        if let Some(question_index) = path.find('?') {
            let query_string = &path[question_index + 1..];
            path = &path[..question_index];

            // Parse the query string into key-value pairs and store them in the query_params HashMap
            if !query_string.is_empty() {
                query_params = query_string
                    .parse::<QueryParams>()
                    .map_err(ParseUriError::QueryParam)?;
            }
        }

        Ok(Uri {
            path: path.to_string(),
            query_params,
            fragment,
        })
    }
}

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

// -----
// TESTS
// -----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_basic_uri() {
        let uri_str = "/path/to/resource";
        let uri = uri_str.parse::<Uri>().unwrap();
        assert_eq!(uri.path, "/path/to/resource");
        assert!(uri.query_params.is_empty());
        assert!(uri.fragment.is_none());
    }

    #[test]
    fn should_parse_absolute_uri() {
        let uri_str = "https://example.com/path/to/resource";
        let uri = uri_str.parse::<Uri>().unwrap();
        assert_eq!(uri.path, "https://example.com/path/to/resource");
        assert!(uri.query_params.is_empty());
        assert!(uri.fragment.is_none());
    }

    #[test]
    fn should_parse_uri_with_query_params() {
        let uri_str = "/search?q=rust&sort=desc";
        let uri = uri_str.parse::<Uri>().unwrap();
        assert_eq!(uri.path, "/search");
        assert_eq!(uri.query_params.get("q"), Some(&"rust".to_string()));
        assert_eq!(uri.query_params.get("sort"), Some(&"desc".to_string()));
        assert!(uri.fragment.is_none());
    }

    #[test]
    fn should_parse_uri_with_fragment() {
        let uri_str = "/path/to/resource#section1";
        let uri = uri_str.parse::<Uri>().unwrap();
        assert_eq!(uri.path, "/path/to/resource");
        assert!(uri.query_params.is_empty());
        assert_eq!(uri.fragment, Some("section1".to_string()));
    }

    #[test]
    fn should_parse_uri_with_query_params_and_fragment() {
        let uri_str = "/search?q=rust#results";
        let uri = uri_str.parse::<Uri>().unwrap();
        assert_eq!(uri.path, "/search");
        assert_eq!(uri.query_params.get("q"), Some(&"rust".to_string()));
        assert_eq!(uri.fragment, Some("results".to_string()));
    }

    #[test]
    fn should_handle_empty_query_params() {
        let uri_str = "/search?";
        let uri = uri_str.parse::<Uri>().unwrap();
        assert_eq!(uri.path, "/search");
        assert!(uri.query_params.is_empty());
        assert!(uri.fragment.is_none());
    }

    #[test]
    fn should_handle_empty_fragment() {
        let uri_str = "/path/to/resource#";
        let uri = uri_str.parse::<Uri>().unwrap();
        assert_eq!(uri.path, "/path/to/resource");
        assert!(uri.query_params.is_empty());
        assert_eq!(uri.fragment, Some("".to_string()));
    }

    #[test]
    fn should_fail_on_empty_path() {
        let uri_str = "";
        let result = uri_str.parse::<Uri>();
        assert!(result.is_err());
        assert!(matches!(result.err(), Some(ParseUriError::EmptyPath)));
    }
}
