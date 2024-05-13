use std::os::raw::c_int;

use super::Scene;
use crate::{colors, scene::Action};
use raylib::{draw, enums::KeyboardKey};

#[derive(Debug, Default)]
pub struct StartMenu;

impl Scene for StartMenu {
    unsafe fn run_step(&mut self) -> Action {
        if raylib::IsKeyReleased(KeyboardKey::Escape as c_int) {
            return Action::Pop;
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
        // TODO: menu
        raylib::EndMode2D();
    }
}
