use std::os::raw::c_int;

use super::{action::Action, Scene};
use crate::{
    game::{Game, Position},
    themes::{self, Theme, ThemeContent},
};
use raylib::{
    enums::{KeyboardKey, MouseButton},
    Camera2D, Font, Rectangle, Vector2,
};

#[derive(Debug)]
pub struct Gameplay {
    font: Font,
    board: Game,
    theme: Theme,
    player: Position,
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
        self.move_player();
        self.detect_click(canvas);
        self.draw_player(canvas);
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
        let player = Position { x: 4, y: 4 };
        unsafe {
            board.shuffle(level.into());
        }
        Self {
            font,
            theme,
            player,
            board,
        }
    }

    fn get_theme(&self) -> ThemeContent {
        themes::get(self.theme)
    }

    unsafe fn move_player(&mut self) {
        if raylib::IsKeyPressed(KeyboardKey::Left as c_int)
            || raylib::IsKeyPressed(KeyboardKey::A as c_int)
        {
            self.player.x = (self.player.x + 8) % 9;
        }
        if raylib::IsKeyPressed(KeyboardKey::Right as c_int)
            || raylib::IsKeyPressed(KeyboardKey::D as c_int)
        {
            self.player.x = (self.player.x + 1) % 9;
        }
        if raylib::IsKeyPressed(KeyboardKey::Up as c_int)
            || raylib::IsKeyPressed(KeyboardKey::W as c_int)
        {
            self.player.y = (self.player.y + 8) % 9;
        }
        if raylib::IsKeyPressed(KeyboardKey::Down as c_int)
            || raylib::IsKeyPressed(KeyboardKey::S as c_int)
        {
            self.player.y = (self.player.y + 1) % 9;
        }
    }

    unsafe fn detect_click(&mut self, canvas: Rectangle) {
        if raylib::IsMouseButtonPressed(MouseButton::Left as c_int) {
            let mouse = raylib::GetMousePosition();
            if raylib::CheckCollisionPointRec(mouse, canvas) {
                let width = canvas.width / 9.0;
                let height = canvas.height / 9.0;
                let x = (mouse.x - canvas.x) / width;
                let y = (mouse.y - canvas.y) / height;
                let x = x as u8;
                let y = y as u8;
                self.player = Position { x, y };
            }
        }
    }

    unsafe fn draw_player(&self, canvas: Rectangle) {
        let width = canvas.width / 9.0;
        let height = canvas.height / 9.0;
        let x = (canvas.x + self.player.x as f32) * width;
        let y = (canvas.y + self.player.y as f32) * height;
        let theme = self.get_theme();
        raylib::DrawRectangle(
            x as i32,
            y as i32,
            width as i32,
            height as i32,
            theme.hover_background,
        );
    }

    unsafe fn draw_lines(&self, canvas: Rectangle) {
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
                if x % 3 == 0 { 3.0 } else { 1.0 },
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
                if y % 3 == 0 { 3.0 } else { 1.0 },
                tint,
            );
        }
    }
}
