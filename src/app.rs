use bevy::prelude::*;
use bevy_ecs::result::Result;

use crate::consts::*;
use crate::fonts::TitleFont;
use crate::main_menu::MainMenu;
use crate::state::GameState;

#[derive(Copy, Clone, Debug)]
pub struct NanpurePlugin;

impl Plugin for NanpurePlugin {

    fn build(&self, app: &mut App) {
        app
            .add_systems(Main, Self::setup)
            .init_state::<GameState>()

            .add_systems(OnEnter(GameState::MainMenu), MainMenu::spawn)
            .add_systems(OnExit(GameState::MainMenu), MainMenu::despawn)
        ;
    }
}

impl NanpurePlugin {

    fn setup(
        mut commands: Commands,
        assets: Res<AssetServer>,
    ) -> Result {
        commands.insert_resource(ClearColor(BACKGROUND_COLOR));
        commands.insert_resource(TitleFont::new(&assets)?);
        Ok(())
    }
}
