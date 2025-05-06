use bevy::prelude::*;

use crate::game::Level;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Event)]
pub enum NanpureEvent {

    StartGame(Level),
    AbortGame,
    PauseGame,
    RenderBoard,
}
