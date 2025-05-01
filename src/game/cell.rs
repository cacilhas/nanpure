use bevy::color::{Color, Srgba};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell;

pub static COLORS: [Color; 10] = [
    Color::BLACK,
    Color::Srgba(Srgba { red: 1.0, green: 0.0, blue: 0.0, alpha: 1.0 }),
    Color::Srgba(Srgba { red: 1.0, green: 0.5, blue: 0.0, alpha: 1.0 }),
    Color::Srgba(Srgba { red: 1.0, green: 1.0, blue: 0.0, alpha: 1.0 }),
    Color::Srgba(Srgba { red: 0.0, green: 1.0, blue: 0.0, alpha: 1.0 }),
    Color::Srgba(Srgba { red: 0.0, green: 1.0, blue: 1.0, alpha: 1.0 }),
    Color::Srgba(Srgba { red: 0.0, green: 0.0, blue: 1.0, alpha: 1.0 }),
    Color::Srgba(Srgba { red: 0.8, green: 0.2, blue: 1.0, alpha: 1.0 }),
    Color::Srgba(Srgba { red: 1.0, green: 0.0, blue: 1.0, alpha: 1.0 }),
    Color::Srgba(Srgba { red: 0.5, green: 0.5, blue: 0.5, alpha: 1.0 }),
];
