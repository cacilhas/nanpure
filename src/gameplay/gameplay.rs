use bevy::ecs::error::Result;
use bevy::prelude::*;

use crate::consts::CELL_SIZE;
use crate::consts::MAGICAL_AJUSTMENT_NUMBER;
use crate::events::NanpureEvent;
use crate::game::BoardCell;
use crate::game::BoardWrapper;
use crate::game::Colors;
use crate::game::Cursor;
use crate::game::Level;
use crate::game::Shapes;
use crate::states::GameState;

use super::paused::{MustUnpause, Paused};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct Gameplay;

impl Gameplay {
    pub fn update(
        board_query: Query<&BoardWrapper>,
        mut cursor_query: Query<&mut Transform, With<Cursor>>,
    ) -> Result<()> {
        let board = board_query.single()?;
        board.update(&mut cursor_query)?;
        Ok(())
    }

    pub fn load_gameplay(
        mut commands: Commands,
        level: Res<Level>,
        paused: Res<Paused>,
        cell_query: Query<Entity, With<BoardCell>>,
        cursor_query: Query<Entity, With<Cursor>>,
        mut shapes: ResMut<Shapes>,
        mut colors: ResMut<Colors>,
    ) -> Result<()> {
        if paused.0 {
            // Do NOT reload gameplay when unpausing
            commands.spawn(MustUnpause);

        } else {
            commands.spawn((
                Gameplay,
                Mesh2d(shapes.full_bg_rect.clone_weak()),
                MeshMaterial2d(colors.white().clone_weak()),
                Transform::from_xyz(0.0, 32.0, -10.0),
            ));

            for y in 0..9 {
                for x in 0..9 {
                    commands.spawn((
                        Gameplay,
                        Mesh2d(shapes.rect.clone_weak()),
                        MeshMaterial2d(colors.background().clone_weak()),
                        Transform {
                            scale: Vec3 { x: 0.9, y: 0.9, z: 1.0 },
                            translation: Vec3 {
                                x: (x as f32 - 4.0) * CELL_SIZE,
                                y: (y as f32 - 4.0) * CELL_SIZE + MAGICAL_AJUSTMENT_NUMBER,
                                z: -5.0,
                            },
                            ..default()
                        },
                    ));
                }
            }

            let mut board = BoardWrapper::default();
            board.add((*level.into_inner()).try_into()?);

            for y in 0..4 {
                commands.spawn((
                    Gameplay,
                    Mesh2d(shapes.horizontal_line.clone_weak()),
                    MeshMaterial2d(colors.get(0).clone_weak()),
                    Transform::from_xyz(
                        0.0,
                        CELL_SIZE * 3.0 * y as f32 - board.size()?.y / 2.0 + MAGICAL_AJUSTMENT_NUMBER,
                        -1.0,
                    ),
                ));
            }

            for x in 0..4 {
                commands.spawn((
                    Gameplay,
                    Mesh2d(shapes.vertical_line.clone_weak()),
                    MeshMaterial2d(colors.get(0).clone_weak()),
                    Transform::from_xyz(
                        CELL_SIZE * 3.0 * x as f32 - board.size()?.x / 2.0,
                        MAGICAL_AJUSTMENT_NUMBER,
                        -1.0,
                    ),
                ));
            }

            board.render(
                0.0,
                32.0,
                &mut commands,
                &cell_query,
                &cursor_query,
                &mut shapes,
                &mut colors,
            )?;
            commands.spawn((Gameplay, board));
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
