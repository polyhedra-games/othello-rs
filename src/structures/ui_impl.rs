use crate::constants::*;
use crate::helpers::draw_rounded_rect;
use crate::structures::{Game, LabelOptions, UI};
use macroquad::prelude::*;
use std::process::exit;

impl UI {
    pub fn new(font: Font) -> Self {
        Self { font }
    }

    fn button(&self, rect: Rect, label: &str) -> bool {
        let (mx, my) = mouse_position();
        let hovered = rect.contains(vec2(mx, my));
        let color = if hovered { COLOR_BK } else { DARKGRAY };

        self.label(
            rect,
            label,
            LabelOptions {
                font_size: 24,
                font_color: COLOR_WT,
                bg_color: color,
            },
        );
        hovered && is_mouse_button_pressed(MouseButton::Left)
    }

    fn label(
        &self,
        rect: Rect,
        label: &str,
        LabelOptions {
            font_size,
            font_color,
            bg_color,
        }: LabelOptions,
    ) {
        draw_rounded_rect(rect.x, rect.y, rect.w, rect.h, 5.0, bg_color);
        let dims = measure_text(label, Some(&self.font), font_size, 1.0);

        let text_x = rect.x + (rect.w - dims.width) / 2.0;
        let text_y = rect.y + (rect.h + dims.height) / 2.0;

        draw_text_ex(
            label,
            text_x,
            text_y,
            TextParams {
                font: Some(&self.font),
                font_size,
                color: font_color,
                ..Default::default()
            },
        );
    }
    pub fn show_win_ui(&self, game: &mut Game) {
        self.label(
            Rect::new(865.0, 130.0, 312.5, 310.0),
            format!("{:?} Won!", game.get_winner()).as_str(),
            LabelOptions {
                font_size: 160,
                font_color: COLOR_WT,
                bg_color: COLOR_BK,
            },
        );
    }
    pub fn show_game_ui(&self, game: &mut Game) {
        if self.button(Rect::new(865.0, 25.0, 312.5, 47.5), "Reset") {
            game.clear();
        }
        if self.button(Rect::new(865.0, 77.5, 312.5, 47.5), "Leave") {
            exit(0);
        }

        self.label(
            Rect::new(865.0, 130.0, 312.5, 310.0),
            &game.count.0.to_string(),
            LabelOptions {
                font_size: 160,
                font_color: COLOR_WT,
                bg_color: COLOR_BK,
            },
        );
        self.label(
            Rect::new(865.0, 445.0, 312.5, 310.0),
            &game.count.1.to_string(),
            LabelOptions {
                font_size: 160,
                font_color: COLOR_BK,
                bg_color: COLOR_WT,
            },
        );

        let (label, font_color, bg_color) = if !game.turn {
            ("Turn: BLACK", COLOR_WT, COLOR_BK)
        } else {
            ("Turn: WHITE", COLOR_BK, COLOR_WT)
        };

        self.label(
            Rect::new(865.0, 760.0, 312.5, 100.0),
            label,
            LabelOptions {
                font_size: 32,
                font_color,
                bg_color,
            },
        );
    }
}
