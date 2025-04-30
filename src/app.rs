use std::borrow::BorrowMut;
use std::sync::LazyLock;

use bevy::app::Plugin;
use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;
use bevy::render::camera::SubCameraView;

use crate::consts::RESOLUTION;
use crate::load::LoadPlugin;
use crate::states::GameState;
use crate::title::TitleScenePlugin;

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
    let mut key_q = false;
    for (input, _) in keyboard.par_read() {
        if input.key_code == KeyCode::ControlLeft || input.key_code == KeyCode::ControlRight {
            unsafe {
                CTRL[0] = input.state.is_pressed();
            }
        }

        else if input.state.is_pressed() && input.key_code == KeyCode::KeyQ {
            key_q = true;
        }
    }
    if unsafe { CTRL[0] } && key_q {
        exit.write(AppExit::Success);
    }
}

static mut CTRL: [bool; 1] = [false];

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
