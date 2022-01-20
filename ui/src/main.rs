use board_ui::BoardUI;
use macroquad::prelude::*;

mod board_ui;

pub struct SquareViewport {
    offset_vec: Vec2,
    pub cell_size: f32,
}

impl SquareViewport {
    pub fn new(x: f32, y: f32, size: f32) -> Self {
        Self {
            offset_vec: vec2(x, y),
            cell_size: size / 8.0,
        }
    }

    pub fn board_to_screen_pos(&self, board_pos: cheseng::Position) -> Vec2 {
        vec2(board_pos.file as f32, board_pos.rank as f32) * self.cell_size + self.offset_vec
    }

    pub fn screen_to_board_pos(&self, screen_pos: Vec2) -> cheseng::Position {
        let pos = (screen_pos - self.offset_vec) / self.cell_size;
        (pos.x as u8, pos.y as u8).into()
    }

    // 1:1 aspect ratio centered
    pub fn calc_for_screen() -> Self {
        let screen_size = if screen_width() > screen_height() {
            screen_height()
        } else {
            screen_width()
        };

        let gap_x = screen_width() - screen_size;
        let gap_y = screen_height() - screen_size;
        Self::new(gap_x / 2.0, gap_y / 2.0, screen_size)
    }
}

#[macroquad::main("Cheseng UI")]
async fn main() {
    let mut board_ui = BoardUI::new();
    loop {
        let screen_view = SquareViewport::calc_for_screen();
        board_ui.update(&screen_view);
        board_ui.draw(&screen_view);
        next_frame().await;
    }
}
