mod candidates;
mod cell;
mod level;
mod position;
mod systems;
mod value;

use self::{candidates::Candidates, cell::Cell, position::Position, systems::*, value::Value};

pub use self::level::Level;
use legion::*;

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

    pub fn stringify(&mut self) -> Result<String, String> {
        let mut resources = Resources::default();
        resources.insert([0_u8; 81]);
        Schedule::builder()
            .add_system(display_system())
            .build()
            .execute(&mut self.0, &mut resources);
        let mut res = "".to_owned();
        let arr = resources.get::<[u8; 81]>().ok_or("resources not found")?;
        for i in 0..81 {
            let value = if arr[i] == 0 {
                " ".to_owned()
            } else {
                format!("{}", arr[i])
            };
            if i % 9 == 0 {
                res = format!("{}\n{}", res, value);
            } else {
                res = format!("{}{}", res, value);
            }
        }
        Ok(res[1..].to_owned())
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
                    value.clean();
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_create_new_game() {
        let mut game = Game::default();
        assert!(game.is_game_over());
    }

    #[test]
    fn it_should_stringify() {
        let mut game = Game::default();
        let expected = "123456789\n\
            456789123\n\
            789123456\n\
            234567891\n\
            567891234\n\
            891234567\n\
            345678912\n\
            678912345\n\
            912345678";
        assert_eq!(game.stringify().unwrap(), expected);
    }

    #[test]
    fn it_should_set_cell() {
        let mut game = Game::default();
        for x in 0..9 {
            game.set_cell(x, 0, None);
        }
        game.set_cell(3, 0, Some(4));
        let expected = "   4     \n\
            456789123\n\
            789123456\n\
            234567891\n\
            567891234\n\
            891234567\n\
            345678912\n\
            678912345\n\
            912345678";
        assert_eq!(game.stringify().unwrap(), expected);
    }
}

pub fn get_random_value<T>(min: i32, max: i32) -> T
where
    T: TryFrom<i32>,
{
    let value = raylib::misc::get_random_value::<i32>(min, max);
    T::try_from(value).unwrap_or_else(|_| panic!("conversion failure"))
}
