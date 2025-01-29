use bevy::prelude::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, States)]
pub enum GameState {
    #[default]
    MainMenu,
    InGame,
    Paused,
}
