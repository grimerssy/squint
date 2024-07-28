use core::fmt;

#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    WrongTag,
    UnknownCharacter,
    Overflow,
    WrongPadding,
    InvalidLength,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WrongTag => f.write_str("id has an invalid tag"),
            Self::UnknownCharacter => f.write_str("id contains invalid characters"),
            Self::Overflow => f.write_str("id is too long"),
            Self::WrongPadding => f.write_str("encoding padding is wrong"),
            Self::InvalidLength => f.write_str("id is of invalid length"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}
