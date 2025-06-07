#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Anim {
    Set {x: usize, y: usize, value: u8},
    Unset {x: usize, y: usize, value: u8},
    SetCandidate {x: usize, y: usize, value: u8},
    UnsetCandidate {x: usize, y: usize, value: u8},
}

impl Anim {

    pub fn is_candidate(&self) -> bool {
        matches!(self, Self::SetCandidate { .. }) || matches!(self, Self::UnsetCandidate { .. })
    }

    pub fn is_set(&self) -> bool {
        matches!(self, Self::Set { .. }) || matches!(self, Self::SetCandidate { .. })
    }

    pub fn is_at(&self, x: usize, y: usize) -> bool {
        let (tx, ty) = self.xy();
        tx == x && ty == y
    }

    pub fn xy(&self) -> (usize, usize) {
        match self {
            Self::Set {x, y, ..} => (*x, *y),
            Self::Unset {x, y, ..} => (*x, *y),
            Self::SetCandidate {x, y, ..} => (*x, *y),
            Self::UnsetCandidate {x, y, ..} => (*x, *y),
        }
    }

    pub fn value(&self) -> u8 {
        match self {
            Self::Set {value, ..} => *value,
            Self::Unset {value, ..} => *value,
            Self::SetCandidate {value, ..} => *value,
            Self::UnsetCandidate {value, ..} => *value,
        }
    }
}
