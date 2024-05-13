use std::os::raw::c_int;

use super::{action::Action, Scene};
use crate::colors;
use raylib::{enums::KeyboardKey, Font};

#[derive(Debug)]
pub struct Gameplay {
    font: Font,
    level: u32, // TODO: replace by the game board
}

impl Scene for Gameplay {
    unsafe fn run_step(&mut self) -> Action {
        if raylib::IsKeyReleased(KeyboardKey::Escape as c_int) {
            return Action::Pop(1);
        }
        raylib::ClearBackground(colors::WHEAT);

        // TODO: gameplay
        Action::Keep
    }
}

impl Gameplay {
    pub fn new(font: Font, level: u32) -> Self {
        // TODO: given a level, instantiate game board
        Self { font, level }
    }
}
