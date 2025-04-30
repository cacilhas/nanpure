use bevy::prelude::*;

use crate::consts::{TITLE, TITLE_COLOR};
use crate::fonts::TitleFont;
use crate::states::GameState;

#[derive(Clone, Copy, Debug)]
pub struct TitleScenePlugin;

impl Plugin for TitleScenePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Title), TitleScene::load_title_scene)
            .add_systems(OnExit(GameState::Title), TitleScene::unload_title_scene);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
struct TitleScene;

impl TitleScene {
    pub fn load_title_scene(
        mut commands: Commands,
        title_font: Res<TitleFont>,
    ) {
        commands.spawn((
            TitleScene,
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
    }

    pub fn unload_title_scene(
        mut commands: Commands,
        title_scene: Query<Entity, With<TitleScene>>,
    ) {
        for entity in &title_scene {
            commands.entity(entity).despawn();
        }
    }
}
