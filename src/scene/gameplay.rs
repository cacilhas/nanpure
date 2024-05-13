use std::os::raw::c_int;

use super::{action::Action, Scene};
use crate::{
    game::Game,
    themes::{self, Theme, ThemeContent},
};
use raylib::{enums::KeyboardKey, Camera2D, Font, Rectangle, Vector2};

#[derive(Debug)]
pub struct Gameplay {
    font: Font,
    board: Game,
    theme: Theme,
}

impl Scene for Gameplay {
    unsafe fn run_step(&mut self) -> eyre::Result<Action> {
        if raylib::IsKeyReleased(KeyboardKey::Escape as c_int) {
            return Ok(Action::Pop(1));
        }
        let camera = Camera2D {
            offset: Vector2 { x: 0.0, y: 0.0 },
            rotation: 0.0,
            target: Vector2 { x: 0.0, y: 0.0 },
            zoom: 1.0,
        };
        let theme = self.get_theme();
        raylib::BeginMode2D(camera);
        raylib::ClearBackground(theme.background);
        let canvas = Rectangle {
            x: 0.0,
            y: 0.0,
            width: raylib::GetScreenWidth() as f32,
            height: raylib::GetScreenHeight() as f32,
        };
        self.draw_lines(canvas);
        self.board.draw(canvas);
        raylib::EndMode2D();

        // TODO: gameplay
        Ok(Action::Keep)
    }
}

impl Gameplay {
    pub fn new(font: Font, theme: Theme, level: u8) -> Self {
        let mut board = Game::default();
        unsafe {
            board.shuffle(level.into());
        }
        Self { font, theme, board }
    }

    fn get_theme(&self) -> ThemeContent {
        themes::get(self.theme)
    }

    unsafe fn draw_lines(&mut self, canvas: Rectangle) {
        let width = canvas.width / 9.0;
        let height = canvas.height / 9.0;
        let tint = self.get_theme().hover_foreground;
        for x in 0..=9 {
            raylib::DrawLineEx(
                Vector2 {
                    x: canvas.x + x as f32 * width,
                    y: canvas.y,
                },
                Vector2 {
                    x: canvas.x + x as f32 * width,
                    y: canvas.y + canvas.height,
                },
                if x % 3 == 0 { 2.0 } else { 1.0 },
                tint,
            );
        }
        for y in 0..=9 {
            raylib::DrawLineEx(
                Vector2 {
                    x: canvas.x,
                    y: canvas.y + y as f32 * height,
                },
                Vector2 {
                    x: canvas.x + canvas.width,
                    y: canvas.y + y as f32 * height,
                },
                if y % 3 == 0 { 2.0 } else { 1.0 },
                tint,
            );
        }
    }
}
