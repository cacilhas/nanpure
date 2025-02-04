use bevy::prelude::*;
use bevy_ecs::result::Result;

use crate::consts::*;
use crate::easy::EasyMode;
use crate::fonts::TitleFont;
use crate::main_menu::MainMenu;
use crate::state::GameState;

#[derive(Copy, Clone, Debug)]
pub struct NanpurePlugin;

impl Plugin for NanpurePlugin {

    fn build(&self, app: &mut App) {
        app
            .init_state::<GameState>()
            .add_systems(Startup, Self::setup)

            // Main menu
            .add_systems(OnEnter(GameState::MainMenu), MainMenu::spawn)
            .add_systems(OnExit(GameState::MainMenu), MainMenu::despawn)
            .add_systems(Update, MainMenu::check_quit.run_if(in_state(GameState::MainMenu)))
        ;
    }
}

impl NanpurePlugin {

    fn setup(
        mut commands: Commands,
        assets: Res<AssetServer>,
    ) -> Result {
        let easy_mode: EasyMode = false.into();
        commands.insert_resource(ClearColor(BACKGROUND_COLOR));
        commands.insert_resource(easy_mode);
        commands.insert_resource(TitleFont::new(&assets)?);
        // Grant OnEnter(GameState::MainMenu) systems are called only after NanpurePlugin::setup
        commands.set_state(GameState::MainMenu);
        Ok(())
    }
}
