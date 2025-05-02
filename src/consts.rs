use bevy::prelude::*;
use bevy::math::Vec2;

pub const RESOLUTION: Vec2 = Vec2 { x: 640.0, y: 768.0 };
pub const TITLE: &str = "Kodumaro Nanpūrë";

pub const BACKGROUND_COLOR: Color = Color::srgb_u8(0xf5, 0xde, 0xb3);
pub const TITLE_COLOR: Color = Color::srgb_u8(0x00, 0x8b, 0x8b);

pub const UNSELECTED_COLOR: Color = Color::srgb_u8(0x8b, 0x8b, 0x8b);
pub const SELECTED_COLOR: Color = Color::srgb_u8(0x00, 0x00, 0x00);

pub const CELL_SIZE: f32 = RESOLUTION.x / 9.0;
pub const CANDIDATE_SIZE: f32 = CELL_SIZE / 3.0;
