use macroquad::prelude::*;

pub struct SquareViewport {
    pub offset_vec: Vec2,
    pub cell_size: f32,
    pub size: f32,
}

impl SquareViewport {
    pub fn new(x: f32, y: f32, size: f32) -> Self {
        Self {
            offset_vec: vec2(x, y),
            cell_size: size / 8.0,
            size,
        }
    }

    pub fn board_to_screen_pos(&self, board_pos: cheseng::Position) -> Vec2 {
        vec2(board_pos.file as f32, board_pos.rank as f32) * self.cell_size + self.offset_vec
    }

    pub fn screen_to_board_pos(&self, screen_pos: Vec2) -> cheseng::Position {
        let pos = (screen_pos - self.offset_vec) / self.cell_size;
        cheseng::pos(pos.x as u8, pos.y as u8)
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
