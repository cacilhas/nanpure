use bevy::ecs::error::Result;
use bevy::prelude::*;

use crate::consts::{
    CELL_SIZE,
    MAGICAL_AJUSTMENT_NUMBER,
    TITLE_COLOR,
};
use crate::events::NumplesEvent;
use crate::fonts::{MonospaceFont, RegularFont};
use crate::game::{
    BoardCell,
    Board,
    Colors,
    Cursor,
    ErrorCell,
    Level,
    Shapes,
};
use crate::gameover::GameOverCheck;
use crate::states::GameState;

use super::clock::ClockDisplay;
use super::paused::{MustUnpause, Paused};
use super::background::BGFlag;
use super::clock::Clock;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct Gameplay;

impl Gameplay {
    pub fn update(
        board_query: Query<&Board>,
        window_query: Query<&Window>,
        mut clock_query: Query<&mut Text, With<ClockDisplay>>,
        mut cursor_query: Query<&mut Transform, With<Cursor>>,
        mut error_cells_query: Query<&mut MeshMaterial2d<ColorMaterial>, With<ErrorCell>>,
        mut event_writer: EventWriter<NumplesEvent>,
        time: Res<Time>,
        colors: Res<Colors>,
        mut clock: ResMut<Clock>,
    ) -> Result<()> {
        let window = window_query.single()?;
        if !window.focused {
            event_writer.write(NumplesEvent::PauseGame);
        }
        clock.update(&time);
        if let Ok(mut display) = clock_query.single_mut() {
            display.0 = clock.to_string();
        }
        let board = board_query.single()?;
        board.update(&mut cursor_query)?;
        for mut material in &mut error_cells_query {
            let elapsed = time.elapsed_secs_f64() % 0.125;
            material.0 = colors.get(
                if elapsed < 0.0625 {
                    1
                } else {
                    3
                }
            ).clone_weak();
        }
        Ok(())
    }

    pub fn load_gameplay(
        mut commands: Commands,
        level: Res<Level>,
        paused: Res<Paused>,
        shapes: Res<Shapes>,
        colors: Res<Colors>,
        regular_font: Res<RegularFont>,
        mut clock: ResMut<Clock>,
        monospace_font: Res<MonospaceFont>,
        mut event_writer: EventWriter<NumplesEvent>,
    ) -> Result<()> {
        if paused.0 {
            // Do NOT reload gameplay when unpausing
            commands.spawn(MustUnpause);
            return Ok(());
        }

        clock.reset();
        commands.spawn((
            Self,
            BGFlag,
            Mesh2d(shapes.full_bg_rect.clone_weak()),
            MeshMaterial2d(colors.black().clone_weak()),
            Transform::from_xyz(0.0, 0.0, -10.0),
        ));

        commands.spawn((
            Self,
            Text::new(level.to_string()),
            TextFont {
                font: regular_font.font().clone_weak(),
                font_size: 32.0,
                ..default()
            },
            TextColor(TITLE_COLOR.clone()),
            TextLayout::new_with_justify(JustifyText::Left),
            Node {
                position_type: PositionType::Absolute,
                align_items: AlignItems::Center,
                left: Val::Px(8.0),
                bottom: Val::Px(8.0),
                ..default()
            },
        ));

        for y in 0..3 {
            for x in 0..3 {
                commands.spawn((
                    Self,
                    BGFlag,
                    Mesh2d(shapes.rect.clone_weak()),
                    MeshMaterial2d(colors.white().clone_weak()),
                    Transform {
                        scale: Vec3 { x: 3.0, y: 3.0, z: 1.0 },
                        translation: Vec3 {
                            x: (x as f32 - 1.0) * CELL_SIZE * 3.0,
                            y: (y as f32 - 1.0) * CELL_SIZE * 3.0 + MAGICAL_AJUSTMENT_NUMBER,
                            z: -6.0,
                        },
                        ..default()
                    },
                ));
            }
        }

        for y in 0..9 {
            for x in 0..9 {
                commands.spawn((
                    Self,
                    BGFlag,
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

        let board: Board = (*level.into_inner()).try_into()?;

        for y in 0..4 {
            commands.spawn((
                Self,
                BGFlag,
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
                Self,
                BGFlag,
                Mesh2d(shapes.vertical_line.clone_weak()),
                MeshMaterial2d(colors.get(0).clone_weak()),
                Transform::from_xyz(
                    CELL_SIZE * 3.0 * x as f32 - board.size()?.x / 2.0,
                    MAGICAL_AJUSTMENT_NUMBER,
                    -1.0,
                ),
            ));
        }

        commands.spawn((Self, board));

        commands.spawn((
            Self,
            ClockDisplay,
            Text::new(clock.to_string()),
            TextFont {
                font: monospace_font.font().clone_weak(),
                font_size: 32.0,
                ..default()
            },
            TextColor(TITLE_COLOR.clone()),
            TextLayout::new_with_justify(JustifyText::Center),
            Node {
                position_type: PositionType::Absolute,
                right: Val::Px(8.0),
                bottom: Val::Px(16.0),
                height: Val::Px(32.0),
                ..default()
            },
        ));

        event_writer.write(NumplesEvent::RenderBoard);

        Ok(())
    }

    pub fn must_unpause(
        mut commands: Commands,
        must_pause_query: Query<Entity, With<MustUnpause>>,
        mut visibilities_query: Query<&mut Visibility, With<Self>>,
        mut paused: ResMut<Paused>,
    ) {
        if let Ok(entity) = must_pause_query.single() {
            commands.entity(entity).despawn();
            paused.0 = false;
            for mut visibility in &mut visibilities_query {
                *visibility = Visibility::Visible;
            }
        }
    }

    pub fn unload_or_pause(
        mut commands: Commands,
        entities: Query<Entity, With<Self>>,
        mut visibilities: Query<&mut Visibility, With<Self>>,
        paused: Res<Paused>,
        game_over: Res<GameOverCheck>,
    ) {
        if paused.0 {
            // Pause
            for mut visibility in &mut visibilities {
                *visibility = Visibility::Hidden;
            }
        } else if !game_over.0 {
            // Unload
            for entity in &entities {
                commands.entity(entity).despawn();
            }
        }
    }

    pub fn event_handle(
        mut commands: Commands,
        mut board_query: Query<&mut Board>,
        cell_query: Query<Entity, With<BoardCell>>,
        cursor_query: Query<Entity, With<Cursor>>,
        mut events: EventReader<NumplesEvent>,
        mut next_state: ResMut<NextState<GameState>>,
        mut game_over: ResMut<GameOverCheck>,
        mut paused: ResMut<Paused>,
        shapes: Res<Shapes>,
        colors: Res<Colors>,
    ) -> Result<()> {
        for event in events.read() {
            match event {
                NumplesEvent::AbortGame => {
                    next_state.set(GameState::Title);
                    return Ok(());
                }

                NumplesEvent::PauseGame => {
                    paused.0 = true;
                    next_state.set(GameState::Paused);
                    return Ok(());
                }

                NumplesEvent::RenderBoard => {
                    let mut board = board_query.single_mut()?;
                    board.render(
                        0.0,
                        MAGICAL_AJUSTMENT_NUMBER,
                        &mut commands,
                        &cell_query,
                        &cursor_query,
                        &shapes,
                        &colors,
                    )?;
                    if board.is_done()? {
                        game_over.0 = true;
                        next_state.set(GameState::GameOver);
                    }
                    return Ok(());
                }

                _ => (),
            }
        }
        Ok(())
    }
}
