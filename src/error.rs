use std::fmt;

#[derive(Debug, thiserror::Error)]
pub struct Error<T>(pub T)
where
    T: fmt::Display;

impl<T> fmt::Display for Error<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)+) => {
        $crate::error::Error(format!($($arg),+))
    }
}
