use std::os::raw::c_int;

use super::{action::Action, Scene};
use crate::{colors, fonts::get_font};
use raylib::{draw, enums::KeyboardKey, rl_str, Font, Vector2};

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

        raylib::EndMode2D();
    }
}
