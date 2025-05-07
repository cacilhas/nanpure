use bevy::prelude::*;

use crate::game::Level;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Event)]
pub enum NumplesEvent {

    StartGame(Level),
    AbortGame,
    PauseGame,
    RenderBoard,
}
