use bevy::ecs::error::Result;
use bevy::input::mouse::MouseButtonInput;
use bevy::prelude::*;

use crate::consts::{SELECTED_COLOR, TITLE, TITLE_COLOR, UNSELECTED_COLOR};
use crate::events::NanpureEvent;
use crate::fonts::{RegularFont, TitleFont};
use crate::game::Level;
use crate::gameplay::{Gameplay, Paused};
use crate::states::GameState;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct TitleScene;

impl TitleScene {
    pub fn load_title_scene(
        mut commands: Commands,
        regular_font: Res<RegularFont>,
        title_font: Res<TitleFont>,
    ) {
        // Title Label
        commands.spawn((
            Self,
            Text::new(TITLE.to_string()),
            TextFont {
                font: title_font.font().clone_weak(),
                font_size: 48.0,
                ..default()
            },
            TextShadow::default(),
            TextColor(TITLE_COLOR.clone()),
            TextLayout::new_with_justify(JustifyText::Center),
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(0.0),
                width: Val::Percent(100.0),
                ..default()
            },
        ));

        let mut y = 100. / 6.;
        for level in Level::levels() {
            let level_number: u8 = level.into();
            commands.spawn((
                Self,
                level,
                Text::new(format!("{}. {}", level_number, level.to_string())),
                TextFont {
                    font: regular_font.font().clone_weak(),
                    font_size: 32.0,
                    ..default()
                },
                TextColor(UNSELECTED_COLOR.clone()),
                TextLayout::new_with_justify(JustifyText::Center),
                Node {
                    position_type: PositionType::Absolute,
                    align_items: AlignItems::Center,
                    top: Val::Percent(y),
                    width: Val::Percent(100.0),
                    height: Val::Percent(12.0),
                    ..default()
                },
            ));
            y += 100. / 6.;
        }
    }

    pub fn unload_title_scene(
        mut commands: Commands,
        entities: Query<Entity, With<Self>>,
    ) {
        for entity in &entities {
            commands.entity(entity).despawn();
        }
    }

    pub fn update(
        mut level_query: Query<(&Level, &Transform, &ComputedNode, &mut TextColor, &mut BackgroundColor), With<Self>>,
        window_query: Query<&Window>,
        mut mouse_input: EventReader<MouseButtonInput>,
        mut event_writer: EventWriter<NanpureEvent>,
    ) -> Result<()> {
        let window = window_query.single()?;
        if let Some(mouse) = window.cursor_position() {
            let y = mouse.y;
            for (&level, transform, node, mut color, mut bg) in &mut level_query {
                let half_size = node.size().y / 2.0;
                if y > transform.translation.y - half_size && y < transform.translation.y + half_size {
                    color.0 = SELECTED_COLOR.clone();
                    bg.0 = Color::Srgba(Srgba { red: 1.0, green: 1.0, blue: 1.0, alpha: 1.0 });

                    for event in mouse_input.read() {
                        if event.button == MouseButton::Left {
                            event_writer.write(NanpureEvent::StartGame(level));
                            return Ok(());
                        }
                    }

                } else {
                    color.0 = UNSELECTED_COLOR.clone();
                    bg.0 = Color::Srgba(Srgba { red: 1.0, green: 1.0, blue: 1.0, alpha: 0.0 })
                }
            }
        }
        Ok(())
    }

    pub fn event_handle(
        mut commands: Commands,
        mut events: EventReader<NanpureEvent>,
        mut next_state: ResMut<NextState<GameState>>,
        mut paused: ResMut<Paused>,
    ) {
        for event in events.read() {
            match event {
                NanpureEvent::StartGame(level) => {
                    commands.insert_resource(*level);
                    paused.0 = false;
                    next_state.set(GameState::Playing);
                }
            }
        }
    }
}
