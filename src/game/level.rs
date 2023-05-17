const EXTREMELY_EASY: usize = (25 << 8) | 31;
const EASY: usize = (32 << 8) | 44;
const MEDIUM: usize = (45 << 8) | 49;
const HARD: usize = (50 << 8) | 53;
const FIENDISH: usize = (54 << 8) | 59;

#[derive(Debug)]
pub enum Level {
    ExtremelyEasy,
    Easy,
    Medium,
    Hard,
    Fiendish,
}

impl Level {
    pub(crate) fn count(&self) -> usize {
        match self {
            Self::ExtremelyEasy => EXTREMELY_EASY,
            Self::Easy => EASY,
            Self::Medium => MEDIUM,
            Self::Hard => HARD,
            Self::Fiendish => FIENDISH,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_return_extremely_easy() {
        assert_eq!(Level::ExtremelyEasy.count(), EXTREMELY_EASY);
    }

    #[test]
    fn it_should_return_easy() {
        assert_eq!(Level::Easy.count(), EASY);
    }

    #[test]
    fn it_should_return_medium() {
        assert_eq!(Level::Medium.count(), MEDIUM);
    }

    #[test]
    fn it_should_return_hard() {
        assert_eq!(Level::Hard.count(), HARD);
    }

    #[test]
    fn it_should_return_fiendish() {
        assert_eq!(Level::Fiendish.count(), FIENDISH);
    }
}
