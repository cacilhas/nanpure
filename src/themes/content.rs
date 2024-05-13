use super::theme::Theme;
use raylib::Color;

#[derive(Clone, Copy, Debug)]
pub struct ThemeContent {
    pub title: Color,
    pub foreground: Color,
    pub background: Color,
    pub hover_foreground: Color,
    pub hover_background: Color,
    pub r#type: Theme,
    pub(super) next_theme: Theme,
}

impl ThemeContent {
    pub fn next(&self) -> ThemeContent {
        super::get(self.next_theme)
    }
}
