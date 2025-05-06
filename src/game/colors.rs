use bevy::prelude::*;

#[derive(Debug, Clone, Resource)]
pub struct Colors(Vec<Handle<ColorMaterial>>);

impl Colors {
    pub fn new(colors: Vec<Handle<ColorMaterial>>) -> Self {
        Colors(colors)
    }

    pub fn get(&self, index: usize) -> &Handle<ColorMaterial> {
        self.0
            .get(index)
            .unwrap_or(
                self.0.get(0).expect("Colors asset is empty")
            )
    }

    pub fn black(&self) -> &Handle<ColorMaterial> {
        self.get(0)
    }

    pub fn white(&self) -> &Handle<ColorMaterial> {
        self.get(10)
    }

    pub fn background(&self) -> &Handle<ColorMaterial> {
        self.get(11)
    }

    pub fn highlight(&self) -> &Handle<ColorMaterial> {
        self.get(12)
    }

    pub fn win(&self) -> &Handle<ColorMaterial> {
        self.get(13)
    }
}
