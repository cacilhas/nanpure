use bevy::app::Plugin;
use bevy::input::{
    keyboard::KeyboardInput,
    ButtonState,
};
use bevy::prelude::*;

use crate::fonts::TitleFont;

pub struct NanpureApp;

impl Plugin for NanpureApp {
    fn build(&self, app: &mut App) {
        app
            .add_systems(PreStartup, TitleFont::init)
            .add_systems(Startup, background_system)
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
