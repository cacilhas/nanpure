mod level;

pub use self::level::Level;
use legion::{systems::CommandBuffer, *};

#[derive(Debug)]
pub struct Game(World);

impl Game {
    pub fn shuffle(&mut self, level: &Level) {
        self.shuffle_x();
        self.shuffle_y();
        self.shuffle_x_group();
        self.shuffle_y_group();
        self.hide_cells(level.count())
    }

    pub fn is_game_over(&mut self) -> bool {
        let mut resources = Resources::default();
        resources.insert(IsGameOver(true));
        Schedule::builder()
            .add_system(game_over_system())
            .build()
            .execute(&mut self.0, &mut resources);
        let res = match resources.get_mut::<IsGameOver>() {
            Some(value) => value.0,
            None => false,
        };
        res
    }

    pub fn set_cell(&mut self, x: u8, y: u8, value: Option<u8>) {
        let mut query = <(&Cell, &Position, &Candidates, &Value)>::query();
        for (_, position, candidates, current) in query.iter(&self.0) {
            if position.x == x && position.y == y {
                if let Some(value) = value {
                    if current.is_some() || !candidates.is_set(value) {
                        return;
                    }
                }
                break;
            }
        }

        let mut resources = Resources::default();
        resources.insert(SetCell { x, y, value });
        Schedule::builder()
            .add_system(set_cell_system())
            .build()
            .execute(&mut self.0, &mut resources);
    }

    pub fn toggle_candidate(&mut self, x: u8, y: u8, value: u8) {
        let mut query = <(&Cell, &Position, &Value)>::query();
        for (_, position, current) in query.iter(&self.0) {
            if position.x == x && position.y == y {
                if current.is_some() {
                    return;
                }
                break;
            }
        }

        let mut resources = Resources::default();
        resources.insert(ToggleCandidate { x, y, value });
        Schedule::builder()
            .add_system(toggle_candidate_system())
            .build()
            .execute(&mut self.0, &mut resources);
    }

    fn shuffle_x(&mut self) {
        for _ in 0..10 {
            let g = get_random_value::<u8>(0, 2);
            let x1 = get_random_value::<u8>(0, 2);
            let x2 = (x1 + get_random_value::<u8>(1, 2)) % 3;
            let x1 = g * 3 + x1;
            let x2 = g * 3 + x2;
            let mut query = <(&Cell, &mut Position)>::query();
            for (_, position) in query.iter_mut(&mut self.0) {
                match position.x {
                    x if x == x1 => position.x = x2,
                    x if x == x2 => position.x = x1,
                    _ => (),
                }
            }
        }
    }

    fn shuffle_x_group(&mut self) {
        for _ in 0..10 {
            let g1 = get_random_value::<u8>(0, 2);
            let g2 = (g1 + get_random_value::<u8>(1, 2)) % 3;
            let mut query = <(&Cell, &mut Position)>::query();
            for (_, position) in query.iter_mut(&mut self.0) {
                match position.x {
                    x if x / 3 == g1 => position.x += g2 * 3 - g1 * 3,
                    x if x / 3 == g2 => position.x += g1 * 3 - g2 * 3,
                    _ => (),
                }
            }
        }
    }

    fn shuffle_y(&mut self) {
        for _ in 0..10 {
            let g = get_random_value::<u8>(0, 2);
            let y1 = get_random_value::<u8>(0, 2);
            let y2 = (y1 + get_random_value::<u8>(1, 2)) % 3;
            let y1 = g * 3 + y1;
            let y2 = g * 3 + y2;
            let mut query = <(&Cell, &mut Position)>::query();
            for (_, position) in query.iter_mut(&mut self.0) {
                match position.y {
                    y if y == y1 => position.y = y2,
                    y if y == y2 => position.y = y1,
                    _ => (),
                }
            }
        }
    }

    fn shuffle_y_group(&mut self) {
        for _ in 0..10 {
            let g1 = get_random_value::<u8>(0, 2);
            let g2 = (g1 + get_random_value::<u8>(1, 2)) % 3;
            let mut query = <(&Cell, &mut Position)>::query();
            for (_, position) in query.iter_mut(&mut self.0) {
                match position.y {
                    y if y / 3 == g1 => position.y += g2 * 3 - g1 * 3,
                    y if y / 3 == g2 => position.y += g1 * 3 - g2 * 3,
                    _ => (),
                }
            }
        }
    }

    fn hide_cells(&mut self, count: usize) {
        let mut query = <(&Cell, &Position, &mut Value)>::query();
        for _ in 0..count {
            let x = get_random_value::<u8>(0, 8);
            let y = get_random_value::<u8>(0, 8);
            for (_, position, value) in query.iter_mut(&mut self.0) {
                if position.x == x && position.y == y {
                    value.0 = None;
                }
            }
        }
    }
}

impl Default for Game {
    fn default() -> Self {
        let mut world = World::default();
        Schedule::builder()
            .add_system(create_cells_system())
            .build()
            .execute(&mut world, &mut Resources::default());
        Self(world)
    }
}

#[derive(Debug)]
struct Cell;

#[derive(Debug)]
struct Position {
    x: u8,
    y: u8,
}

#[derive(Debug)]
struct Candidates(u16);

impl Candidates {
    fn is_set(&self, value: u8) -> bool {
        self.0 & (1 << value) != 0
    }

    fn set(&mut self, value: u8) {
        self.0 |= 1 << value;
    }

    fn clear(&mut self, value: u8) {
        self.0 &= 0b1111111110 ^ (1 << value);
    }
}

#[derive(Debug, Default)]
struct Value(Option<u8>);

impl Value {
    fn new(value: u8) -> Self {
        Self(Some(value))
    }

    fn is_none(&self) -> bool {
        self.0.is_none()
    }

    fn is_some(&self) -> bool {
        self.0.is_some()
    }
}

impl From<&Value> for Option<u8> {
    fn from(value: &Value) -> Self {
        value.0
    }
}

impl From<&Value> for u8 {
    fn from(value: &Value) -> Self {
        value.0.unwrap()
    }
}

#[derive(Debug)]
struct IsGameOver(pub bool);

#[derive(Debug)]
struct SetCell {
    x: u8,
    y: u8,
    value: Option<u8>,
}

#[derive(Debug)]
struct ToggleCandidate {
    x: u8,
    y: u8,
    value: u8,
}

#[system(for_each)]
fn game_over(_: &Cell, value: &Value, #[resource] res: &mut IsGameOver) {
    if value.is_none() {
        res.0 = false;
    }
}

#[system]
fn create_cells(cmd: &mut CommandBuffer) {
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
            cmd.push((Cell, Position { x, y }, Candidates(0), Value::new(value)));
        }
    }
}

#[system(for_each)]
fn set_cell(
    _: &Cell,
    position: &Position,
    candidates: &mut Candidates,
    value: &mut Value,
    #[resource] res: &SetCell,
) {
    if position.x == res.x && position.y == res.y {
        value.0 = res.value;
    } else if position.x == res.x
        || position.y == res.y
        || position.x % 3 == res.x % 3
        || position.y % 3 == res.y % 3
    {
        // TODO: deal with None
        if let Some(v) = res.value {
            candidates.clear(v);
        }
        if value.0 == res.value {
            value.0 = None;
        }
    }
}

#[system(for_each)]
fn toggle_candidate(
    _: &Cell,
    position: &Position,
    candidates: &mut Candidates,
    #[resource] res: &ToggleCandidate,
) {
    if position.x == res.x && position.y == res.y {
        if candidates.is_set(res.value) {
            candidates.clear(res.value);
        } else {
            candidates.set(res.value);
        }
    }
}

fn get_random_value<T>(min: i32, max: i32) -> T
where
    T: TryFrom<i32>,
{
    let value = raylib::misc::get_random_value::<i32>(min, max);
    T::try_from(value).unwrap_or_else(|_| panic!("conversion failure"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_create_new_game() {
        let mut game = Game::default();
        assert!(game.is_game_over());
    }
}
