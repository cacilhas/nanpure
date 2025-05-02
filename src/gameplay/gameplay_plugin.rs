use bevy::prelude::*;

use crate::states::GameState;

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
                OnEnter(GameState::Playing),
                Gameplay::must_unpause,
            )
            .add_systems(
                OnExit(GameState::Playing),
                Gameplay::unload_gameplay,
            )
            .add_systems(
                OnExit(GameState::Playing),
                Gameplay::must_pause,
            )
        ;
    }
}
