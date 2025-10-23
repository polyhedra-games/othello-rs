use crate::constants::*;
use macroquad::prelude::*;

pub fn window_conf() -> Conf {
    Conf {
        window_title: "Othello".to_string(),
        window_width: (13. * CELL_SIZE) as i32,
        window_height: (8.85 * CELL_SIZE) as i32,
        window_resizable: false,
        ..Default::default()
    }
}
