use std::os::raw::c_int;

use super::{action::Action, pause::Pause, Scene};
use crate::{
    game::{Game, Level, Position},
    themes::{self, Theme, ThemeContent},
};
use raylib::{
    enums::{KeyboardKey, MouseButton},
    rl_str, Camera2D, Font, Rectangle, Vector2,
};

#[derive(Debug)]
pub struct Gameplay {
    font: Font,
    board: Game,
    theme: Theme,
    player: Position,
    draft: bool,
    time: f32,
    game_over: bool,
    level: Level,
}

impl Scene for Gameplay {
    unsafe fn run_step(&mut self) -> eyre::Result<Action> {
        if raylib::IsKeyReleased(KeyboardKey::Escape as c_int) {
            return Ok(Action::Pop(1));
        }
        if raylib::IsKeyReleased(KeyboardKey::Pause as c_int) {
            return Ok(Action::Push(Box::new(Pause::new(self.font, self.theme))));
        }

        let camera = Camera2D {
            offset: Vector2 { x: 0.0, y: 0.0 },
            rotation: 0.0,
            target: Vector2 { x: 0.0, y: 0.0 },
            zoom: 1.0,
        };
        let theme = self.get_theme();
        let screen = Vector2 {
            x: raylib::GetScreenWidth() as f32,
            y: raylib::GetScreenHeight() as f32,
        };
        let screen = Rectangle {
            x: if screen.x > screen.y {
                (screen.x - screen.y) / 2.0
            } else {
                0.0
            },
            y: 0.0,
            width: if screen.x > screen.y {
                screen.y
            } else {
                screen.x
            },
            height: screen.y,
        };
        let canvas = Rectangle {
            x: screen.x,
            y: screen.y,
            width: screen.width,
            height: screen.height - 48.0,
        };
        let info = Rectangle {
            x: 12.0 + screen.x,
            y: screen.y + canvas.height,
            width: screen.width - 24.0,
            height: screen.height - canvas.height,
        };
        let background = if self.game_over {
            theme.title
        } else {
            theme.background
        };
        raylib::BeginMode2D(camera);
        raylib::ClearBackground(background);

        if raylib::IsWindowFocused() || self.game_over {
            if !self.game_over {
                self.time += raylib::GetFrameTime();
                self.move_player();
                self.detect_click(canvas);
                self.detect_change_cell();
                self.draw_player(canvas);
                self.draw_draft_bt(info);
            }
            self.draw_lines(canvas);
            self.board.draw(canvas);
        } else {
            self.draw_locked(canvas);
        }
        self.draw_level(info);
        self.draw_timelapse(info);
        raylib::EndMode2D();

        self.game_over = self.board.is_game_over();

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
        Self {
            font,
            theme,
            board,
            player: Position { x: 4, y: 4 },
            draft: false,
            time: 0.0,
            game_over: false,
            level: level.into(),
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

    unsafe fn detect_change_cell(&mut self) {
        if raylib::IsKeyReleased(KeyboardKey::LeftControl as c_int)
            || raylib::IsKeyReleased(KeyboardKey::RightControl as c_int)
            || raylib::IsKeyReleased(KeyboardKey::KpDecimal as c_int)
        {
            self.draft = !self.draft;
        }

        let key = raylib::GetKeyPressed();
        if (key >= KeyboardKey::Zero as i32 && key <= KeyboardKey::Nine as i32)
            || (key >= KeyboardKey::Kp0 as i32 && key <= KeyboardKey::Kp9 as i32)
        {
            let code = if key >= KeyboardKey::Zero as i32 && key <= KeyboardKey::Nine as i32 {
                key - KeyboardKey::Zero as i32
            } else {
                key - KeyboardKey::Kp0 as i32
            } as u8;

            if self.draft {
                self.board
                    .toggle_candidate(self.player.x, self.player.y, code);
            } else {
                let value = if code == 0 { None } else { Some(code) };
                self.board.set_cell(self.player.x, self.player.y, value);
            }
        }
    }

    unsafe fn draw_player(&self, canvas: Rectangle) {
        let width = canvas.width / 9.0;
        let height = canvas.height / 9.0;
        let x = canvas.x + self.player.x as f32 * width;
        let y = canvas.y + self.player.y as f32 * height;
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

    unsafe fn draw_locked(&self, canvas: Rectangle) {
        let theme = self.get_theme();
        let size = raylib::MeasureTextEx(self.font, rl_str!("LOCKED"), 64.0, 2.0);
        let position = Vector2 {
            x: (canvas.width - size.x) / 2.0,
            y: (canvas.height - size.y) / 2.0,
        };
        raylib::DrawTextEx(
            self.font,
            rl_str!("LOCKED"),
            position,
            64.0,
            2.0,
            theme.foreground,
        );
    }

    unsafe fn draw_draft_bt(&mut self, canvas: Rectangle) {
        let mouse = raylib::GetMousePosition();
        let theme = self.get_theme();
        let size = raylib::MeasureTextEx(self.font, rl_str!("Draft"), 24.0, 1.0);
        let bt = Rectangle {
            x: canvas.x,
            y: canvas.y + (canvas.height - size.y) / 2.0,
            width: size.x,
            height: size.y,
        };
        let tint = if self.draft {
            raylib::DrawRectangleRec(bt, theme.hover_foreground);
            theme.hover_background
        } else {
            theme.foreground
        };
        raylib::DrawTextEx(
            self.font,
            rl_str!("Draft"),
            Vector2 { x: bt.x, y: bt.y },
            24.0,
            1.0,
            tint,
        );
        if raylib::IsMouseButtonPressed(MouseButton::Left as c_int)
            && raylib::CheckCollisionPointRec(mouse, bt)
        {
            self.draft = !self.draft;
        }
    }

    unsafe fn draw_level(&self, canvas: Rectangle) {
        let theme = self.get_theme();
        let text = format!("{:?}", self.level);
        let size = raylib::MeasureTextEx(self.font, rl_str!(text), 24.0, 1.0);
        let position = Vector2 {
            x: canvas.x + (canvas.width - size.x) / 2.0,
            y: canvas.y + (canvas.height - size.y) / 2.0,
        };
        raylib::DrawTextEx(
            self.font,
            rl_str!(text),
            position,
            24.0,
            1.0,
            theme.foreground,
        );
    }

    unsafe fn draw_timelapse(&self, canvas: Rectangle) {
        let mouse = raylib::GetMousePosition();
        let theme = self.get_theme();
        let time = self.time.floor() as u32;
        let text = format!("{:02}:{:02}", time / 60, time % 60);
        let size = raylib::MeasureTextEx(self.font, rl_str!(text), 24.0, 1.0);
        let label = Rectangle {
            x: canvas.x + canvas.width - size.x,
            y: canvas.y + (canvas.height - size.y) / 2.0,
            width: size.x,
            height: size.y,
        };
        if raylib::CheckCollisionPointRec(mouse, canvas) {
            raylib::DrawTextEx(
                self.font,
                rl_str!(text),
                Vector2 {
                    x: label.x,
                    y: label.y,
                },
                24.0,
                1.0,
                theme.foreground,
            );
        }
    }
}
