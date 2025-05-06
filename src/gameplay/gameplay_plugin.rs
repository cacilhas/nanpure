use bevy::prelude::*;

use crate::states::GameState;

use super::keybindings::keybindings_system;
use super::mouse::mouse_system;
use super::Gameplay;

#[derive(Clone, Copy, Debug)]
pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                OnEnter(GameState::Playing),
                Gameplay::load_gameplay,
            )
            .add_systems(
                Update,
                keybindings_system.run_if(in_state(GameState::Playing)),
            )
            .add_systems(
                Update,
                mouse_system.run_if(in_state(GameState::Playing)),
            )
            .add_systems(
                Update,
                Gameplay::must_unpause.run_if(in_state(GameState::Playing)),
            )
            .add_systems(
                Update,
                Gameplay::event_handle.run_if(in_state(GameState::Playing)),
            )
            .add_systems(
                Update,
                Gameplay::update.run_if(in_state(GameState::Playing)),
            )
            .add_systems(
                OnExit(GameState::Playing),
                Gameplay::unload_or_pause,
            )
        ;
    }
}
