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
use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::prelude::*;

use crate::events::NumplesEvent;
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
    pub use super::NumplesApp;
}

#[derive(Clone, Copy, Debug)]
pub struct NumplesApp;

impl Plugin for NumplesApp {
    fn build(&self, app: &mut App) {
        app
            .add_event::<NumplesEvent>()
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
    commands.spawn(Camera2d);
}
