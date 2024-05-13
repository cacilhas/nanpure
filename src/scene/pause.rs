use std::os::raw::c_int;

use super::{action::Action, Scene};
use crate::themes::{self, Theme};
use raylib::{enums::KeyboardKey, rl_str, Font, Vector2};

#[derive(Debug)]
pub struct Pause {
    font: Font,
    theme: Theme,
}

impl Pause {
    pub fn new(font: Font, theme: Theme) -> Self {
        Self { font, theme }
    }
}

impl Scene for Pause {
    unsafe fn run_step(&mut self) -> eyre::Result<Action> {
        if raylib::IsKeyReleased(KeyboardKey::Pause as c_int) {
            return Ok(Action::Pop(1));
        }
        if raylib::IsKeyReleased(KeyboardKey::Escape as c_int) {
            return Ok(Action::Pop(2));
        }
        let camera = raylib::Camera2D {
            offset: raylib::Vector2 { x: 0.0, y: 0.0 },
            rotation: 0.0,
            target: raylib::Vector2 { x: 0.0, y: 0.0 },
            zoom: 1.0,
        };
        let theme = themes::get(self.theme);
        raylib::BeginMode2D(camera);
        raylib::ClearBackground(theme.background);

        let font = self.font;
        let width = raylib::GetScreenWidth() as f32;
        let height = raylib::GetScreenHeight() as f32;
        let size = raylib::MeasureTextEx(font, rl_str!("Nanpurë"), 84.0, 2.0);
        let position = Vector2 {
            x: (width - size.x) / 2.0,
            y: 0.0,
        };
        raylib::DrawTextEx(font, rl_str!("Nanpurë"), position, 84.0, 2.0, theme.title);
        let bottom = size.y + 16.0;
        let size = raylib::MeasureTextEx(font, rl_str!("(Sudoku)"), 32.0, 1.0);
        let position = Vector2 {
            x: (width - size.x) / 2.0,
            y: bottom,
        };
        raylib::DrawTextEx(font, rl_str!("(Sudoku)"), position, 32.0, 1.0, theme.title);
        let size = raylib::MeasureTextEx(font, rl_str!("Paused"), 64.0, 2.0);
        let position = Vector2 {
            x: (width - size.x) / 2.0,
            y: (height - size.y) / 2.0,
        };
        raylib::DrawTextEx(
            font,
            rl_str!("Paused"),
            position,
            64.,
            2.0,
            theme.foreground,
        );

        raylib::EndMode2D();
        Ok(Action::Keep)
    }
}
