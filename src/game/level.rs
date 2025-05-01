use bevy_ecs::component::Component;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Component)]
pub enum Level {
    ExtremelyEasy = 1,
    Easy,
    Medium,
    Hard,
    Fiendish,
}

impl From<u8> for Level {
    fn from(value: u8) -> Self {
        match value {
            0 | 1 => Level::ExtremelyEasy,
            2 => Level::Easy,
            3 => Level::Medium,
            4 => Level::Hard,
            _ => Level::Fiendish,
        }
    }
}

impl From<Level> for u8 {
    fn from(value: Level) -> Self {
        value as u8
    }
}

impl ToString for Level {
    fn to_string(&self) -> String {
        match self {
            Self::ExtremelyEasy => "Extremely Easy",
            Self::Easy => "Easy",
            Self::Medium => "Medium",
            Self::Hard => "Hard",
            Self::Fiendish => "Fiendish",
        }.to_string()
    }
}

impl Level {

    pub fn kennett_flag(&self) -> &'static str {
        match self {
            Self::ExtremelyEasy => "-cvery easy",
            Self::Easy => "-ceasy",
            Self::Medium => "-cmedium",
            Self::Hard => "-chard",
            Self::Fiendish => "-cfiendish",
        }
    }
}
