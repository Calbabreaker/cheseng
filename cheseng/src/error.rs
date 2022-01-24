pub enum Error {
    InvalidFEN,
    InvalidFENStr(String),
    OutsideBounds(crate::Position),
    InvalidPosition(Option<char>),
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
            Self::InvalidPosition(Some(char)) => {
                write!(f, "InvalidPosition: Unexpected char: {}", char)
            }
            Self::InvalidPosition(None) => {
                write!(f, "InvalidPosition: Expected 2 chars")
            }
        }
    }
}
