use bevy::prelude::*;

#[derive(Clone, Debug, Deref, DerefMut, Resource)]
pub struct EasyMode(bool);

impl From<bool> for EasyMode {

    fn from(easy_mode_enabled: bool) -> Self {
        Self(easy_mode_enabled)
    }
}
