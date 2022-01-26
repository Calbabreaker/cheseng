use crate::{Board, Move, MoveFlag, Side};

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn as_index(&self) -> usize {
        match self {
            Self::White => 0,
            Self::Black => 1,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
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

    pub fn get_legal_moves(&self, piece_index: u8, board: &Board) -> Vec<Move> {
        let mut moves = Vec::new();
        self.add_legal_moves(piece_index, &mut moves, board);
        moves
    }

    pub fn add_legal_moves(&self, piece_index: u8, moves: &mut Vec<Move>, board: &Board) {
        let piece_color = *self.get_color();
        if piece_color != board.turn {
            return;
        }

        macro_rules! handle_sliding_piece {
            ($start: expr, $end: expr) => {
                handle_sliding_piece(moves, piece_index, piece_color, &board, $start, $end)
            };
        }

        let grid_index = piece_index as usize;
        match self {
            Piece::Queen(_) => handle_sliding_piece!(0, 8),
            Piece::Rook(_) => handle_sliding_piece!(0, 4),
            Piece::Bishop(_) => handle_sliding_piece!(4, 8),
            Piece::King(_) => {
                for (dir_index, offset) in DIRECTION_OFFSETS.iter().enumerate() {
                    // ensure not ouside bounds
                    if NUM_TIMES_TO_EDGE[grid_index][dir_index] == 0 {
                        continue;
                    }

                    let end_index = piece_index as i8 + offset;
                    if let Some(end_piece) = board.grid[end_index as usize] {
                        // friendly piece so don't add move
                        if *end_piece.get_color() == piece_color {
                            continue;
                        }
                    }

                    // check not ouside bounds
                    moves.push(Move::new(piece_index, end_index as u8));
                }

                // check castling
                let first_rank_index = match piece_color {
                    Color::White => 56,
                    Color::Black => 0,
                };

                macro_rules! check_castle {
                    ($rook_file: expr, $end_file: expr, $side: expr) => {
                        if board.castle_rights[piece_color.as_index()][$side.as_index()] {
                            moves.push(
                                Move::new(piece_index, first_rank_index as u8 + $end_file)
                                    .flag(MoveFlag::Castle($side)),
                            );
                        }
                    };
                }

                check_castle!(0, 2, Side::Queen);
                check_castle!(7, 6, Side::King);
            }
            Piece::Knight(_) => {
                for offset in &KNIGHT_MOVES[grid_index] {
                    let end_index = piece_index as i8 + offset;
                    if let Some(end_piece) = board.grid[end_index as usize] {
                        // friendly piece so don't add move
                        if *end_piece.get_color() == piece_color {
                            continue;
                        }
                    }

                    moves.push(Move::new(piece_index, end_index as u8));
                }
            }
            Piece::Pawn(_) => {
                let (forward_offset, second_rank, last_rank, attack_dir_iter) = match piece_color {
                    Color::White => (-8, 6, 1, 4..6),
                    Color::Black => (8, 1, 6, 6..8),
                };

                let mut end_index = piece_index as i8 + forward_offset;
                let about_to_promote = piece_index / 8 == last_rank;
                if board.grid[end_index as usize].is_none() {
                    if about_to_promote {
                        add_promote_moves(moves, piece_index, piece_color, end_index as u8);
                    } else {
                        moves.push(Move::new(piece_index, end_index as u8));
                    }

                    // on first rank, do double push
                    if piece_index / 8 == second_rank {
                        end_index += forward_offset;
                        if board.grid[end_index as usize].is_none() {
                            moves.push(
                                Move::new(piece_index, end_index as u8)
                                    .flag(MoveFlag::PawnDoublePush),
                            );
                        }
                    }
                }

                // diagonal captures
                for dir_index in attack_dir_iter {
                    let offset = DIRECTION_OFFSETS[dir_index];
                    let end_index = (piece_index as i8 + offset) as u8;
                    if let Some(end_piece) = board.grid[end_index as usize] {
                        // check different colour and out of bounds (prevents wrapping)
                        if *end_piece.get_color() != piece_color
                            && NUM_TIMES_TO_EDGE[grid_index][dir_index] != 0
                        {
                            if about_to_promote {
                                add_promote_moves(moves, piece_index, piece_color, end_index as u8);
                            } else {
                                moves.push(Move::new(piece_index, end_index));
                            }
                        }
                    }

                    // check for en passant
                    if board.en_passant_square == Some(end_index) {
                        moves.push(
                            Move::new(piece_index, end_index).flag(MoveFlag::EnPassantCapture),
                        );
                    }
                }
            }
        };
    }

    pub fn get_char(&self) -> char {
        match self {
            Self::King(Color::White) => '♚',
            Self::Queen(Color::White) => '♛',
            Self::Rook(Color::White) => '♜',
            Self::Knight(Color::White) => '♞',
            Self::Bishop(Color::White) => '♝',
            Self::Pawn(Color::White) => '♟',

            Self::King(Color::Black) => '♔',
            Self::Queen(Color::Black) => '♕',
            Self::Rook(Color::Black) => '♖',
            Self::Knight(Color::Black) => '♘',
            Self::Bishop(Color::Black) => '♗',
            Self::Pawn(Color::Black) => '♙',
        }
    }
}

impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.get_char())
    }
}

// Array containing the index offset needed to move once in a direction as an index in the order:
// (N, S, W, E, NW, NE, SW, SE) assuming origin is top left
const DIRECTION_OFFSETS: [i8; 8] = [-8, 8, -1, 1, -9, -7, 7, 9];

lazy_static::lazy_static! {
    // Array containing the number of times needed to move in a direction as an index in the order:
    // (N, S, W, E, NW, NE, SW, SE) to get to the edge of the grid for each square.
    // TODO: when const_for is stabilized, change this to a const array
    static ref NUM_TIMES_TO_EDGE: [[i8; 8]; 64] = calc_num_times_to_edge();
}

lazy_static::lazy_static! {
    // Array representing grid containing the possible knight moves (offset) for that grid index
    static ref KNIGHT_MOVES: [Vec<i8>; 64] = calc_knight_moves();
}

fn add_promote_moves(moves: &mut Vec<Move>, piece_index: u8, piece_color: Color, end_index: u8) {
    let base_move = Move::new(piece_index, end_index);
    moves.push(base_move.flag(MoveFlag::Promote(Piece::Queen(piece_color))));
    moves.push(base_move.flag(MoveFlag::Promote(Piece::Rook(piece_color))));
    moves.push(base_move.flag(MoveFlag::Promote(Piece::Bishop(piece_color))));
    moves.push(base_move.flag(MoveFlag::Promote(Piece::Knight(piece_color))));
}

fn handle_sliding_piece(
    moves: &mut Vec<Move>,
    piece_index: u8,
    piece_color: Color,
    board: &Board,
    dir_index_start: usize,
    dir_index_end: usize,
) {
    let grid_index = piece_index as usize;
    for dir_index in dir_index_start..dir_index_end {
        // keep adding offset until reach edge of board or hit piece
        let offset = DIRECTION_OFFSETS[dir_index];
        let mut end_index = piece_index as i8;

        for _ in 0..NUM_TIMES_TO_EDGE[grid_index][dir_index] {
            end_index += offset;
            let move_ = Move::new(piece_index, end_index as u8);

            // hit piece break loop
            if let Some(end_piece) = board.grid[end_index as usize] {
                // piece is opponent so add capture move
                if *end_piece.get_color() != piece_color {
                    moves.push(move_);
                }
                break;
            }

            moves.push(move_);
        }
    }
}

fn calc_num_times_to_edge() -> [[i8; 8]; 64] {
    let mut num_times_to_edge = [[0; 8]; 64];
    for index in 0..64 {
        let (file, rank) = (index as i8 % 8, index as i8 / 8);
        let num_north = rank;
        let num_south = 7 - rank;
        let num_west = file;
        let num_east = 7 - file;

        num_times_to_edge[index] = [
            num_north,
            num_south,
            num_west,
            num_east,
            i8::min(num_north, num_west),
            i8::min(num_north, num_east),
            i8::min(num_south, num_west),
            i8::min(num_south, num_east),
        ]
    }

    num_times_to_edge
}

fn calc_knight_moves() -> [Vec<i8>; 64] {
    const EMPTY_VEC: Vec<i8> = Vec::new();
    let mut knight_moves = [EMPTY_VEC; 64];
    const KNIGHT_OFFSETS: [i8; 8] = [-17, -15, -10, -6, 6, 10, 15, 17];

    for index in 0..64 {
        for offset in KNIGHT_OFFSETS {
            let end_index = index + offset;
            if end_index >= 0 && end_index < 64 {
                let (start_file, start_rank) = (index % 8, index / 8);
                let (end_file, end_rank) = (end_index % 8, end_index / 8);

                // only allow moves that move maximum 2 squares on the file or rank (ensure no wrap)
                let max_xy_move_dist = i8::max(
                    i8::abs(start_file - end_file),
                    i8::abs(start_rank - end_rank),
                );
                if max_xy_move_dist == 2 {
                    knight_moves[index as usize].push(offset)
                }
            }
        }
    }

    knight_moves
}
