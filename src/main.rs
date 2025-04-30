use bevy::prelude::*;
use bevy::window::{
    EnabledButtons,
    PresentMode,
    WindowTheme,
};

mod consts;
mod exit;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: consts::TITLE.into(),
                    resolution: consts::RESOLUTION.into(),
                    present_mode: PresentMode::AutoVsync,
                    prevent_default_event_handling: true,
                    window_theme: Some(WindowTheme::Light),
                    enabled_buttons: EnabledButtons {
                        minimize: true,
                        maximize: false,
                        close: true,
                    },
                    visible: true,
                    ..default()
                }),
                close_when_requested: true,
                ..default()
            })
        )
        .add_systems(Update, exit::exit_system)
        .run();
}
