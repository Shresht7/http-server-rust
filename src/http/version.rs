// ------------
// HTTP VERSION
// ------------

/// Defines the HTTP protocol version used in the status line of [`Request`] and [`Response`] structs.
///
/// See: https://www.rfc-editor.org/rfc/rfc1945#section-3.1
pub struct Version(u32, u32);

impl Default for Version {
    fn default() -> Self {
        Self(0, 9) // If the protocol version is not specified, the recepient must assume that the message is in the HTTP/0.9 format.
    }
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HTTP/{}.{}", self.0, self.1)
    }
}
