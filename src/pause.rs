use bevy::{input::keyboard::{Key, KeyboardInput}, prelude::*};

use crate::consts::TITLE_COLOR;
use crate::fonts::RegularFont;
use crate::states::GameState;

#[derive(Debug, Clone, Copy)]
pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Paused), PauseState::load)
            .add_systems(OnExit(GameState::Paused), PauseState::unload)
            .add_systems(Update, PauseState::keybinding.run_if(in_state(GameState::Paused)))
        ;
    }
}

#[derive(Debug, Clone, Copy, Component)]
struct PauseState;

impl PauseState {

    fn load(
        mut commands: Commands,
        regular_font: Res<RegularFont>,
    ) {
        commands.spawn((
            Self,
            Text::new("Paused"),
            TextFont {
                font: regular_font.font().clone_weak(),
                font_size: 48.0,
                ..default()
            },
            TextShadow::default(),
            TextColor(TITLE_COLOR.clone()),
            TextLayout::new_with_justify(JustifyText::Center),
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                top: Val::Percent(50.0),
                ..default()
            },
        ));
    }

    fn unload(
        mut commands: Commands,
        query: Query<Entity, With<Self>>,
    ) {
        for entity in query.iter() {
            commands.entity(entity).despawn();
        }
    }

    fn keybinding(
        mut keyboard: EventReader<KeyboardInput>,
        mut next_state: ResMut<NextState<GameState>>,
    ) {
        for (input, _) in keyboard.par_read() {
            if input.state.is_pressed() && !input.repeat {
                match input.logical_key {
                    Key::Escape |
                    Key::Pause => {
                        next_state.set(GameState::Playing);
                        return;
                    },
                    _ => (),
                }
            }
        }
    }
}
