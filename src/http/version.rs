// ------------
// HTTP VERSION
// ------------

/// Defines the HTTP protocol version used in the request line of [`Request`][crate::http::Request]
/// and the status line of [`Response`][crate::http::Response] structs.
///
/// See: https://www.rfc-editor.org/rfc/rfc1945#section-3.1
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Version(u32, u32);

impl Default for Version {
    fn default() -> Self {
        Self(0, 9) // If the protocol version is not specified, the recepient must assume that the message is in the HTTP/0.9 format.
    }
}

impl From<(u32, u32)> for Version {
    fn from(value: (u32, u32)) -> Self {
        Self(value.0, value.1)
    }
}

impl TryFrom<f64> for Version {
    type Error = ParseVersionError;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        let s = format!("HTTP/{}", value);
        s.parse::<Version>()
    }
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HTTP/{}.{}", self.0, self.1)
    }
}

impl std::str::FromStr for Version {
    type Err = ParseVersionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // If the version string can be parsed as a float, we can convert it to a Version using the TryFrom implementation
        if let Ok(value) = s.parse::<f64>() {
            return Self::try_from(value);
        }

        if !s.starts_with("HTTP/") {
            return Err(ParseVersionError::InvalidPrefix(s.to_string()));
        }

        let version_part = &s[5..]; // Remove the "HTTP/" prefix

        // Split the version part into major and minor components
        let parts: Vec<&str> = version_part.split('.').collect();

        if parts.len() != 2 {
            return Err(ParseVersionError::NumberingFormat(s.to_string()));
        }

        let major = parts[0]
            .parse::<u32>()
            .map_err(|_| ParseVersionError::MajorVersion(s.to_string()))?;

        let minor = parts[1]
            .parse::<u32>()
            .map_err(|_| ParseVersionError::MinorVersion(s.to_string()))?;

        Ok(Self(major, minor))
    }
}

// ------
// ERRORS
// ------

/// Represents errors that can occur while parsing an HTTP version string.
/// This includes errors such as an invalid prefix, incorrect numbering format, or invalid major/minor version numbers.
#[derive(Debug)]
pub enum ParseVersionError {
    /// The version string does not start with the expected "HTTP/" prefix.
    InvalidPrefix(String),
    /// The version string does not follow the expected "<major>.<minor>" format.
    NumberingFormat(String),
    /// The major version number is invalid.
    MajorVersion(String),
    /// The minor version number is invalid.
    MinorVersion(String),
}

impl std::fmt::Display for ParseVersionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseVersionError::InvalidPrefix(s) => {
                write!(f, "Invalid HTTP version prefix: '{}'. Should be 'HTTP/'", s)
            }
            ParseVersionError::NumberingFormat(s) => {
                write!(
                    f,
                    "Invalid HTTP version numbering format (<major>.<minor>): '{}'",
                    s
                )
            }
            ParseVersionError::MajorVersion(s) => {
                write!(f, "Invalid major version number in HTTP version: '{}'", s)
            }
            ParseVersionError::MinorVersion(s) => {
                write!(f, "Invalid minor version number in HTTP version: '{}'", s)
            }
        }
    }
}

impl std::error::Error for ParseVersionError {}

// -----
// TESTS
// -----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_correct_version() {
        let version_str = "HTTP/1.1";
        let version = version_str.parse::<Version>().unwrap();
        assert_eq!(version.to_string(), version_str);
    }

    #[test]
    fn should_fail_on_invalid_prefix() {
        let version_str = "HTP/1.1";
        let result = version_str.parse::<Version>();
        assert!(result.is_err());
        assert!(matches!(
            result.err(),
            Some(ParseVersionError::InvalidPrefix(_))
        ));
    }

    #[test]
    fn should_fail_on_invalid_numbering_format() {
        let version_str = "HTTP/1";
        let result = version_str.parse::<Version>();
        assert!(result.is_err());
        assert!(matches!(
            result.err(),
            Some(ParseVersionError::NumberingFormat(_))
        ));
    }

    #[test]
    fn should_fail_on_more_than_two_numbering_parts() {
        let version_str = "HTTP/1.1.1";
        let result = version_str.parse::<Version>();
        assert!(result.is_err());
        assert!(matches!(
            result.err(),
            Some(ParseVersionError::NumberingFormat(_))
        ));
    }

    #[test]
    fn should_fail_on_invalid_major_version() {
        let version_str = "HTTP/a.1";
        let result = version_str.parse::<Version>();
        assert!(result.is_err());
        assert!(matches!(
            result.err(),
            Some(ParseVersionError::MajorVersion(_))
        ));
    }

    #[test]
    fn should_fail_on_invalid_minor_version() {
        let version_str = "HTTP/1.b";
        let result = version_str.parse::<Version>();
        assert!(result.is_err());
        assert!(matches!(
            result.err(),
            Some(ParseVersionError::MinorVersion(_))
        ));
    }

    #[test]
    fn should_pass_valid_test_cases() {
        let valid_versions = vec!["HTTP/0.9", "HTTP/1.0", "HTTP/1.1", "HTTP/2.0"];
        for version_str in valid_versions {
            let version = version_str.parse::<Version>();
            assert!(version.is_ok());
            assert_eq!(version.unwrap().to_string(), version_str);
        }
    }

    #[test]
    fn should_pass_default_version() {
        let default_version = Version::default();
        assert_eq!(default_version.to_string(), "HTTP/0.9");
    }

    #[test]
    fn should_convert_from_tuple() {
        let version_tuple = (1, 1);
        let version: Version = version_tuple.into();
        assert_eq!(version.to_string(), "HTTP/1.1");
    }

    #[test]
    fn should_convert_from_f64() {
        let version_f64 = 1.1;
        let version: Version = version_f64.try_into().unwrap();
        assert_eq!(version.to_string(), "HTTP/1.1");
    }

    #[test]
    fn should_convert_from_str_with_float() {
        let version_str = "1.1";
        let version: Version = version_str.parse().unwrap();
        assert_eq!(version.to_string(), "HTTP/1.1");
    }
}
