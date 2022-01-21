use board_ui::BoardUI;
use macroquad::prelude::*;
use utils::SquareViewport;

mod board_ui;
mod utils;

#[macroquad::main("Cheseng UI")]
async fn main() {
    let mut board_ui = BoardUI::new();
    loop {
        let screen_view = SquareViewport::calc_for_screen();
        let board_pos = screen_view.screen_to_board_pos(mouse_position().into());

        if is_mouse_button_pressed(MouseButton::Left) {
            board_ui.begin_drag(board_pos);
        } else if is_mouse_button_released(MouseButton::Left) {
            board_ui.end_drag(board_pos);
        }

        board_ui.draw(&screen_view);
        next_frame().await;
    }
}
