use bevy::prelude::*;
use bevy::math::Vec2;

pub const MAGICAL_AJUSTMENT_NUMBER: f32 = 32.0;

pub const CELL_SIZE: f32 = 90.0;
pub const CANDIDATE_SIZE: f32 = CELL_SIZE / 3.0;

pub const RESOLUTION: Vec2 = Vec2 {
    x: CELL_SIZE * 10.0 + 10.0,
    y: CELL_SIZE * 10.0 + 10.0 + MAGICAL_AJUSTMENT_NUMBER,
};
pub const TITLE: &str = "Kodumaro Numples";

pub const BACKGROUND_COLOR: Color = Color::srgb_u8(0xf5, 0xde, 0xb3);
pub const WIN_COLOR: Color = Color::srgb_u8(0x00, 0x2a, 0x35);
pub const TITLE_COLOR: Color = Color::srgb_u8(0x00, 0x8b, 0x8b);

pub const UNSELECTED_COLOR: Color = Color::srgb_u8(0x8b, 0x8b, 0x8b);
pub const SELECTED_COLOR: Color = Color::srgb_u8(0x00, 0x00, 0x00);
