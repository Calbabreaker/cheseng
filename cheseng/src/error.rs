use crate::Color;

pub enum Error {
    InvalidFEN,
    InvalidFENStr(String),
    OutsideBounds(crate::Position),
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::InvalidFEN => {
                write!(f, "InvalidFEN: Invalid number of sections or subsections")
            }
            Self::InvalidFENStr(string) => {
                write!(f, "InvalidFEN: Unexpected string: {}", string)
            }
            Self::OutsideBounds(position) => write!(
                f,
                "OustideBounds: {:?} is ousisde 8x8 chess board",
                position
            ),
        }
    }
}
