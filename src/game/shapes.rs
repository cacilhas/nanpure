use bevy::prelude::*;

#[derive(Debug, Clone, Resource)]
pub struct Shapes {
    pub rect: Handle<Mesh>,
    pub cell: Handle<Mesh>,
    pub cell_candidate: Handle<Mesh>,
}
