use bevy::prelude::*;

#[derive(Debug, Clone, Resource)]
pub struct Shapes {
    pub rect: Handle<Mesh>,
    pub full_bg_rect: Handle<Mesh>,
    pub cell: Handle<Mesh>,
    pub cell_candidate: Handle<Mesh>,

    pub vertical_line: Handle<Mesh>,
    pub horizontal_line: Handle<Mesh>,
}
