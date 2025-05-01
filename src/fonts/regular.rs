use bevy::prelude::*;
use bevy::ecs::error::Result;

#[derive(Clone, Debug, Deref, Resource)]
pub struct RegularFont(Handle<Font>);

impl RegularFont {

    pub fn new(assets: &Res<AssetServer>) -> Result<Self> {
        let font = include_bytes!("../../assets/regular.ttf");
        let font = Font::try_from_bytes(font.to_vec())?;
        Ok(Self(assets.add(font)))
    }

    pub fn font(&self) -> &Handle<Font> {
        &self.0
    }
}
