use macroquad::prelude::{Color, Font};

mod game_impl;
mod ui_impl;

pub struct LabelOptions {
    font_size: u16,
    font_color: Color,
    bg_color: Color,
}

pub enum State {
    Playing,
    Win(Cell),
}

pub struct UI {
    font: Font,
}

pub struct Game {
    pub board: [[Cell; 8]; 8],
    pub turn: bool,
    pub hovering_over: (usize, usize),
    pub valid_moves: Vec<ValidMove>,
    pub count: (u8, u8),
    pub skipped: bool,
    pub state: State,
}
#[derive(PartialEq, Clone, Debug)]
pub struct ValidMove {
    pub pos: (usize, usize),
    pub pos_to_flip: Vec<(usize, usize)>,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Cell {
    Black,
    White,
    Empty,
}
