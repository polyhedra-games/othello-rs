use macroquad::prelude::*;
mod constants;
mod helpers;
mod structures;

use constants::*;
use helpers::*;
use structures::*;

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new();
    let fontfile = include_bytes!("../assets/JBMono.ttf");
    let font = load_ttf_font_from_bytes(fontfile).unwrap();

    let ui = UI::new(font);

    loop {
        clear_background(COLOR_BG);
        match game.state {
            State::Playing => {
                game.show();
                game.mouse_handling();
                ui.show_game_ui(&mut game);
            }
            State::Win(c) => ui.show_win_ui(&mut game),
        }

        next_frame().await;
    }
}
