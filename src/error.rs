use core::fmt;

#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    TagMismatch,
    InvalidFormat,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TagMismatch => f.write_str("id has an unexpected tag"),
            Self::InvalidFormat => f.write_str("id did not match the expected format"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}
