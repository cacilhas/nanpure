mod consts;
mod events;
mod fonts;
mod game;
mod gameover;
mod gameplay;
mod kennett;
mod load;
mod pause;
mod states;
mod title;

use bevy::app::Plugin;
use bevy::core_pipeline::bloom::Bloom;
use bevy::core_pipeline::tonemapping::DebandDither;
use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::prelude::*;
use bevy::render::camera::SubCameraView;

use crate::consts::RESOLUTION;
use crate::events::NanpureEvent;
use crate::gameover::GameOverPlugin;
use crate::gameplay::GameplayPlugin;
use crate::load::{Ctrl, LoadPlugin};
use crate::pause::PausePlugin;
use crate::states::GameState;
use crate::title::TitleScenePlugin;

pub mod prelude {
    pub mod consts {
        pub use crate::consts::*;
    }
    pub use super::NanpureApp;
}

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
            .add_plugins(PausePlugin)
            .add_plugins(GameOverPlugin)
            .add_systems(Startup, background_system)
            .add_systems(Startup, setup_camera)
            .add_systems(Update, exit_system)
        ;
    }
}

fn background_system(mut commands: Commands) {
    commands.insert_resource(ClearColor(crate::consts::BACKGROUND_COLOR));
}

fn exit_system(
    mut keyboard: EventReader<KeyboardInput>,
    mut exit: EventWriter<AppExit>,
    mut ctrl: ResMut<Ctrl>,
) {
    let mut key_q = false;
    for (input, _) in keyboard.par_read() {
        match (&input.logical_key, input.state.is_pressed()) {
            (Key::Control, pressed) => ctrl.0 = pressed,

            (Key::Character(ch), pressed) if pressed && (ch == "q" || ch == "Q") =>
                    key_q = true,

            _ => (),
        }
    }
    if ctrl.0 && key_q {
        exit.write(AppExit::Success);
    }
}

fn setup_camera(mut commands: Commands) {
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
