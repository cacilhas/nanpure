use super::{candidates::Candidates, cell::Cell, position::Position, value::Value};
use rscenes::prelude::*;

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

pub fn create_cells(world: &mut World) {
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
            world.spawn((
                Cell,
                Position { x, y },
                Candidates::default(),
                Value::new(value),
            ));
        }
    }
}

pub fn create_empty_system(world: &mut World) {
    let mut entities = vec![];
    for (entity, _) in world.query_mut::<&Cell>() {
        entities.push(entity);
    }
    for entity in entities.iter() {
        world.despawn(*entity).unwrap();
    }
    for y in 0..9 {
        for x in 0..9 {
            world.spawn((
                Cell,
                Position { x, y },
                Candidates::default(),
                Value::default(),
            ));
        }
    }
}

pub fn set_cell_system(
    position: Position,
    candidates: &mut Candidates,
    value: &mut Value,
    res: &SetCell,
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
            let value_0: Option<u8> = (*value).into();
            match value_0 {
                Some(c) if c == v => value.clean(),
                _ => (),
            }
        }
        // TODO: deal with None
    }
}

pub fn toggle_candidate_system(
    position: &Position,
    candidates: &mut Candidates,
    res: &ToggleCandidate,
) {
    if position.x == res.x && position.y == res.y {
        if candidates.is_set(res.value) {
            candidates.clean(res.value);
        } else {
            candidates.set(res.value);
        }
    }
}

#[cfg(test)]
pub fn display_system(position: Position, value: Value, res: &mut [u8; 81]) {
    let index = (position.x + position.y * 9) as usize;
    let value: Option<u8> = value.into();
    res[index] = value.unwrap_or(0);
}
