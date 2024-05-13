mod content;
mod list;
mod theme;

use std::fmt::Display;

pub use self::{content::ThemeContent, theme::Theme};

impl Display for Theme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{:?}", self))
    }
}

pub fn get(theme: Theme) -> ThemeContent {
    match theme {
        Theme::Light => list::LIGHT,
        Theme::Dark => list::DARK,
        Theme::Solarised => list::SOLARISED,
        Theme::Purple => list::PURPLE,
    }
}
