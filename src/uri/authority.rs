///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Authority {
    /// The user information specified in the authority, if present.
    /// This typically includes a username and, optionally, a password, separated by a colon (':').
    /// This is optional and may be `None` if no user information is specified.
    pub user_info: Option<String>,

    /// The host component of the authority, which can be a domain name, an IPv4 address, or an IPv6 address.
    pub host: String,

    /// The port number specified in the authority, if present.
    /// Indicates the port on which the server is listening on for incoming requests.
    /// This is optional and may be `None` if no port is specified.
    pub port: Option<u16>,
}

// FROM
// ----

impl<T: AsRef<str>> From<T> for Authority {
    fn from(s: T) -> Self {
        let s = s.as_ref();
        let mut user_info = None;
        let mut host_port = s;

        // Check for the presence of user information (indicated by '@') and extract it if present
        if let Some(at_index) = s.find('@') {
            user_info = Some(s[..at_index].to_string());
            host_port = &s[at_index + 1..];
        }

        // Check for the presence of a port number (indicated by ':') and extract it if present
        let (host, port) = if let Some(colon_index) = host_port.rfind(':') {
            let host_part = &host_port[..colon_index];
            let port_part = &host_port[colon_index + 1..];
            let port_number = port_part.parse::<u16>().ok();
            (host_part.to_string(), port_number)
        } else {
            (host_port.to_string(), None)
        };

        Authority {
            user_info,
            host,
            port,
        }
    }
}

// DISPLAY
// -------

impl std::fmt::Display for Authority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(user_info) = &self.user_info {
            write!(f, "{}@", user_info)?;
        }
        write!(f, "{}", self.host)?;
        if let Some(port) = self.port {
            write!(f, ":{}", port)?;
        }
        Ok(())
    }
}

// -----
// TESTS
// -----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_authority() {
        let s = "user:pass@host:8080";
        let authority: Authority = s.into();
        assert_eq!(
            authority,
            Authority {
                user_info: Some("user:pass".to_string()),
                host: "host".to_string(),
                port: Some(8080),
            }
        );

        let s = "user:pass@host.com";
        let authority: Authority = s.into();
        assert_eq!(
            authority,
            Authority {
                user_info: Some("user:pass".to_string()),
                host: "host.com".to_string(),
                port: None,
            }
        );

        let s = "host.com:8080";
        let authority: Authority = s.into();
        assert_eq!(
            authority,
            Authority {
                user_info: None,
                host: "host.com".to_string(),
                port: Some(8080),
            }
        );

        let s = "example.com";
        let authority: Authority = s.into();
        assert_eq!(
            authority,
            Authority {
                user_info: None,
                host: "example.com".to_string(),
                port: None,
            }
        );

        let s = "locahost:3000";
        let authority: Authority = s.into();
        assert_eq!(
            authority,
            Authority {
                user_info: None,
                host: "locahost".to_string(),
                port: Some(3000),
            }
        );
    }

    #[test]
    fn should_display_authority() {
        let authority = Authority {
            user_info: Some("user:pass".to_string()),
            host: "host.com".to_string(),
            port: Some(8080),
        };
        assert_eq!(authority.to_string(), "user:pass@host.com:8080");
    }
}
