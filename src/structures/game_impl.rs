use crate::constants::*;
use crate::helpers::draw_rounded_rect;
use crate::structures::{Cell, Game, ValidMove};
use macroquad::prelude::*;
impl Game {
    pub fn new() -> Self {
        let mut new_game = Self {
            board: [[Cell::Empty; 8]; 8],
            turn: false,
            hovering_over: (0, 0),
            valid_moves: vec![],
            count: (0, 0),
        };
        new_game.clear();
        new_game
    }
    pub fn clear(&mut self) {
        self.board = [[Cell::Empty; 8]; 8];
        self.board[3][3] = Cell::Black;
        self.board[4][4] = Cell::Black;
        self.board[3][4] = Cell::White;
        self.board[4][3] = Cell::White;
        self.calculate_moves();
        self.update_count();
    }
    pub fn show(&self) {
        for i in 0..64 {
            let x = i % 8;
            let y = i / 8;

            draw_rounded_rect(
                CELL_SIZE * 0.25 + x as f32 * 1.05 * CELL_SIZE,
                CELL_SIZE * 0.25 + y as f32 * 1.05 * CELL_SIZE,
                CELL_SIZE,
                CELL_SIZE,
                5.0,
                {
                    if self.hovering_over == (x, y) {
                        COLOR_HV
                    } else {
                        COLOR_FG
                    }
                },
            );

            match self.board[x][y] {
                Cell::Black => {
                    draw_circle(
                        CELL_SIZE * 0.75 + x as f32 * 1.05 * CELL_SIZE,
                        CELL_SIZE * 0.75 + y as f32 * 1.05 * CELL_SIZE,
                        CELL_SIZE * 0.4,
                        COLOR_BK,
                    );
                }
                Cell::White => {
                    draw_circle(
                        CELL_SIZE * 0.75 + x as f32 * 1.05 * CELL_SIZE,
                        CELL_SIZE * 0.75 + y as f32 * 1.05 * CELL_SIZE,
                        CELL_SIZE * 0.4,
                        COLOR_WT,
                    );
                }
                _ => (),
            }
            for valid_move in self.valid_moves.clone() {
                let (x, y) = valid_move.pos;
                draw_circle(
                    CELL_SIZE * 0.75 + x as f32 * 1.05 * CELL_SIZE,
                    CELL_SIZE * 0.75 + y as f32 * 1.05 * CELL_SIZE,
                    CELL_SIZE * 0.2,
                    COLOR_MV,
                );
            }
        }
    }

    pub fn play(&mut self) {
        let mut valid_move = None;
        let (x, y) = self.hovering_over;
        for i in self.valid_moves.clone() {
            if i.pos == (x, y) {
                valid_move = Some(i);
                break;
            }
        }

        if valid_move.is_some() {
            self.turn = !self.turn;
            self.board[x][y] = if self.turn { Cell::Black } else { Cell::White };
            for (i, j) in valid_move.unwrap().pos_to_flip {
                match self.board[i][j] {
                    Cell::Black => self.board[i][j] = Cell::White,
                    Cell::White => self.board[i][j] = Cell::Black,
                    Cell::Empty => panic!("Tried flipping empty cell. Should never happen"),
                }
            }
            self.update_count();
            self.calculate_moves();
        } else {
            eprintln!("othello: ({x}, {y}):Invalid Move!");
        }
    }

    fn update_count(&mut self) {
        self.count = (0, 0);
        for i in 1..64 {
            let x = i % 8;
            let y = i / 8;

            match self.board[x][y] {
                Cell::Black => self.count.0 += 1,
                Cell::White => self.count.1 += 1,
                _ => (),
            }
        }
    }
    pub fn calculate_moves(&mut self) {
        self.valid_moves.clear();

        let (my_color, opp_color) = if self.turn {
            (Cell::White, Cell::Black)
        } else {
            (Cell::Black, Cell::White)
        };

        let dirs = [
            (0, -1),
            (1, -1),
            (1, 0),
            (1, 1),
            (0, 1),
            (-1, 1),
            (-1, 0),
            (-1, -1),
        ];

        for x in 0..8 {
            for y in 0..8 {
                if self.board[x][y] != Cell::Empty {
                    continue;
                }

                let mut all_flips: Vec<(usize, usize)> = vec![];

                for &(dx, dy) in &dirs {
                    let mut nx = x as isize + dx;
                    let mut ny = y as isize + dy;
                    let mut line: Vec<(usize, usize)> = vec![];
                    let mut seen_opp = false;

                    while (0..8).contains(&nx) && (0..8).contains(&ny) {
                        match self.board[nx as usize][ny as usize] {
                            c if c == opp_color => {
                                seen_opp = true;
                                line.push((nx as usize, ny as usize));
                            }
                            c if c == my_color && seen_opp => {
                                all_flips.extend(line);
                                break;
                            }
                            _ => break,
                        }
                        nx += dx;
                        ny += dy;
                    }
                }

                if !all_flips.is_empty() {
                    self.valid_moves.push(ValidMove {
                        pos: (x, y),
                        pos_to_flip: all_flips,
                    });
                }
            }
        }
    }
}
