mod absolute;
mod relative;

mod authority;
mod errors;
mod query_params;
mod scheme;

pub use absolute::AbsoluteUri;
pub use relative::RelativeUri;

pub use errors::*;

// ---
// URI
// ---

/// A URI (Uniform Resource Identifier) is a unique sequence of characters that identifies a resource on the internet.
/// It is a fundamental component of the web and is used to locate resources such as web pages, images, videos, and APIs.
/// A URI can either be an [`AbsoluteUri`] or a [`RelativeUri`], depending on whether it is relative to a base URI or not.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Uri {
    /// An absolute URI is a full URI that includes all components (scheme, authority, path, query parameters, and fragment).
    Absolute(AbsoluteUri),

    /// A relative URI is a URI that is relative to a base URI.
    /// It typically includes only the path, query parameters, and fragment, and does not include the scheme or authority.
    Relative(RelativeUri),
}

// DISPLAY
// --------

impl std::fmt::Display for Uri {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

// FromStr
// -------

impl std::str::FromStr for Uri {
    type Err = ParseUriError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains("://") {
            Ok(Uri::Absolute(s.parse()?))
        } else {
            Ok(Uri::Relative(s.parse()?))
        }
    }
}

// -----
// TESTS
// -----

#[cfg(test)]
mod tests {
    use super::*;
    use crate::uri::absolute::AbsoluteUri;
    use crate::uri::authority::Authority;
    use crate::uri::query_params::QueryParams;
    use crate::uri::scheme::Scheme;

    #[test]
    fn should_parse_uri() {
        let s = "https://user:pass@host:8080/path?query=value#fragment";
        let uri: Uri = s.parse().unwrap();
        assert_eq!(
            uri,
            Uri::Absolute(AbsoluteUri {
                scheme: Scheme::Https,
                authority: Some(Authority {
                    user_info: Some("user:pass".to_string()),
                    host: "host".to_string(),
                    port: Some(8080),
                }),
                path: "/path".to_string(),
                query_params: QueryParams::from(vec![("query".to_string(), "value".to_string())]),
                fragment: Some("fragment".to_string()),
            })
        );

        let s = "foo://example.com:8042/over/there?name=ferret#nose";
        let uri: Uri = s.parse().unwrap();
        assert_eq!(
            uri,
            Uri::Absolute(AbsoluteUri {
                scheme: Scheme::Other("foo".to_string()),
                authority: Some(Authority {
                    user_info: None,
                    host: "example.com".to_string(),
                    port: Some(8042),
                }),
                path: "/over/there".to_string(),
                query_params: QueryParams::from(vec![("name".to_string(), "ferret".to_string())]),
                fragment: Some("nose".to_string()),
            })
        );
    }
}
