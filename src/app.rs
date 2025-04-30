use bevy::app::Plugin;
use bevy::input::{
    keyboard::KeyboardInput,
    ButtonState,
};
use bevy::prelude::*;
use bevy::render::camera::SubCameraView;

use crate::consts::RESOLUTION;
use crate::load::LoadPlugin;
use crate::states::GameState;
use crate::title_scene::TitleScenePlugin;

#[derive(Clone, Copy, Debug)]
pub struct NanpureApp;

impl Plugin for NanpureApp {
    fn build(&self, app: &mut App) {
        app
            .init_state::<GameState>()
            .add_plugins(LoadPlugin)
            .add_plugins(TitleScenePlugin)
            .add_systems(Startup, background_system)
            .add_systems(Startup, setup_camera)
            .add_systems(Update, exit_system);
    }
}

fn background_system(mut commands: Commands) {
    commands.insert_resource(ClearColor(crate::consts::BACKGROUND_COLOR));
}

pub fn exit_system(
    mut keyboard: EventReader<KeyboardInput>,
    mut exit: EventWriter<AppExit>,
) {
    for (input, _) in keyboard.par_read() {
        if input.state == ButtonState::Pressed && input.key_code == KeyCode::KeyQ {
            exit.write(AppExit::Success);
        }
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
    ));
}
