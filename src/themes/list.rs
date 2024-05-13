use super::{content::ThemeContent, theme::Theme};
use crate::colors;
use raylib::Color;

pub(super) static LIGHT: ThemeContent = ThemeContent {
    title: colors::DARKCYAN,
    foreground: colors::DARKGRAY,
    background: colors::WHEAT,
    hover_foreground: colors::BLACK,
    hover_background: colors::BEIGE,
    r#type: Theme::Light,
    next_theme: Theme::Dark,
};

pub(super) static DARK: ThemeContent = ThemeContent {
    title: colors::DARKCYAN,
    foreground: colors::DARKGRAY,
    background: colors::BLACK,
    hover_foreground: colors::WHITE,
    hover_background: colors::DARKBROWN,
    r#type: Theme::Dark,
    next_theme: Theme::Solarised,
};

pub(super) static SOLARISED: ThemeContent = ThemeContent {
    title: Color {
        r: 0xaf,
        g: 0x88,
        b: 0x07,
        a: 0xff,
    },
    foreground: Color {
        r: 0x42,
        g: 0x40,
        b: 0x3c,
        a: 0xff,
    },
    background: Color {
        r: 0x00,
        g: 0x2b,
        b: 0x36,
        a: 0xff,
    },
    hover_foreground: colors::WHITE,
    hover_background: Color {
        r: 0x07,
        g: 0x36,
        b: 0x42,
        a: 0xff,
    },
    r#type: Theme::Solarised,
    next_theme: Theme::Light,
};
