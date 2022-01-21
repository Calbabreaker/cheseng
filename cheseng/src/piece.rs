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
