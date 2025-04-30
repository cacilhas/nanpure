use bevy::prelude::*;

mod consts;
mod exit;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, exit::exit_system)
        .run();
}
