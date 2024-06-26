use core::fmt;

#[derive(Debug)]
pub enum Error {
    WrongTag,
    UnknownCharacter,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WrongTag => f.write_str("id has an invalid tag"),
            Self::UnknownCharacter => f.write_str("id contains invalid characters"),
        }
    }
}
