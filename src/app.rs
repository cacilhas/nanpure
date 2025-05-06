use bevy::app::Plugin;
use bevy::core_pipeline::bloom::Bloom;
use bevy::core_pipeline::tonemapping::DebandDither;
use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::prelude::*;
use bevy::render::camera::SubCameraView;

use crate::consts::RESOLUTION;
use crate::events::NanpureEvent;
use crate::gameplay::GameplayPlugin;
use crate::load::{Ctrl, LoadPlugin};
use crate::states::GameState;
use crate::title::TitleScenePlugin;

#[derive(Clone, Copy, Debug)]
pub struct NanpureApp;

impl Plugin for NanpureApp {
    fn build(&self, app: &mut App) {
        app
            .add_event::<NanpureEvent>()
            .init_state::<GameState>()
            .add_plugins(LoadPlugin)
            .add_plugins(TitleScenePlugin)
            .add_plugins(GameplayPlugin)
            .add_systems(Startup, background_system)
            .add_systems(Startup, setup_camera)
            .add_systems(Update, exit_system)
        ;
    }
}

fn background_system(mut commands: Commands) {
    commands.insert_resource(ClearColor(crate::consts::BACKGROUND_COLOR));
}

pub fn exit_system(
    mut keyboard: EventReader<KeyboardInput>,
    mut exit: EventWriter<AppExit>,
    mut ctrl: ResMut<Ctrl>,
) {
    let mut key_q = false;
    for (input, _) in keyboard.par_read() {
        if input.logical_key == Key::Control {
            ctrl.0 = input.state.is_pressed();
        }

        else if input.state.is_pressed() {
            if let Some(text) = &input.text {
                let text = text.as_str();
                if text == "q" || text == "Q" {
                    key_q = true;
                }
            }
        }
    }
    if ctrl.0 && key_q {
        exit.write(AppExit::Success);
    }
}

pub fn setup_camera(mut commands: Commands) {
    let size = UVec2::new(RESOLUTION.x as u32, RESOLUTION.y as u32);
    commands.spawn((
        Camera2d,
        Camera {
            sub_camera_view: Some(SubCameraView {
                full_size: size,
                offset: Vec2::new(0.0, 0.0),
                size,
            }),
            order: 1,
            ..default()
        },
        Bloom::default(),
        DebandDither::Enabled,
    ));
}
