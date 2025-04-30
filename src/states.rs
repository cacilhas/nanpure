use bevy_state::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Default, States)]
pub enum GameState {
    #[default]
    Loading,
    Title,
    Playing,
    Paused,
    GameOver,
}
