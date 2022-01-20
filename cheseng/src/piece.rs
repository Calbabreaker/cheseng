use crate::Error;

/// Represents an index into a square in a 8x8 chessboard grid
#[derive(Copy, Clone)]
pub struct Square(u8);

impl Square {
    pub fn new(x: u8, y: u8) -> Self {
        Self(y * 8 + x)
    }
}

impl From<(u8, u8)> for Square {
    fn from((file, rank): (u8, u8)) -> Self {
        Self::new(file, rank)
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

    pub fn from_index(i: usize) -> Self {
        Self::new((i % 8) as u8, (i / 8) as u8)
    }

    pub fn as_index(&self) -> Result<usize, Error> {
        self.check_ouside_bounds()?;
        Ok(self.rank as usize * 8 + self.file as usize)
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

#[derive(Copy, Clone, PartialEq)]
pub enum Color {
    White,
    Black,
}

#[derive(Copy, Clone)]
pub enum Piece {
    Pawn(Color),
    Knight(Color),
    Bishop(Color),
    Rook(Color),
    Queen(Color),
    King(Color),
}

impl Piece {
    pub fn get_color(&self) -> &Color {
        use Piece::*;
        match self {
            Pawn(c) | Knight(c) | Bishop(c) | Rook(c) | Queen(c) | King(c) => c,
        }
    }
}
