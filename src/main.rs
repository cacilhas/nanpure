use bevy::prelude::*;
use bevy::window::{
    EnabledButtons,
    ExitCondition,
    PresentMode,
    WindowTheme,
};

use bevy_render::batching::gpu_preprocessing::{
    GpuPreprocessingMode,
    GpuPreprocessingSupport,
};
use bevy_render::RenderApp;
use numples::prelude::*;

fn main() {
    let mut app = App::new();
    app
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: consts::TITLE.into(),
                    name: Some("Kodumaro-numples".to_string()),
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

        // Sub-app
        .sub_app_mut(RenderApp)
        .insert_resource(GpuPreprocessingSupport {
            max_supported_mode: GpuPreprocessingMode::None,
        })
    ;
    app.run();
}
