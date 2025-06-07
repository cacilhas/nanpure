#[derive(Debug, Clone)]
pub enum Anim {
    Set {x: usize, y: usize, value: u8},
    Unset {x: usize, y: usize, value: u8},
    SetCandidate {x: usize, y: usize, value: u8},
    UnsetCandidate {x: usize, y: usize, value: u8},
}
