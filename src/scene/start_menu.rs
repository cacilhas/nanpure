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
        let size = raylib::MeasureTextEx(font, rl_str!("1. Extremely Easy"), 64.0, 1.0);
        let extr_easy_bt = Rectangle {
            x: (width - size.x) / 2.0,
            y: bottom,
            width: size.x,
            height: size.y,
        };
        let position = Vector2 {
            x: extr_easy_bt.x,
            y: extr_easy_bt.y,
        };
        let tint = if raylib::CheckCollisionPointRec(mouse_pos, extr_easy_bt) {
            colors::BLACK
        } else {
            colors::DARKGRAY
        };
        raylib::DrawTextEx(
            font,
            rl_str!("1. Extremely Easy"),
            position,
            64.0,
            1.0,
            tint,
        );

        let bottom = bottom + size.y + 12.0;
        let size = raylib::MeasureTextEx(font, rl_str!("2. Easy"), 64.0, 1.0);
        let easy_bt = Rectangle {
            x: (width - size.x) / 2.0,
            y: bottom,
            width: size.x,
            height: size.y,
        };
        let position = Vector2 {
            x: easy_bt.x,
            y: easy_bt.y,
        };
        let tint = if raylib::CheckCollisionPointRec(mouse_pos, easy_bt) {
            colors::BLACK
        } else {
            colors::DARKGRAY
        };
        raylib::DrawTextEx(font, rl_str!("2. Easy"), position, 64.0, 1.0, tint);

        let bottom = bottom + size.y + 12.0;
        let size = raylib::MeasureTextEx(font, rl_str!("3. Medium"), 64.0, 1.0);
        let medium_bt = Rectangle {
            x: (width - size.x) / 2.0,
            y: bottom,
            width: size.x,
            height: size.y,
        };
        let position = Vector2 {
            x: medium_bt.x,
            y: medium_bt.y,
        };
        let tint = if raylib::CheckCollisionPointRec(mouse_pos, medium_bt) {
            colors::BLACK
        } else {
            colors::DARKGRAY
        };
        raylib::DrawTextEx(font, rl_str!("3. Medium"), position, 64.0, 1.0, tint);

        let bottom = bottom + size.y + 12.0;
        let size = raylib::MeasureTextEx(font, rl_str!("4. Hard"), 64.0, 1.0);
        let hard_bt = Rectangle {
            x: (width - size.x) / 2.0,
            y: bottom,
            width: size.x,
            height: size.y,
        };
        let position = Vector2 {
            x: hard_bt.x,
            y: hard_bt.y,
        };
        let tint = if raylib::CheckCollisionPointRec(mouse_pos, hard_bt) {
            colors::BLACK
        } else {
            colors::DARKGRAY
        };
        raylib::DrawTextEx(font, rl_str!("4. Hard"), position, 64.0, 1.0, tint);

        let bottom = bottom + size.y + 12.0;
        let size = raylib::MeasureTextEx(font, rl_str!("5. Fiendish"), 64.0, 1.0);
        let fiendish_bt = Rectangle {
            x: (width - size.x) / 2.0,
            y: bottom,
            width: size.x,
            height: size.y,
        };
        let position = Vector2 {
            x: fiendish_bt.x,
            y: fiendish_bt.y,
        };
        let tint = if raylib::CheckCollisionPointRec(mouse_pos, fiendish_bt) {
            colors::BLACK
        } else {
            colors::DARKGRAY
        };
        raylib::DrawTextEx(font, rl_str!("5. Fiendish"), position, 64.0, 1.0, tint);

        raylib::EndMode2D();
    }
}
