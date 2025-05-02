use bevy::ecs::error::Result;
use bevy::prelude::*;

use crate::events::NanpureEvent;
use crate::{app::NanpureApp, game::Level};
use crate::states::GameState;

use super::paused::{MustUnpause, Paused};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct Gameplay;

impl Gameplay {
    pub fn load_gameplay(
        mut commands: Commands,
        level: Res<Level>,
        paused: Res<Paused>,
    ) -> Result<()> {
        if paused.0 {
            // Do NOT reload gameplay when unpausing
            commands.spawn(MustUnpause);

        } else {
            // TODO: create game board
            // TODO: spawn board
        }

        Ok(())
    }

    pub fn must_unpause(
        mut commands: Commands,
        must_pause_query: Query<Entity, With<MustUnpause>>,
        mut visibilities_query: Query<&mut Visibility, With<Self>>,
        mut paused: ResMut<Paused>,
    ) {
        for entity in &must_pause_query {
            commands.entity(entity).despawn();
            if paused.0 {
                paused.0 = false;
                for mut visibility in &mut visibilities_query {
                    *visibility = Visibility::Visible;
                }
            }
        }
    }

    pub fn unload_or_pause(
        mut commands: Commands,
        entities: Query<Entity, With<Self>>,
        mut visibilities: Query<&mut Visibility, With<Self>>,
        next_state: Res<NextState<GameState>>,
        mut paused: ResMut<Paused>,
    ) {
        match next_state.into_inner() {
            NextState::Pending(state) if state == &GameState::Paused => {
                // Pause

                for mut visibility in &mut visibilities {
                    *visibility = Visibility::Hidden;
                }

                paused.0 = true;
            }

            _ => {
                // Unload

                commands.remove_resource::<Level>();
                for entity in &entities {
                    commands.entity(entity).despawn();
                }
            }
        }
    }
    pub fn event_handle(
        mut events: EventReader<NanpureEvent>,
        mut next_state: ResMut<NextState<GameState>>,
    ) {
        for event in events.read() {
            match event {
                NanpureEvent::AbortGame => {
                    next_state.set(GameState::Title);
                    return;
                }

                NanpureEvent::PauseGame => {
                    next_state.set(GameState::Paused);
                    return;
                }

                _ => (),
            }
        }
    }
}
