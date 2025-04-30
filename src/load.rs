use bevy::prelude::*;
use bevy::ecs::error::Result;

use crate::fonts::TitleFont;
use crate::states::GameState;

#[derive(Clone, Copy, Debug)]
pub struct LoadPlugin;

impl Plugin for LoadPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Loading), Load::init)
            .add_systems(Update, Load::load_title.run_if(in_state(GameState::Loading)));
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
struct Load;

impl Load {
    pub fn init(
        mut commands: Commands,
        assets: Res<AssetServer>,
    ) -> Result<()> {
        let font = TitleFont::new(&assets)?;
        commands.insert_resource(font);
        Ok(())
    }

    pub fn load_title(mut next_state: ResMut<NextState<GameState>>) {
        next_state.set(GameState::Title);
    }
}
