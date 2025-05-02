use bevy::prelude::*;
use bevy::ecs::error::Result;

use crate::fonts::{RegularFont, TitleFont};
use crate::gameplay::Paused;
use crate::states::GameState;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct Load;

impl Load {
    pub fn init(
        mut commands: Commands,
        assets: Res<AssetServer>,
    ) -> Result<()> {
        let font = TitleFont::new(&assets)?;
        commands.insert_resource(font);

        let font = RegularFont::new(&assets)?;
        commands.insert_resource(font);

        commands.insert_resource(Paused(false));

        Ok(())
    }

    pub fn load_title(mut next_state: ResMut<NextState<GameState>>) {
        next_state.set(GameState::Title);
    }
}
