use bevy::prelude::*;

use crate::states::GameState;

use super::load::Load;

#[derive(Clone, Copy, Debug)]
pub struct LoadPlugin;

impl Plugin for LoadPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                OnEnter(GameState::Loading),
                Load::init,
            )
            .add_systems(
                Update,
                Load::load_title.run_if(in_state(GameState::Loading)),
            )
        ;
    }
}
