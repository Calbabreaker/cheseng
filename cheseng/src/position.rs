use crate::Error;

pub struct Move {
    pub start_index: u8,
    pub end_index: u8,
}

impl Move {
    /// Creates a new move using start and end index
    /// For performance, start_square and end_square will not be checked if it's outside bounds
    pub fn new(start_index: u8, end_index: u8) -> Self {
        Self {
            start_index,
            end_index,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Position {
    pub file: u8,
    pub rank: u8,
}

impl Position {
    pub fn new(file: u8, rank: u8) -> Self {
        Self { file, rank }
    }

    pub fn from_index(i: u8) -> Self {
        Self::new(i % 8, i / 8)
    }

    pub fn as_index(&self) -> Result<u8, Error> {
        self.check_ouside_bounds()?;
        Ok(self.rank * 8 + self.file)
    }

    pub fn check_ouside_bounds(&self) -> Result<(), Error> {
        if self.file > 7 || self.rank > 7 {
            Err(Error::OutsideBounds(*self))
        } else {
            Ok(())
        }
    }
}

impl From<(u8, u8)> for Position {
    fn from((row, col): (u8, u8)) -> Self {
        Self::new(row, col)
    }
}

impl From<u8> for Position {
    fn from(index: u8) -> Self {
        Self::from_index(index)
    }
}
