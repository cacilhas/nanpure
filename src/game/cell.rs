use rscenes::prelude::Color;

#[derive(Debug)]
pub struct Cell;

pub static COLORS: [Color; 10] = [
    Color::BLACK,
    Color::RED,
    Color::ORANGE,
    Color::YELLOW,
    Color::GREEN,
    Color::SKYBLUE,
    Color::new(75, 0, 130, 255), //INDIGO,
    Color::VIOLET,
    Color::MAGENTA,
    Color::DARKGRAY,
];
