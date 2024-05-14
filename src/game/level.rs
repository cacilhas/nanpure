use super::get_random_value;

const EXTREMELY_EASY: (usize, usize) = (25, 31);
const EASY: (usize, usize) = (32, 44);
const MEDIUM: (usize, usize) = (45, 49);
const HARD: (usize, usize) = (50, 53);
const FIENDISH: (usize, usize) = (54, 59);

#[derive(Clone, Copy, Debug)]
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

impl Level {
    pub fn count(&self) -> usize {
        match self {
            Self::ExtremelyEasy => get_random(EXTREMELY_EASY),
            Self::Easy => get_random(EASY),
            Self::Medium => get_random(MEDIUM),
            Self::Hard => get_random(HARD),
            Self::Fiendish => get_random(FIENDISH),
        }
    }

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

fn get_random((min, max): (usize, usize)) -> usize {
    unsafe { get_random_value::<i32>(min as i32, max as i32) as usize }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_return_extremely_easy() {
        assert!(Level::ExtremelyEasy.count() >= EXTREMELY_EASY.0);
        assert!(Level::ExtremelyEasy.count() <= EXTREMELY_EASY.1);
    }

    #[test]
    fn it_should_return_easy() {
        assert!(Level::Easy.count() >= EASY.0);
        assert!(Level::Easy.count() <= EASY.1);
    }

    #[test]
    fn it_should_return_medium() {
        assert!(Level::Medium.count() >= MEDIUM.0);
        assert!(Level::Medium.count() <= MEDIUM.1);
    }

    #[test]
    fn it_should_return_hard() {
        assert!(Level::Hard.count() >= HARD.0);
        assert!(Level::Hard.count() <= HARD.1);
    }

    #[test]
    fn it_should_return_fiendish() {
        assert!(Level::Fiendish.count() >= FIENDISH.0);
        assert!(Level::Fiendish.count() <= FIENDISH.1);
    }
}
