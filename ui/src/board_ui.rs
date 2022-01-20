use crate::SquareViewport;
use macroquad::prelude::*;

const BOARD_COLOR_DARK: Color = color_u8!(232, 208, 172, 255);
const BOARD_COLOR_LIGHT: Color = color_u8!(192, 129, 92, 255);

pub struct BoardUI {
    board: cheseng::Board,
    pieces_tileset: Texture2D,
    current_dragged_piece: Option<cheseng::Piece>,
    dragged_piece_orig_pos: Option<cheseng::Position>,
}

impl BoardUI {
    pub fn new() -> Self {
        Self {
            board: cheseng::Board::default(),
            pieces_tileset: Texture2D::from_file_with_format(
                include_bytes!("ChessPieces.png"),
                None,
            ),
            current_dragged_piece: None,
            dragged_piece_orig_pos: None,
        }
    }

    pub fn update(&mut self, screen_view: &SquareViewport) {
        let board_pos = screen_view.screen_to_board_pos(mouse_position().into());

        if is_mouse_button_pressed(MouseButton::Left) && self.current_dragged_piece.is_none() {
            self.current_dragged_piece = self.board.take_piece(board_pos);
            self.dragged_piece_orig_pos = Some(board_pos);
        } else if is_mouse_button_released(MouseButton::Left) {
            // make sure piece is actually being grabbed
            if let (Some(piece), Some(orig_pos)) =
                (self.current_dragged_piece, self.dragged_piece_orig_pos)
            {
                self.board.move_piece(orig_pos, board_pos, piece);
                // let go of piece
                self.current_dragged_piece = None;
            }
        }
    }

    pub fn draw(&self, screen_view: &SquareViewport) {
        for (i, piece) in self.board.grid.iter().enumerate() {
            let board_pos = cheseng::Position::from_index(i);
            let screen_pos = screen_view.board_to_screen_pos(board_pos);

            // draw grid squares
            draw_rectangle(
                screen_pos.x,
                screen_pos.y,
                screen_view.cell_size,
                screen_view.cell_size,
                if (board_pos.file + board_pos.rank) % 2 == 0 {
                    BOARD_COLOR_DARK
                } else {
                    BOARD_COLOR_LIGHT
                },
            );

            if let Some(piece) = piece {
                self.draw_piece(piece, screen_pos, screen_view.cell_size);
            }
        }

        if let Some(piece) = &self.current_dragged_piece {
            let offset = screen_view.cell_size / 2.0;
            self.draw_piece(
                piece,
                Vec2::from(mouse_position()) - vec2(offset, offset),
                screen_view.cell_size,
            );
        }
    }

    fn draw_piece(&self, piece: &cheseng::Piece, screen_pos: Vec2, cell_size: f32) {
        use cheseng::Piece;
        let x_index = match piece {
            Piece::King(..) => 0.,
            Piece::Queen(..) => 1.,
            Piece::Rook(..) => 2.,
            Piece::Bishop(..) => 3.,
            Piece::Knight(..) => 4.,
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
