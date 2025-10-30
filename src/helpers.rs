use crate::constants::*;
use macroquad::prelude::*;

pub fn window_conf() -> Conf {
    Conf {
        window_title: "Othello".to_string(),
        window_width: (12. * CELL_SIZE) as i32,
        window_height: (8.85 * CELL_SIZE) as i32,
        window_resizable: false,
        ..Default::default()
    }
}

pub fn draw_rounded_rect(x: f32, y: f32, w: f32, h: f32, radius: f32, color: Color) {
    draw_rectangle(x + radius, y, w - 2.0 * radius, h, color);
    draw_rectangle(x, y + radius, w, h - 2.0 * radius, color);
    draw_circle(x + radius, y + radius, radius, color);
    draw_circle(x + w - radius, y + radius, radius, color);
    draw_circle(x + radius, y + h - radius, radius, color);
    draw_circle(x + w - radius, y + h - radius, radius, color);
}
