mod app;
mod consts;
mod easy;
mod fonts;
mod game;
mod main_menu;
mod state;

use app::NanpurePlugin;
use bevy::prelude::*;
use consts::*;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(WindowPlugin{
                primary_window: Some(Window {
                    title: TITLE.to_string(),
                    resolution: RESOLUTION.into(),
                    position: WindowPosition::Centered(MonitorSelection::Current),
                    resizable: false,
                    decorations: true,
                    focused: true,
                    ..default()
                }),
                close_when_requested: true,
                ..default()
            })
        )
        .add_plugins(NanpurePlugin)
        .run();
}
