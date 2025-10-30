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
    let font = load_ttf_font("assets/JBMono.ttf").await.unwrap();
    let ui = UI::new(font);

    loop {
        clear_background(COLOR_BG);
        game.show();
        ui.show(&game);

        // get mouse position
        let (mx, my) = mouse_position();

        // map to board coordinates
        let bx = ((mx - CELL_SIZE * 0.25) / (CELL_SIZE * 1.05)) as isize;
        let by = ((my - CELL_SIZE * 0.25) / (CELL_SIZE * 1.05)) as isize;

        // check if within board
        if (0..8).contains(&bx) && (0..8).contains(&by) {
            game.hovering_over = (bx as usize, by as usize);

            // play if clicked
        } else {
            game.hovering_over = (9, 9);
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            if game.hovering_over != (9, 9) {
                game.play();
            } else {
                println!("{mx}, {my}");
            }
        }

        next_frame().await;
    }
}
