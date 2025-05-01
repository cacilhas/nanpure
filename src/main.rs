use bevy::prelude::*;
use bevy::window::{
    EnabledButtons,
    PresentMode,
    WindowTheme,
};
use bevy_rand::plugin::EntropyPlugin;
use bevy_rand::prelude::WyRand;

mod app;
mod consts;
mod fonts;
mod game;
mod kennett;
mod load;
mod states;
mod title;

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
        .add_plugins(EntropyPlugin::<WyRand>::default())
        .add_plugins(app::NanpureApp)
        .run();
}
