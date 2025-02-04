use bevy::prelude::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, States)]
pub enum GameState {
    #[default]
    Loading,
    MainMenu,
    InGame,
    Paused,
}
