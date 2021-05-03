//! APIs for interacting specifically with Fedora CoreOS

use std::fmt::Display;

/// Base URL to Fedora CoreOS streams metadata.
pub const STREAM_BASE_URL: &str = "https://builds.coreos.fedoraproject.org/streams/";

/// Well-known streams for Fedora CoreOS.
///
/// For more information, see https://docs.fedoraproject.org/en-US/fedora-coreos/update-streams/
pub enum StreamID {
    /// The stable stream.
    Stable,
    /// The testing stream.
    Testing,
    /// The next stream.
    Next,
}

impl Display for StreamID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            StreamID::Stable => "stable",
            StreamID::Testing => "testing",
            StreamID::Next => "next",
        })
    }
}

impl StreamID {
    /// Return the URL for this stream.
    pub fn url(&self) -> String {
        format!("{}{}.json", STREAM_BASE_URL, self)
    }
}
