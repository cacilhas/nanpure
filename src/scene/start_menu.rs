use std::os::raw::c_int;

use super::{action::Action, Scene};
use crate::{colors, fonts::get_font};
use raylib::{draw, enums::KeyboardKey, rl_str, Font, Rectangle, Vector2};

#[derive(Debug)]
pub struct StartMenu {
    font: Font,
}

impl Default for StartMenu {
    fn default() -> Self {
        unsafe {
            Self {
                font: get_font().unwrap(),
            }
        }
    }
}

impl Scene for StartMenu {
    unsafe fn run_step(&mut self) -> Action {
        if raylib::IsKeyReleased(KeyboardKey::Escape as c_int) {
            return Action::Pop(1);
        }
        draw! {
            self.draw2d()
        }
        Action::Keep
    }
}

impl StartMenu {
    unsafe fn draw2d(&mut self) {
        let camera = raylib::Camera2D {
            offset: raylib::Vector2 { x: 0.0, y: 0.0 },
            rotation: 0.0,
            target: raylib::Vector2 { x: 0.0, y: 0.0 },
            zoom: 1.0,
        };
        raylib::BeginMode2D(camera);
        raylib::ClearBackground(colors::WHEAT);

        let font = self.font;
        let width = raylib::GetScreenWidth() as f32;
        let size = raylib::MeasureTextEx(font, rl_str!("Nanpurë"), 84.0, 2.0);
        let position = Vector2 {
            x: (width - size.x) / 2.0,
            y: 0.0,
        };
        raylib::DrawTextEx(
            font,
            rl_str!("Nanpurë"),
            position,
            84.0,
            2.0,
            colors::DARKCYAN,
        );
        let bottom = size.y + 16.0;
        let size = raylib::MeasureTextEx(font, rl_str!("(Sudoku)"), 32.0, 1.0);
        let position = Vector2 {
            x: (width - size.x) / 2.0,
            y: bottom,
        };
        raylib::DrawTextEx(
            font,
            rl_str!("(Sudoku)"),
            position,
            32.0,
            1.0,
            colors::DARKCYAN,
        );
        let mouse_pos = raylib::GetMousePosition();

        let bottom = bottom + size.y + 64.0;
        let (etr_easy_bt, bottom) = self.draw_bt("1. Extremely Easy", width, bottom, mouse_pos);
        let bottom = bottom + 12.0;
        let (easy_bt, bottom) = self.draw_bt("2. Easy", width, bottom, mouse_pos);
        let bottom = bottom + 12.0;
        let (medium_bt, bottom) = self.draw_bt("3. Medium", width, bottom, mouse_pos);
        let bottom = bottom + 12.0;
        let (hard_bt, bottom) = self.draw_bt("4. Hard", width, bottom, mouse_pos);
        let bottom = bottom + 12.0;
        let (fiendish_bt, _) = self.draw_bt("5. Fiendish", width, bottom, mouse_pos);

        raylib::EndMode2D();
    }

    unsafe fn draw_bt(
        &self,
        text: impl Into<String>,
        width: f32,
        y: f32,
        mouse_pos: Vector2,
    ) -> (Rectangle, f32) {
        let text = text.into();
        let size = raylib::MeasureTextEx(self.font, rl_str!(text), 64.0, 1.0);
        let bt = Rectangle {
            x: (width - size.x) / 2.0,
            y,
            width: size.x,
            height: size.y,
        };
        let position = Vector2 { x: bt.x, y: bt.y };
        let tint = if raylib::CheckCollisionPointRec(mouse_pos, bt) {
            colors::BLACK
        } else {
            colors::DARKGRAY
        };
        raylib::DrawTextEx(self.font, rl_str!(text), position, 64.0, 1.0, tint);
        (bt, y + size.y)
    }
}
