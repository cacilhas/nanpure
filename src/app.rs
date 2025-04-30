use bevy::app::Plugin;
use bevy::prelude::*;

use crate::fonts::TitleFont;

pub struct NanpureApp;

impl Plugin for NanpureApp {
    fn build(&self, app: &mut App) {
        app
            .add_systems(PreStartup, TitleFont::init)
            .add_systems(Update, crate::exit::exit_system);
    }
}
