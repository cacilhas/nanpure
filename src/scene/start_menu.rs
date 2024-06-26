use std::os::raw::c_int;

use super::{action::Action, gameplay::Gameplay, helpers, Scene};
use crate::{
    colors,
    fonts::get_font,
    settings::{self, Config},
    themes::{self, Theme},
};
use raylib::{
    enums::{KeyboardKey, MouseButton},
    rl_str, Font, Rectangle, Vector2,
};

#[derive(Debug)]
pub struct StartMenu {
    font: Font,
    theme: Theme,
    config: Config,
}

impl Default for StartMenu {
    fn default() -> Self {
        let config = settings::load();
        unsafe {
            Self {
                font: get_font().unwrap(),
                theme: config.theme.unwrap_or(Theme::Light),
                config,
            }
        }
    }
}

impl Scene for StartMenu {
    unsafe fn run_step(&mut self) -> eyre::Result<Action> {
        if raylib::IsKeyReleased(KeyboardKey::Escape as c_int) {
            return self.exit(Action::Pop(1));
        }
        let camera = raylib::Camera2D {
            offset: raylib::Vector2 { x: 0.0, y: 0.0 },
            rotation: 0.0,
            target: raylib::Vector2 { x: 0.0, y: 0.0 },
            zoom: 1.0,
        };
        let theme = themes::get(self.theme);
        raylib::BeginMode2D(camera);
        raylib::ClearBackground(colors::DARKGRAY);

        let screen = helpers::get_screen();
        raylib::DrawRectangleRec(screen, theme.background);

        let font = self.font;
        let size = raylib::MeasureTextEx(font, rl_str!("Nanpurë"), 84.0, 2.0);
        let mut bottom = screen.y;
        let position = Vector2 {
            x: screen.x + (screen.width - size.x) / 2.0,
            y: bottom,
        };
        raylib::DrawTextEx(font, rl_str!("Nanpurë"), position, 84.0, 2.0, theme.title);
        bottom += size.y + 16.0;
        let size = raylib::MeasureTextEx(font, rl_str!("(Sudoku)"), 32.0, 1.0);
        let position = Vector2 {
            x: screen.x + (screen.width - size.x) / 2.0,
            y: bottom,
        };
        raylib::DrawTextEx(font, rl_str!("(Sudoku)"), position, 32.0, 1.0, theme.title);
        let mouse_pos = raylib::GetMousePosition();

        let (etr_easy_bt, bottom) =
            self.draw_bt("1. Extremely Easy", screen, bottom + 64.0, mouse_pos);
        let (easy_bt, bottom) = self.draw_bt("2. Easy", screen, bottom + 12.0, mouse_pos);
        let (medium_bt, bottom) = self.draw_bt("3. Medium", screen, bottom + 12.0, mouse_pos);
        let (hard_bt, bottom) = self.draw_bt("4. Hard", screen, bottom + 12.0, mouse_pos);
        let (fiendish_bt, _) = self.draw_bt("5. Fiendish", screen, bottom + 12.0, mouse_pos);
        let theme_bt = self.theme_bt(screen, mouse_pos);

        if raylib::IsMouseButtonPressed(MouseButton::Left as c_int) {
            let mut level: u8 = 0;
            if raylib::CheckCollisionPointRec(mouse_pos, etr_easy_bt) {
                level = 1;
            }
            if raylib::CheckCollisionPointRec(mouse_pos, easy_bt) {
                level = 2;
            }
            if raylib::CheckCollisionPointRec(mouse_pos, medium_bt) {
                level = 3;
            }
            if raylib::CheckCollisionPointRec(mouse_pos, hard_bt) {
                level = 4;
            }
            if raylib::CheckCollisionPointRec(mouse_pos, fiendish_bt) {
                level = 5;
            }
            if level != 0 {
                return self.exit(Action::Push(Box::new(Gameplay::new(
                    font, self.theme, level,
                ))));
            }
            if raylib::CheckCollisionPointRec(mouse_pos, theme_bt) {
                self.theme = theme.next().r#type;
            }
        }

        for level in 1..=5 {
            let code = (KeyboardKey::Zero as i32 + level) as c_int;
            if raylib::IsKeyReleased(code) {
                return self.exit(Action::Push(Box::new(Gameplay::new(
                    font,
                    self.theme,
                    level as u8,
                ))));
            }
            let code = (KeyboardKey::Kp0 as i32 + level) as c_int;
            if raylib::IsKeyReleased(code) {
                return self.exit(Action::Push(Box::new(Gameplay::new(
                    font,
                    self.theme,
                    level as u8,
                ))));
            }
        }

        raylib::EndMode2D();

        Ok(Action::Keep)
    }
}

impl StartMenu {
    fn exit(&self, action: Action) -> eyre::Result<Action> {
        let mut config = self.config;
        config.theme = Some(self.theme);
        settings::save(config);
        Ok(action)
    }

    unsafe fn draw_bt(
        &self,
        text: impl Into<String>,
        canvas: Rectangle,
        y: f32,
        mouse_pos: Vector2,
    ) -> (Rectangle, f32) {
        let text = text.into();
        let theme = themes::get(self.theme);
        let size = raylib::MeasureTextEx(self.font, rl_str!(text), 64.0, 1.0);
        let bt = Rectangle {
            x: canvas.x,
            y: canvas.y + y,
            width: canvas.width,
            height: size.y,
        };
        let position = Vector2 {
            x: bt.x + (canvas.width - size.x) / 2.0,
            y: bt.y,
        };
        let tint = if raylib::CheckCollisionPointRec(mouse_pos, bt) {
            raylib::DrawRectangleRec(bt, theme.hover_background);
            theme.hover_foreground
        } else {
            theme.foreground
        };
        raylib::DrawTextEx(self.font, rl_str!(text), position, 64.0, 1.0, tint);
        (bt, canvas.y + y + size.y)
    }

    unsafe fn theme_bt(&mut self, canvas: Rectangle, mouse_pos: Vector2) -> Rectangle {
        let mut theme = themes::get(self.theme);
        let mut text = format!(" {}", theme.r#type);
        let next = theme.next();
        let size = raylib::MeasureTextEx(self.font, rl_str!("0000000000"), 28.0, 1.0);
        let bt = Rectangle {
            x: canvas.x + canvas.width - size.x - 4.0,
            y: canvas.y + canvas.height - size.y - 4.0,
            width: size.x,
            height: size.y,
        };
        let mut tint = theme.foreground;
        if raylib::CheckCollisionPointRec(mouse_pos, bt) {
            theme = next;
            text = format!(" {}", theme.r#type);
            raylib::DrawRectangleRec(bt, theme.background);
            tint = theme.hover_foreground;
        }
        raylib::DrawTextEx(
            self.font,
            rl_str!(text),
            Vector2 { x: bt.x, y: bt.y },
            28.0,
            1.0,
            tint,
        );

        bt
    }
}
