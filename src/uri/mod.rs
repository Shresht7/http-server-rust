mod authority;
mod query_params;
mod scheme;
use authority::Authority;
use query_params::{ParseQueryParamError, QueryParams};
use scheme::Scheme;

// ---
// URI
// ---

// TODO: Refactor this into an enum with variants for AbsoluteUri and RelativeUri. This will make my life much easier.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Uri {
    /// The URI [`Scheme`] indicating the protocol to be used when accessing the resource.
    pub scheme: Scheme,

    /// The authority component of the URI, which includes the user information (if present), host, and port.
    /// For example, in the URI "https://user:pass@example.com:8080/path", the authority would be "user:pass@example.com:8080".
    /// Generally, only the host / domain-name is used in this section.
    pub authority: Option<Authority>,

    /// The path to the resource being requested.
    pub path: String,

    /// The query parameters in the URI, if present. This is a collection of key-value pairs that appear after the '?' in the URI.
    /// For example, in the URI "/search?q=rust&sort=desc", the query parameters would be "q=rust" and "sort=desc".
    /// Note that query parameters are optional and may be empty if no parameters are specified in the URI.
    ///
    /// The query parameters are typically used to provide additional information to the server about the request,
    /// such as search terms, filters, or pagination details. They are not part of the path but are included in the URI to convey extra data.
    pub query_params: QueryParams,

    /// The fragment identifier (the part after '#') in the URI, if present. This is optional and may be `None` if no fragment is specified.
    /// For example, in the URI "/path/to/resource#section1", the fragment would be "section1".
    ///
    /// Note that the fragment is not sent to the server in HTTP requests; it is only used client-side.
    pub fragment: Option<String>,
}

// Display
// -------

impl std::fmt::Display for Uri {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = self.path.clone();

        if !self.query_params.is_empty() {
            result.push_str(&self.query_params.to_string());
        }

        if let Some(fragment) = &self.fragment {
            result.push('#');
            result.push_str(fragment);
        }

        write!(f, "{}", result)
    }
}

// FromStr
// -------

impl std::str::FromStr for Uri {
    type Err = ParseUriError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(ParseUriError::EmptyPath);
        }

        let mut path = s;
        let mut query_params = QueryParams::new();
        let mut fragment = None;

        // First, extract the scheme from before the first colon (':'), if present. If there is no colon, we assume the scheme is Http by default.
        let scheme = s
            .split(':')
            .next()
            .map(|scheme| Scheme::from(scheme))
            .unwrap_or(Scheme::Http);

        // Check for the presence of a fragment identifier (indicated by '#') and extract it if present
        if let Some(hash_index) = s.find('#') {
            fragment = Some(s[hash_index + 1..].to_string());
            path = &s[..hash_index];
        }

        // Check for the presence of query parameters (indicated by '?') and extract them if present
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
            scheme,
            authority: None, // Authority parsing is not implemented in this version
            path: path.to_string(),
            query_params,
            fragment,
        })
    }
}

// -----
// ERROR
// -----

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
    fn should_display_uri() {
        let uri = Uri {
            scheme: Scheme::Http,
            authority: None,
            path: "/search".to_string(),
            query_params: QueryParams::from(vec![
                ("q".to_string(), "rust".to_string()),
                ("sort".to_string(), "desc".to_string()),
            ]),
            fragment: Some("section-1".to_string()),
        };
        assert_eq!(uri.to_string(), "/search?q=rust&sort=desc#section-1");
    }

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
