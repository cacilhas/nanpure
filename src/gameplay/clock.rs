use std::time::Duration;

use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Component)]
pub struct ClockDisplay;

#[derive(Clone, Debug, Default, Resource)]
pub struct Clock(pub Duration);

impl Clock {

    pub fn reset(&mut self) {
        self.0 = Duration::ZERO;
    }

    pub fn update(&mut self, time: &Time) {
        self.0 += time.delta();
    }
}

impl ToString for Clock {
    fn to_string(&self) -> String {
        let seconds = self.0.as_secs();
        format!("{:02}:{:02}", seconds / 60, seconds % 60)
    }
}
