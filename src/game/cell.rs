use crate::colors;
use raylib::Color;

#[derive(Debug)]
pub struct Cell;

pub static COLORS: [Color; 10] = [
    colors::BLACK,
    colors::RED,
    colors::ORANGE,
    colors::YELLOW,
    colors::GREEN,
    colors::SKYBLUE,
    colors::INDIGO,
    colors::VIOLET,
    colors::MAGENTA,
    colors::GRAY,
];
