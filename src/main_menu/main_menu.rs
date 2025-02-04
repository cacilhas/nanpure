use bevy::input::ButtonState;
use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::prelude::*;

use crate::consts::*;
use crate::fonts::TitleFont;

#[derive(Clone, Copy, Debug, Component)]
pub struct MainMenu;

impl MainMenu {

    pub fn spawn(
        mut commands: Commands,
        font: Res<TitleFont>,
    ) {
        commands
            .spawn(Self)
            .insert(Text::new(TITLE))
            .insert(TextFont {
                font: (**font).clone(),
                font_size: 120.0,
                ..default()
            })
            .insert(TextLayout::new_with_justify(JustifyText::Center))
            .insert(TextColor(TITLE_COLOR))
            .insert(Node {
                position_type: PositionType::Absolute,
                justify_self: JustifySelf::Center,
                top: Val::Percent(5.0),
                ..default()
            })
        ;
    }

    pub fn despawn(
        mut commands: Commands,
        query: Query<Entity, With<Self>>,
    ) {
        if let Ok(entity) = query.get_single() {
            commands.entity(entity).despawn();
        }
    }

    pub fn check_quit(
        mut reader: EventReader<KeyboardInput>,
        mut quit: EventWriter<AppExit>,
    ) {
        for evt in reader.read() {
            if evt.state == ButtonState::Released && evt.logical_key == Key::Escape {
                quit.send(AppExit::Success);
            }
        }
    }
}
