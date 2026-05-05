use super::authority::Authority;
use super::errors::ParseUriError;
use super::query_params::QueryParams;
use super::scheme::Scheme;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AbsoluteUri {
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

impl std::fmt::Display for AbsoluteUri {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = format!("{}://", self.scheme);

        if let Some(authority) = &self.authority {
            result.push_str(&authority.to_string());
        }

        result.push_str(&self.path);

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

impl std::str::FromStr for AbsoluteUri {
    type Err = ParseUriError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // First, we need to parse the scheme from the URI string. The scheme is the part of the URI that appears before the first colon (`:`).
        let scheme = Scheme::from_url(s).expect("ParseUriError::InvalidScheme");

        // Next, we need to parse the authority component of the URI, which includes the user information (if present), host, and port.
        // The authority is typically separated from the path by a double slash (`//`).
        let authority = if let Some(authority_start) = s.find("//") {
            let authority_end = s[authority_start + 2..]
                .find('/')
                .map(|index| authority_start + 2 + index)
                .unwrap_or_else(|| s.len());
            Some(Authority::from(&s[authority_start + 2..authority_end]))
        } else {
            None
        };

        // Finally, we can parse the path, query parameters, and fragment from the remaining part of the URI string.
        let path_start = s.find('/').unwrap_or_else(|| s.len());
        let path_end = s[path_start..]
            .find(['?', '#'].as_ref())
            .map(|index| path_start + index)
            .unwrap_or_else(|| s.len());
        let path = s[path_start..path_end].to_string();

        let query_params = if let Some(query_start) = s.find('?') {
            let query_end = s[query_start..]
                .find('#')
                .map(|index| query_start + index)
                .unwrap_or_else(|| s.len());
            QueryParams::from_str(&s[query_start + 1..query_end]).unwrap()
        } else {
            QueryParams::new()
        };

        let fragment = s
            .find('#')
            .map(|fragment_start| s[fragment_start + 1..].to_string());

        Ok(AbsoluteUri {
            scheme,
            authority,
            path,
            query_params,
            fragment,
        })
    }
}

// -----
// TESTS
// -----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_absolute_uri_display() {
        let uri = AbsoluteUri {
            scheme: Scheme::Http,
            authority: Some(Authority {
                user_info: Some("user:pass".to_string()),
                host: "example.com".to_string(),
                port: Some(8080),
            }),
            path: "/path".to_string(),
            query_params: QueryParams::from(vec![("query".to_string(), "1".to_string())]),
            fragment: Some("fragment".to_string()),
        };
        assert_eq!(
            uri.to_string(),
            "http://user:pass@example.com:8080/path?query=1#fragment"
        );
    }
}
