use super::{content::ThemeContent, theme::Theme};
use crate::colors;

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
    next_theme: Theme::Light,
};
