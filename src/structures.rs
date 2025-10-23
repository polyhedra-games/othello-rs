pub struct Game {
    pub board: [[Cell; 8]; 8],
    pub turn: bool,
    pub hovering_over: (usize, usize),
    pub valid_moves: Vec<ValidMove>,
}

#[derive(PartialEq, Clone, Debug)]
pub struct ValidMove {
    pub pos: (usize, usize),
    pub pos_to_flip: Vec<(usize, usize)>,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Cell {
    Black,
    White,
    Empty,
}
