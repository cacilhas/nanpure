use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum Theme {
    Light,
    Dark,
    Solarised,
    Purple,
}
