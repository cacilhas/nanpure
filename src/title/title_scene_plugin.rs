use bevy::prelude::*;

use crate::states::GameState;

use super::{keybindings::keybindings_system, title_scene::TitleScene};

#[derive(Clone, Copy, Debug)]
pub struct TitleScenePlugin;

impl Plugin for TitleScenePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                OnEnter(GameState::Title),
                TitleScene::load_title_scene,
            )
            .add_systems(
                OnExit(GameState::Title),
                TitleScene::unload_title_scene,
            )
            .add_systems(
                Update,
                keybindings_system.run_if(in_state(GameState::Title)),
            );
    }
}
