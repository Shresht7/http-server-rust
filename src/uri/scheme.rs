/// The [`Scheme`] is the first part of a Uniform Resource Identifier (URI) that indicates the protocol used to access the resource.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Scheme {
    /// The `http` scheme is used for accessing resources over the Hypertext Transfer Protocol (HTTP).
    /// It is the most common scheme for web resources.
    Http,

    /// The `https` scheme is used for accessing resources over HTTP Secure (HTTPS),
    /// which is an extension of HTTP that provides encryption and secure communication.
    /// It is commonly used for secure websites and online transactions.
    Https,

    /// The `ftp` scheme is used for accessing resources over the File Transfer Protocol (FTP).
    /// It is commonly used for transferring files between a client and a server.
    Ftp,

    /// The `mailto` scheme is used for email addresses. It allows users to create
    /// hyperlinks that open the default email client with a pre-filled recipient address.
    /// For example, `mailto:example@example.com` would open the default email client with the "To" field set to "example@example.com".
    Mailto,

    /// The `file` scheme is used for accessing local files on a computer.
    /// It allows users to create hyperlinks that point to files on their local file system.
    /// For example, `file:///C:/path/to/file.txt` would point to a file located at `C:\path\to\file.txt` on a Windows system.
    /// Note the extra `/` after the scheme name, which indicates that the path is an absolute path on the local file system.
    File,

    /// The `data` scheme is used for embedding small data items directly in a URI.
    /// It allows users to include data such as images or text directly in the URI itself, rather than referencing an external resource.
    /// For example, `data:text/plain;base64,SGVsbG8sIFdvcmxkIQ==` would represent the text "Hello, World!" encoded in Base64 format.
    Data,

    /// The `javascript` scheme is used for executing JavaScript code directly from a URI.
    /// It allows users to create hyperlinks that execute JavaScript code when clicked.
    /// For example, `javascript:alert('Hello, World!')` would display an alert box with the message "Hello, World!" when clicked.
    JavaScript,

    /// The `tel` scheme is used for telephone numbers.
    /// It allows users to create hyperlinks that initiate a phone call when clicked.
    /// For example, `tel:+1234567890` would initiate a phone call to the number "+1234567890" when clicked.
    /// Note that the `tel` scheme is typically used on mobile devices that have telephony capabilities, and may not work on desktop browsers.
    Tel,

    /// The `blob` scheme is used for representing binary data as a URI.
    /// It allows users to create hyperlinks that point to binary data, such as images or files, that are stored in memory or on disk.
    Blob,

    /// The `ssh` scheme is used for accessing resources over the Secure Shell (SSH) protocol.
    /// It is commonly used for remote server administration and secure file transfers.
    Ssh,

    /// The `urn` scheme is used for Uniform Resource Names (URNs), which are a type of URI that identifies a resource by name rather than by location.
    /// URNs are typically used for identifying resources in a persistent and location-independent manner.
    Urn,

    /// The `view-source` scheme is used for viewing the source code of a web page.
    /// It allows users to create hyperlinks that open the source code of a web page in the browser.
    ViewSource,

    /// The `ws` scheme is used for WebSocket connections, which are a protocol for full-duplex communication over a single TCP connection.
    Ws,

    /// The `wss` scheme is used for secure WebSocket connections, which are encrypted using TLS (Transport Layer Security).
    Wss,

    /// Any scheme that is not explicitly listed above can be represented using the `Other` variant, which holds a string value representing the scheme name.
    Other(String),
}

impl Scheme {
    /// Parses a URI string and extracts the scheme component, if present. The scheme is the part of the URI that appears before the first colon (`:`).
    pub fn from_url(s: &str) -> Option<Self> {
        s.split(':').next().map(|scheme| Scheme::from(scheme))
    }
}

// From
// ----

impl<T: AsRef<str>> From<T> for Scheme {
    fn from(s: T) -> Self {
        match s.as_ref() {
            "http" => Scheme::Http,
            "https" => Scheme::Https,
            "ftp" => Scheme::Ftp,
            "mailto" => Scheme::Mailto,
            "file" => Scheme::File,
            "data" => Scheme::Data,
            "javascript" => Scheme::JavaScript,
            "tel" => Scheme::Tel,
            "blob" => Scheme::Blob,
            "ssh" => Scheme::Ssh,
            "urn" => Scheme::Urn,
            "view-source" => Scheme::ViewSource,
            "ws" => Scheme::Ws,
            "wss" => Scheme::Wss,
            other => Scheme::Other(other.to_string()),
        }
    }
}

// DISPLAY
// -------

impl std::fmt::Display for Scheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Scheme::Http => write!(f, "http"),
            Scheme::Https => write!(f, "https"),
            Scheme::Ftp => write!(f, "ftp"),
            Scheme::Mailto => write!(f, "mailto"),
            Scheme::File => write!(f, "file"),
            Scheme::Data => write!(f, "data"),
            Scheme::JavaScript => write!(f, "javascript"),
            Scheme::Tel => write!(f, "tel"),
            Scheme::Blob => write!(f, "blob"),
            Scheme::Ssh => write!(f, "ssh"),
            Scheme::Urn => write!(f, "urn"),
            Scheme::ViewSource => write!(f, "view-source"),
            Scheme::Ws => write!(f, "ws"),
            Scheme::Wss => write!(f, "wss"),
            Scheme::Other(other) => write!(f, "{}", other),
        }
    }
}

// -----
// TESTS
// -----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_from_string() {
        assert_eq!(Scheme::from("http"), Scheme::Http);
        assert_eq!(Scheme::from("https"), Scheme::Https);
        assert_eq!(Scheme::from("ftp"), Scheme::Ftp);
        assert_eq!(Scheme::from("mailto"), Scheme::Mailto);
        assert_eq!(Scheme::from("file"), Scheme::File);
        assert_eq!(Scheme::from("data"), Scheme::Data);
        assert_eq!(Scheme::from("javascript"), Scheme::JavaScript);
        assert_eq!(Scheme::from("tel"), Scheme::Tel);
        assert_eq!(Scheme::from("blob"), Scheme::Blob);
        assert_eq!(Scheme::from("ssh"), Scheme::Ssh);
        assert_eq!(Scheme::from("urn"), Scheme::Urn);
        assert_eq!(Scheme::from("view-source"), Scheme::ViewSource);
        assert_eq!(Scheme::from("ws"), Scheme::Ws);
        assert_eq!(Scheme::from("wss"), Scheme::Wss);
        assert_eq!(
            Scheme::from("custom-scheme"),
            Scheme::Other("custom-scheme".to_string())
        );
    }

    #[test]
    fn should_format_to_string() {
        assert_eq!(Scheme::Http.to_string(), "http");
        assert_eq!(Scheme::Https.to_string(), "https");
        assert_eq!(Scheme::Ftp.to_string(), "ftp");
        assert_eq!(Scheme::Mailto.to_string(), "mailto");
        assert_eq!(Scheme::File.to_string(), "file");
        assert_eq!(Scheme::Data.to_string(), "data");
        assert_eq!(Scheme::JavaScript.to_string(), "javascript");
        assert_eq!(Scheme::Tel.to_string(), "tel");
        assert_eq!(Scheme::Blob.to_string(), "blob");
        assert_eq!(Scheme::Ssh.to_string(), "ssh");
        assert_eq!(Scheme::Urn.to_string(), "urn");
        assert_eq!(Scheme::ViewSource.to_string(), "view-source");
        assert_eq!(Scheme::Ws.to_string(), "ws");
        assert_eq!(Scheme::Wss.to_string(), "wss");
        assert_eq!(
            Scheme::Other("custom-scheme".to_string()).to_string(),
            "custom-scheme"
        );
    }

    #[test]
    fn should_parse_from_url() {
        assert_eq!(
            Scheme::from_url("https://example.com/path?query=param#fragment"),
            Some(Scheme::Https)
        );
        assert_eq!(
            Scheme::from_url("ftp://example.com/resource"),
            Some(Scheme::Ftp)
        );
        assert_eq!(
            Scheme::from_url("wss://example.com/socket"),
            Some(Scheme::Wss)
        );
        assert_eq!(
            Scheme::from_url("custom-scheme://example.com"),
            Some(Scheme::Other("custom-scheme".to_string()))
        );
        assert_eq!(
            Scheme::from_url("no-scheme"),
            Some(Scheme::Other("no-scheme".to_string()))
        );
    }
}
