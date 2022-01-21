use crate::{Color, Error, Move, Piece, Position};

pub struct Board {
    pub turn: Color,
    pub grid: [Option<Piece>; 64],
}

impl Board {
    pub fn empty() -> Self {
        Self {
            grid: [None; 64],
            turn: Color::White,
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

        Ok(board)
    }

    /// Moves a using specified move's start and end square index.
    /// Will handle promotion, castling, etc. but will not check if its a legal move.
    pub fn make_move(&mut self, move_: Move) {
        let (si, ei) = (move_.start_index as usize, move_.end_index as usize);
        let piece = self.grid[si];
        self.grid[si] = None;
        self.grid[ei] = piece;
    }

    pub fn generate_all_moves(&self) -> Vec<Move> {
        let mut moves = Vec::new();
        for (i, piece) in self.grid.iter().enumerate() {
            if let Some(piece) = piece {
                self.add_moves_for_piece(&mut moves, piece, i as u8);
            }
        }

        moves
    }

    pub fn generate_moves_for_piece(&self, piece: &Piece, index: u8) -> Vec<Move> {
        let mut moves = Vec::new();
        self.add_moves_for_piece(&mut moves, &piece, index);
        moves
    }

    fn add_moves_for_piece(&self, moves: &mut Vec<Move>, piece: &Piece, index: u8) {
        if *piece.get_color() != self.turn {
            return;
        }

        moves.push(Move::new(index, index + 1));
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap()
    }
}
