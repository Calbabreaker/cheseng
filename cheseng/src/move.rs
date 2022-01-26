use crate::{Error, Position};

#[derive(Debug, Clone, Copy)]
pub enum MoveFlag {
    None,
    EnPassantCapture,
    PawnDoublePush,
}

#[derive(Debug, Clone, Copy)]
pub struct Move {
    pub start_index: u8,
    pub end_index: u8,
    pub flag: MoveFlag,
}

impl Move {
    /// Creates a new move using start and end index
    /// For performance, start_square and end_square will not be checked if it's outside bounds
    pub fn new(start_index: u8, end_index: u8) -> Self {
        Self {
            start_index,
            end_index,
            flag: MoveFlag::None,
        }
    }

    pub fn flag(mut self, flag: MoveFlag) -> Self {
        self.flag = flag;
        self
    }
}

impl std::str::FromStr for Move {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (pos1, pos2) = s.split_at(2);
        Ok(Move::new(
            pos1.parse::<Position>()?.as_index()?,
            pos2.parse::<Position>()?.as_index()?,
        ))
    }
}
