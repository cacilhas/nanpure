use bevy::prelude::*;

#[derive(Debug, Resource)]
pub struct Paused(pub bool);

#[derive(Copy, Clone, Debug, Component)]
pub struct MustUnpause;
