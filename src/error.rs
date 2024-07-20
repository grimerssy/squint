use core::fmt;

#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    WrongTag,
    UnknownCharacter,
    Overflow,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WrongTag => f.write_str("id has an invalid tag"),
            Self::UnknownCharacter => f.write_str("id contains invalid characters"),
            Self::Overflow => f.write_str("id is too long"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}
