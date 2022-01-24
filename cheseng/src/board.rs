use crate::{Color, Error, Move, Piece, Position};

pub struct Board {
    pub turn: Color,
    pub grid: [Option<Piece>; 64],
    pub en_passant_square: Option<u8>,
}

impl Board {
    pub fn empty() -> Self {
        Self {
            grid: [None; 64],
            turn: Color::White,
            en_passant_square: None,
        }
    }

    /// Creates a new board from Forsyth-Edwards Notation.
    // TODO: parse the remaining information
    pub fn from_fen(fen: &str) -> Result<Self, Error> {
        let mut board = Self::empty();
        let mut sections = fen.split_whitespace();

        let mut pos = Position::new(0, 0);
        for char in sections.next().ok_or(Error::InvalidFEN)?.chars() {
            let piece = match char {
                'p' => Piece::Pawn(Color::Black),
                'n' => Piece::Knight(Color::Black),
                'b' => Piece::Bishop(Color::Black),
                'r' => Piece::Rook(Color::Black),
                'q' => Piece::Queen(Color::Black),
                'k' => Piece::King(Color::Black),

                'P' => Piece::Pawn(Color::White),
                'N' => Piece::Knight(Color::White),
                'B' => Piece::Bishop(Color::White),
                'R' => Piece::Rook(Color::White),
                'Q' => Piece::Queen(Color::White),
                'K' => Piece::King(Color::White),

                '/' => {
                    pos.rank += 1;
                    pos.file = 0;
                    continue;
                }

                '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    pos.file += char.to_digit(10).unwrap() as u8;
                    continue;
                }

                _ => Err(Error::InvalidFENStr(char.into()))?,
            };

            board.grid[pos.as_index()? as usize] = Some(piece);
            pos.file += 1;
        }

        board.turn = match sections.next().ok_or(Error::InvalidFEN)? {
            "w" => Color::White,
            "b" => Color::Black,
            turn_str => Err(Error::InvalidFENStr(turn_str.into()))?,
        };

        // TODO: castling oppotunities sections
        sections.next();

        let en_passant_char = sections.next().ok_or(Error::InvalidFEN)?;
        if en_passant_char != "-" {
            board.en_passant_square = Some(en_passant_char.parse::<Position>()?.as_index()?);
        }

        Ok(board)
    }

    /// Moves a using specified move's start and end square index.
    /// Will handle promotion, castling, etc. but will not check if its a legal move.
    pub fn make_move(&mut self, move_: Move) {
        let (start_i, end_i) = (move_.start_index as usize, move_.end_index as usize);
        let piece = self.grid[start_i];
        self.grid[start_i] = None;
        self.grid[end_i] = piece;

        // change turns
        self.turn = match self.turn {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };

        match piece {
            Some(Piece::Pawn(color)) => {
                let (sign, first_rank) = match color {
                    Color::White => (-1, 6),
                    Color::Black => (1, 1),
                };

                let (start_i, end_i) = (move_.start_index as i8, move_.end_index as i8);
                let (start_rank, end_rank) = (start_i / 8, end_i / 8);
                let backward_index = end_i + 8 * -sign;
                if Some(move_.end_index) == self.en_passant_square {
                    self.grid[backward_index as usize] = None;
                } else if start_rank == first_rank && end_rank == first_rank + 2 * sign {
                    self.en_passant_square = Some(backward_index as u8);
                    return;
                }
            }
            _ => (),
        }

        self.en_passant_square = None;
    }

    pub fn move_is_capture(&self, move_: &Move) -> bool {
        self.grid[move_.end_index as usize].is_some()
            || self.en_passant_square == Some(move_.end_index)
    }

    pub fn get_all_legal_moves(&self) -> Vec<Move> {
        let mut moves = Vec::new();
        for (i, piece) in self.grid.iter().enumerate() {
            if let Some(piece) = piece {
                piece.add_legal_moves(i as u8, &mut moves, &self);
            }
        }

        moves
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap()
    }
}

// TODO: Implement display fmt trait for board

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_start_en_passant() {
        let mut board = Board::from_fen("8/8/8/pP6/8/8/8/8 w ---- a6 0 1").unwrap();
        assert_eq!(board.en_passant_square, Some(16));

        board.make_move(Move::new(25, 16)); // do en passant
        assert_eq!(board.grid[24], None);
    }

    #[test]
    fn test_move_into_en_passant() {
        let mut board = Board::from_fen("8/pp6/8/1P6/8/8/8/8 w ---- - 0 1").unwrap();
        board.make_move(Move::new(8, 24)); // do double push
        assert_eq!(board.grid[24], Some(Piece::Pawn(Color::Black)));
        assert_eq!(board.en_passant_square, Some(16));

        board.make_move(Move::new(25, 16)); // do en passant
        assert_eq!(board.grid[24], None);

        board.make_move(Move::new(9, 17)); // singal push
        board.make_move(Move::new(16, 9)); // try take
        assert_eq!(board.grid[17], Some(Piece::Pawn(Color::Black)));
    }
}
