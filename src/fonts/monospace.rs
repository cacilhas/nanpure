use bevy::prelude::*;
use bevy::ecs::error::Result;

#[derive(Clone, Debug, Deref, Resource)]
pub struct MonospaceFont(Handle<Font>);

impl MonospaceFont {

    pub fn new(assets: &Res<AssetServer>) -> Result<Self> {
        let font = include_bytes!("../../assets/monospace.ttf");
        let font = Font::try_from_bytes(font.to_vec())?;
        Ok(Self(assets.add(font)))
    }

    pub fn font(&self) -> &Handle<Font> {
        &self.0
    }
}
