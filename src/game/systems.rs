use super::{candidates::Candidates, cell::Cell, position::Position, value::Value};
use legion::{systems::CommandBuffer, *};

#[derive(Clone, Copy, Debug)]
pub struct IsGameOver(bool);

impl From<bool> for IsGameOver {
    fn from(value: bool) -> Self {
        IsGameOver(value)
    }
}

impl From<IsGameOver> for bool {
    fn from(value: IsGameOver) -> Self {
        value.0
    }
}

#[derive(Debug)]
pub struct SetCell {
    pub x: u8,
    pub y: u8,
    pub value: Option<u8>,
}

#[derive(Debug)]
pub struct ToggleCandidate {
    pub x: u8,
    pub y: u8,
    pub value: u8,
}

#[system]
pub fn create_cells(cmd: &mut CommandBuffer) {
    for y in 0..9 {
        for x in 0..9 {
            let value = match y {
                1 => x + 3,
                2 => x + 6,
                3 => x + 1,
                4 => x + 4,
                5 => x + 7,
                6 => x + 2,
                7 => x + 5,
                8 => x + 8,
                _ => x,
            };
            let value = (value % 9) + 1;
            cmd.push((
                Cell,
                Position { x, y },
                Candidates::default(),
                Value::new(value),
            ));
        }
    }
}

#[system(for_each)]
pub fn game_over(_: &Cell, value: &Value, #[resource] res: &mut IsGameOver) {
    if value.is_none() {
        res.0 = false;
    }
}

#[system(for_each)]
pub fn set_cell(
    _: &Cell,
    position: &Position,
    candidates: &mut Candidates,
    value: &mut Value,
    #[resource] res: &SetCell,
) {
    if position.x == res.x && position.y == res.y {
        match res.value {
            Some(v) => value.insert(v),
            None => value.clean(),
        }
    } else if position.x == res.x
        || position.y == res.y
        || (position.x / 3 == res.x / 3 && position.y / 3 == res.y / 3)
    {
        if let Some(v) = res.value {
            candidates.clean(v);
            let value_0: Option<u8> = value.into();
            match value_0 {
                Some(c) if c == v => value.clean(),
                _ => (),
            }
        }
        // TODO: deal with None
    }
}

#[system(for_each)]
pub fn toggle_candidate(
    _: &Cell,
    position: &Position,
    candidates: &mut Candidates,
    #[resource] res: &ToggleCandidate,
) {
    if position.x == res.x && position.y == res.y {
        if candidates.is_set(res.value) {
            candidates.clean(res.value);
        } else {
            candidates.set(res.value);
        }
    }
}

#[system(for_each)]
pub fn display(_: &Cell, position: &Position, value: &Value, #[resource] res: &mut [u8; 81]) {
    let index = (position.x + position.y * 9) as usize;
    let value: Option<u8> = value.into();
    res[index] = value.unwrap_or(0);
}
