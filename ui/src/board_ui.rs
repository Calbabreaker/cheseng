use crate::SquareViewport;
use macroquad::prelude::*;

struct PieceWrapper {
    internal_piece: cheseng::Piece,
    index: u8,
    legal_moves: Vec<cheseng::Move>,
}

pub struct BoardUI {
    board: cheseng::Board,
    pieces_tileset: Texture2D,
    dragged_piece: Option<PieceWrapper>,
}

impl BoardUI {
    pub fn new() -> Self {
        Self {
            board: cheseng::Board::default(),
            pieces_tileset: Texture2D::from_file_with_format(
                include_bytes!("ChessPieces.png"),
                None,
            ),
            dragged_piece: None,
        }
    }

    pub fn begin_drag(&mut self, board_pos: cheseng::Position) {
        if !self.dragged_piece.is_none() {
            return;
        }

        if let Ok(index) = board_pos.as_index() {
            if let Some(internal_piece) = self.board.grid[index as usize] {
                self.dragged_piece = Some(PieceWrapper {
                    internal_piece,
                    index,
                    legal_moves: self.board.generate_moves_for_piece(&internal_piece, index),
                });
            }
        }
    }

    pub fn end_drag(&mut self, board_pos: cheseng::Position) {
        // make sure piece is actually being grabbed
        if let Some(piece) = &self.dragged_piece {
            // make sure mouse pos is inside bounds
            if let Ok(end_index) = board_pos.as_index() {
                let is_legal_move = piece
                    .legal_moves
                    .iter()
                    .any(|legal_move| legal_move.end_index == end_index);
                if is_legal_move {
                    self.board
                        .make_move(cheseng::Move::new(piece.index, end_index));
                }
            }
        }

        // let go of piece
        self.dragged_piece = None;
    }

    pub fn draw(&self, screen_view: &SquareViewport) {
        for (i, piece) in self.board.grid.iter().enumerate() {
            let board_pos = cheseng::Position::from_index(i as u8);
            let screen_pos = screen_view.board_to_screen_pos(board_pos);

            // draw grid squares
            const BOARD_COLOR_LIGHT: Color = color_u8!(232, 208, 172, 255);
            const BOARD_COLOR_DARK: Color = color_u8!(187, 138, 99, 255);
            draw_rectangle(
                screen_pos.x,
                screen_pos.y,
                screen_view.cell_size,
                screen_view.cell_size,
                if (board_pos.file + board_pos.rank) % 2 == 0 {
                    BOARD_COLOR_LIGHT
                } else {
                    BOARD_COLOR_DARK
                },
            );

            if let Some(piece) = piece {
                self.draw_piece(piece, screen_pos, screen_view.cell_size);
            }
        }

        if let Some(piece) = &self.dragged_piece {
            // draw legal move
            self.draw_moves_hints(&screen_view, &piece.legal_moves);

            // draw actual piece
            let offset = screen_view.cell_size / 2.0;
            self.draw_piece(
                &piece.internal_piece,
                Vec2::from(mouse_position()) - vec2(offset, offset),
                screen_view.cell_size,
            );
        }
    }

    fn draw_moves_hints(&self, screen_view: &SquareViewport, moves: &Vec<cheseng::Move>) {
        let cell_size = screen_view.cell_size;
        const MOVE_HINT_COLOR: Color = color_u8!(89, 133, 41, 255);
        for move_ in moves {
            let screen_pos = screen_view.board_to_screen_pos(move_.end_index.into());

            if self.board.grid[move_.end_index as usize].is_some() {
                // draw captures
                draw_rectangle_lines(
                    screen_pos.x,
                    screen_pos.y,
                    cell_size,
                    cell_size,
                    cell_size / 10.,
                    MOVE_HINT_COLOR,
                );
            } else {
                // normal moves
                draw_circle(
                    screen_pos.x + cell_size / 2.0,
                    screen_pos.y + cell_size / 2.0,
                    cell_size / 8.0,
                    MOVE_HINT_COLOR,
                );
            }
        }
    }

    fn draw_piece(&self, piece: &cheseng::Piece, screen_pos: Vec2, cell_size: f32) {
        use cheseng::Piece;
        let x_index = match piece {
            Piece::King(..) => 0.,
            Piece::Queen(..) => 1.,
            Piece::Bishop(..) => 2.,
            Piece::Knight(..) => 3.,
            Piece::Rook(..) => 4.,
            Piece::Pawn(..) => 5.,
        };

        let y_index = match piece.get_color() {
            cheseng::Color::White => 0.,
            cheseng::Color::Black => 1.,
        };

        let piece_size = self.pieces_tileset.height() / 2.0;
        let piece_rect = Rect::new(
            x_index * piece_size,
            y_index * piece_size,
            piece_size,
            piece_size,
        );

        draw_texture_ex(
            self.pieces_tileset,
            screen_pos.x,
            screen_pos.y,
            WHITE,
            DrawTextureParams {
                source: Some(piece_rect),
                dest_size: Some(vec2(cell_size, cell_size)),
                ..Default::default()
            },
        );
    }
}
