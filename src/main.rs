use bevy::prelude::*;
use bevy::window::{
    EnabledButtons,
    ExitCondition,
    PresentMode,
    WindowTheme,
};

use numples::prelude::*;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: consts::TITLE.into(),
                    name: Some("kodumaro-numples".to_string()),
                    resolution: consts::RESOLUTION.into(),
                    position: WindowPosition::Centered(MonitorSelection::Current),
                    present_mode: PresentMode::AutoVsync,
                    prevent_default_event_handling: true,
                    window_theme: Some(WindowTheme::Light),
                    enabled_buttons: EnabledButtons {
                        minimize: true,
                        maximize: false,
                        close: true,
                    },
                    resize_constraints: WindowResizeConstraints {
                        min_width: consts::RESOLUTION.x,
                        min_height: consts::RESOLUTION.y,
                        max_width: consts::RESOLUTION.x,
                        max_height: consts::RESOLUTION.y,
                    },
                    ..default()
                }),
                exit_condition: ExitCondition::OnPrimaryClosed,
                ..default()
            })
        )
        .add_plugins(NumplesApp)
        .run();
}
