use std::fmt::{Display, Write};

use crate::colors;
use raylib::Color;

#[derive(Clone, Copy, Debug)]
pub struct ThemeContent {
    pub title: Color,
    pub foreground: Color,
    pub background: Color,
    pub hover_foreground: Color,
    pub hover_background: Color,
    pub r#type: Theme,
    next_theme: Theme,
}

impl ThemeContent {
    pub fn next(&self) -> ThemeContent {
        get(self.next_theme)
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Theme {
    Light,
    Dark,
}

impl Display for Theme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{:?}", self))
    }
}

pub fn get(theme: Theme) -> ThemeContent {
    match theme {
        Theme::Light => LIGHT,
        Theme::Dark => DARK,
    }
}

static LIGHT: ThemeContent = ThemeContent {
    title: colors::DARKCYAN,
    foreground: colors::DARKGRAY,
    background: colors::WHEAT,
    hover_foreground: colors::BLACK,
    hover_background: colors::BEIGE,
    r#type: Theme::Light,
    next_theme: Theme::Dark,
};

static DARK: ThemeContent = ThemeContent {
    title: colors::DARKCYAN,
    foreground: colors::DARKGRAY,
    background: colors::BLACK,
    hover_foreground: colors::WHITE,
    hover_background: colors::DARKBROWN,
    r#type: Theme::Dark,
    next_theme: Theme::Light,
};
