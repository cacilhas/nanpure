use bevy::ecs::error::Result;
use bevy::prelude::*;

use crate::game::Level;
use crate::states::GameState;

use super::paused::Paused;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct Gameplay;

impl Gameplay {
    pub fn load_gameplay(
        mut commands: Commands,
        level: Res<Level>,
        paused: Res<Paused>,
    ) -> Result<()> {
        if !paused.0 {
            // Do NOT reload gameplay when unpausing

            // TODO: create game board
            // TODO: spawn board
        }

        Ok(())
    }

    pub fn must_unpause(
        mut visibilities_query: Query<&mut Visibility, With<Self>>,
        mut paused: ResMut<Paused>,
    ) -> Result<()> {
        if paused.0 {
            paused.0 = false;
            for mut visibility in &mut visibilities_query {
                *visibility = Visibility::Visible;
            }
        }
        Ok(())
    }

    pub fn unload_gameplay(
        mut commands: Commands,
        entities: Query<Entity, With<Self>>,
        next_state: Res<NextState<GameState>>,
    ) {
        match next_state.into_inner() {
            NextState::Pending(state) if state == &GameState::Paused =>
                // Do NOT unload gameplay when pausing
                (),

            _ => {
                commands.remove_resource::<Level>();
                for entity in &entities {
                    commands.entity(entity).despawn();
                }
            }
        }
    }

    pub fn must_pause(
        mut visibilities: Query<&mut Visibility, With<Self>>,
        mut paused: ResMut<Paused>,
        next_state: Res<NextState<GameState>>,
    ) -> Result<()> {
        match next_state.into_inner() {
            NextState::Pending(state) if state == &GameState::Paused => {
                for mut visibility in &mut visibilities {
                    *visibility = Visibility::Hidden;
                }

                paused.0 = true;
            }

            _ => (),
        }
        Ok(())
    }
}
