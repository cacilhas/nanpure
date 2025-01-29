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
                position_type: PositionType::Relative,
                justify_self: JustifySelf::Center,
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
}
