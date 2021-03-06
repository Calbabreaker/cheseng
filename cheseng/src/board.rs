use crate::{Color, Error, Move, MoveFlag, Piece, Position, Side};

pub struct Board {
    pub turn: Color,
    pub grid: [Option<Piece>; 64],
    pub en_passant_square: Option<u8>,
    /// Array with the castle rights for a colour with white first, black second as 2 legth array
    /// of bools with queenside first kingside second
    pub castle_rights: [[bool; 2]; 2],
}

impl Board {
    pub const LETTERS: &'static str = "abcdefgh";

    pub fn empty() -> Self {
        Self {
            grid: [None; 64],
            turn: Color::White,
            en_passant_square: None,
            castle_rights: [[false, false]; 2],
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
                'P' => Piece::Pawn(Color::White),
                'N' => Piece::Knight(Color::White),
                'B' => Piece::Bishop(Color::White),
                'R' => Piece::Rook(Color::White),
                'Q' => Piece::Queen(Color::White),
                'K' => Piece::King(Color::White),

                'p' => Piece::Pawn(Color::Black),
                'n' => Piece::Knight(Color::Black),
                'b' => Piece::Bishop(Color::Black),
                'r' => Piece::Rook(Color::Black),
                'q' => Piece::Queen(Color::Black),
                'k' => Piece::King(Color::Black),

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

        let castle_rights_str = sections.next().ok_or(Error::InvalidFEN)?;
        if castle_rights_str != "-" {
            for char in castle_rights_str.chars() {
                match char {
                    'Q' => board.castle_rights[0][0] = true,
                    'K' => board.castle_rights[0][1] = true,
                    'k' => board.castle_rights[1][0] = true,
                    'q' => board.castle_rights[1][1] = true,
                    char => Err(Error::InvalidFENStr(char.into()))?,
                }
            }
        }

        let en_passant_char = sections.next().ok_or(Error::InvalidFEN)?;
        if en_passant_char != "-" {
            board.en_passant_square = Some(en_passant_char.parse::<Position>()?.as_index()?);
        }

        Ok(board)
    }

    /// Moves a using specified move's start and end square index.
    /// Will not check if the move is a legal move.
    pub fn make_move(&mut self, raw_move: Move) {
        let (start_i, end_i) = (raw_move.start_index as usize, raw_move.end_index as usize);
        let piece = self.grid[start_i];
        let end_piece = self.grid[end_i];

        self.grid[start_i] = None;
        self.grid[end_i] = piece;
        self.en_passant_square = None;

        macro_rules! set_rights {
            ($index: expr, $color: expr) => {
                let first_rank_index = if $color == Color::White { 56 } else { 0 };
                let rights = &mut self.castle_rights[$color.as_index()];
                if $index == first_rank_index {
                    rights[0] = false
                }
                if $index == first_rank_index + 7 {
                    rights[1] = false
                }
            };
        }

        if let Some(piece) = piece {
            let piece_color = *piece.get_color();
            let backward_index = match piece_color {
                Color::White => (end_i + 8),
                Color::Black => (end_i.saturating_sub(8)),
            };

            match raw_move.flag {
                MoveFlag::PawnDoublePush => {
                    self.en_passant_square = Some(backward_index as u8);
                }
                MoveFlag::EnPassantCapture => {
                    self.grid[backward_index] = None;
                }
                MoveFlag::Castle(side) => {
                    let first_rank_index = if piece_color == Color::White { 56 } else { 0 };
                    let (start_file, end_file) = match side {
                        Side::King => (7, 5),
                        Side::Queen => (0, 3),
                    };

                    // move rook
                    self.grid[first_rank_index + start_file] = None;
                    self.grid[first_rank_index + end_file] = Some(Piece::Rook(piece_color));
                }
                MoveFlag::Promote(piece) => {
                    println!("{}", piece);
                    self.grid[end_i] = Some(piece);
                }
                _ => (),
            }

            match piece {
                Piece::King(_) => self.castle_rights[piece_color.as_index()] = [false; 2],
                Piece::Rook(_) => {
                    set_rights!(start_i, piece_color);
                }
                _ => (),
            }
        }

        match end_piece {
            Some(Piece::Rook(color)) => {
                set_rights!(end_i, color);
            }
            _ => (),
        }

        // change turns
        self.turn = match self.turn {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };
    }

    pub fn move_is_capture(&self, test_move: Move) -> bool {
        self.grid[test_move.end_index as usize].is_some()
            || self.en_passant_square == Some(test_move.end_index)
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

    /// Tests if the move is legal and return it with the neccessery flags set (en passant, double push, etc.)
    /// else it will return none
    pub fn as_legal_move(&self, test_move: Move) -> Option<Move> {
        if let Some(piece) = self.grid[test_move.start_index as usize] {
            let legal_moves = piece.get_legal_moves(test_move.start_index, &self);
            let legal_move = legal_moves
                .iter()
                .find(|legal_move| legal_move.end_index == test_move.end_index);

            legal_move.copied()
        } else {
            None
        }
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap()
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for (index, piece) in self.grid.iter().enumerate() {
            if index == 0 {
                write!(f, "   ???????????????????????????????????????????????????????????????????????????????????????????????????\n 8 ???")?;
            } else if index % 8 == 0 {
                write!(
                    f,
                    "\n   ???????????????????????????????????????????????????????????????????????????????????????????????????\n {} ???",
                    8 - index / 8
                )?;
            }

            let char = piece.map_or(' ', |p| p.get_char());
            write!(f, " {} ???", char)?;
        }

        write!(f, "\n   ???????????????????????????????????????????????????????????????????????????????????????????????????\n   ")?;

        for letter in Board::LETTERS.chars() {
            write!(f, "  {} ", letter)?;
        }

        Ok(())
    }
}
