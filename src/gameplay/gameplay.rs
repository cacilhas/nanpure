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
        level_query: Query<&Level, With<Self>>,
        paused_query: Query<&Paused, With<Self>>,
        paused_clean_query: Query<Entity, With<Paused>>,
    ) -> Result<()> {
        if paused_query.single().unwrap_or_else(|_| &Paused(false)).0 {
            // Do NOT reload gameplay when unpausing
            return Ok(());
        }

        // Clean up paused entities if they exist (it should always be one or none)
        for entity in paused_clean_query.iter() {
            commands.entity(entity).despawn();
        }

        let level = level_query.single()?;
        // TODO: create game board

        // TODO: spawn board
        commands.spawn((Paused(false), Self));

        Ok(())
    }

    pub fn must_unpause(
        mut paused_query: Query<&mut Paused, With<Self>>,
        mut visibilities_query: Query<&mut Visibility, With<Self>>,
    ) -> Result<()> {
        if let Ok(mut paused) = paused_query.single_mut() {
            if paused.0 {
                paused.0 = false;
                for mut visibility in &mut visibilities_query {
                    *visibility = Visibility::Visible;
                }
            }
        }
        Ok(())
    }

    pub fn unload_gameplay(
        mut commands: Commands,
        entities: Query<Entity, With<Self>>,
        next_state: Res<NextState<GameState>>,
    ) {
        let paused = match next_state.into_inner() {
            NextState::Pending(state) => state == &GameState::Paused,
            _ => false,
        };
        if paused {
            // Do NOT unload gameplay when pausing
            return;
        }

        for entity in &entities {
            commands.entity(entity).despawn();
        }
    }

    pub fn must_pause(
        mut visibilities: Query<&mut Visibility, With<Self>>,
        mut paused_query: Query<&mut Paused, With<Self>>,
        next_state: Res<NextState<GameState>>,
    ) -> Result<()> {
        let paused = match next_state.into_inner() {
            NextState::Pending(state) => state == &GameState::Paused,
            _ => false,
        };
        if !paused {
            return Ok(());
        }

        for mut visibility in &mut visibilities {
            *visibility = Visibility::Hidden;
        }

        let mut paused = paused_query.single_mut()?;
        paused.0 = true;
        Ok(())
    }
}
