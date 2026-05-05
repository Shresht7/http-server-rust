// --------
// FRAGMENT
// --------

/// The fragment is the final part of a URI that follows the `#` symbol.
/// It is used to identify  a specific part of the resource, such as a section of a document or a position in the window.
/// The fragment is not sent to the server as part of the HTTP request, it is entirely processed client side.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Fragment {
    /// An anchor is a reference to a specific section of a document, such as a heading or a paragraph.
    /// It behaves like a "bookmark" that allows users to jump directly to a specific part of the document.
    /// In an HTML document, the browser will scroll to the element into view.
    Anchor(String),

    /// A text fragment is a reference to a specific piece of text within a document.
    /// The browser highlights the text fragment in the document.
    Text(String),

    /// A media fragment is a reference to a specific part of a media resource, such as a video or audio file.
    /// It allows users to jump directly to a specific time or position in the media resource.
    Media(String),
}

// DISPLAY
// -------

impl std::fmt::Display for Fragment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Fragment::Anchor(anchor) => write!(f, "#{}", anchor),
            Fragment::Text(text) => write!(f, "#:~:text={}", text),
            Fragment::Media(media) => write!(f, "#t={}", media),
        }
    }
}

// FromStr
// -------

impl std::str::FromStr for Fragment {
    type Err = ParseFragmentError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("#:~:text=") {
            Ok(Fragment::Text(s[9..].to_string()))
        } else if s.starts_with("#t=") {
            Ok(Fragment::Media(s[3..].to_string()))
        } else if s.starts_with('#') {
            Ok(Fragment::Anchor(s[1..].to_string()))
        } else {
            Err(ParseFragmentError::InvalidFormat)
        }
    }
}

// -----
// ERROR
// -----

#[derive(Debug)]
pub enum ParseFragmentError {
    InvalidFormat,
}

impl std::fmt::Display for ParseFragmentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseFragmentError::InvalidFormat => write!(f, "Invalid fragment format"),
        }
    }
}

impl std::error::Error for ParseFragmentError {}

// -----
// TESTS
// -----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_anchor_fragment() {
        let s = "#section1";
        let fragment: Fragment = s.parse().unwrap();
        assert_eq!(fragment, Fragment::Anchor("section1".to_string()));
    }

    #[test]
    fn should_parse_text_fragment() {
        let s = "#:~:text=example";
        let fragment: Fragment = s.parse().unwrap();
        assert_eq!(fragment, Fragment::Text("example".to_string()));
    }

    #[test]
    fn should_parse_media_fragment() {
        let s = "#t=10,20";
        let fragment: Fragment = s.parse().unwrap();
        assert_eq!(fragment, Fragment::Media("10,20".to_string()));
    }

    #[test]
    fn should_fail_to_parse_invalid_fragment() {
        let s = "invalid_fragment";
        let result: Result<Fragment, _> = s.parse();
        assert!(result.is_err());
    }

    #[test]
    fn should_display_fragment_correctly() {
        let fragment = Fragment::Anchor("section1".to_string());
        assert_eq!(fragment.to_string(), "#section1");

        let fragment = Fragment::Text("example".to_string());
        assert_eq!(fragment.to_string(), "#:~:text=example");

        let fragment = Fragment::Media("10,20".to_string());
        assert_eq!(fragment.to_string(), "#t=10,20");
    }
}
