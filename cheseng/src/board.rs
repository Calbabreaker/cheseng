use crate::{Color, Error, Piece, Position};

pub struct Board {
    pub grid: [Option<Piece>; 64],
    pub turn: Color,
}

impl Board {
    pub fn empty() -> Self {
        Self {
            grid: [None; 64],
            turn: Color::White,
        }
    }

    /// Creates a new board from Forsyth-Edwards Notation
    /// TODO: parse the remaining information
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

            board.grid[pos.as_index()?] = Some(piece);
            pos.file += 1;
        }

        board.turn = match sections.next().ok_or(Error::InvalidFEN)? {
            "w" => Color::White,
            "b" => Color::Black,
            turn_str => Err(Error::InvalidFENStr(turn_str.into()))?,
        };

        Ok(board)
    }

    /// Moves a from old_pos piece to new_pos
    /// Will set the piece back to old_pos if it's not a legal move
    /// Uses [is_legal_move]
    pub fn move_piece(&mut self, old_pos: Position, new_pos: Position, piece: Piece) {
        if let Ok(i) = self.check_legal_move(old_pos, new_pos, piece) {
            self.grid[i] = Some(piece);
            self.turn = if self.turn == Color::White {
                Color::Black
            } else {
                Color::White
            }
        } else if let Ok(i) = old_pos.as_index() {
            self.grid[i] = Some(piece);
        }
    }

    /// Takes a piece from the board at position and returns it
    pub fn take_piece(&mut self, position: Position) -> Option<Piece> {
        let i = position.as_index().ok()?;
        let piece = self.grid[i];
        self.grid[i] = None;
        piece
    }

    pub fn check_legal_move(
        &mut self,
        old_pos: Position,
        new_pos: Position,
        piece: Piece,
    ) -> Result<usize, Error> {
        let color = *piece.get_color();
        if self.turn != color {
            Err(Error::WrongTurn(color))?
        }

        let i = new_pos.as_index()?;
        Ok(i)
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap()
    }
}
