mod candidates;
mod cell;
pub mod level;
mod position;
mod systems;
mod value;

use std::collections::HashMap;
use std::fmt;
use std::os::raw::c_int;

use self::{candidates::Candidates, cell::Cell, systems::*, value::Value};
pub use self::{cell::COLORS, level::Level, position::Position};
use crate::{colors, kennett::KennettConnector};
use hecs::*;
use raylib::Rectangle;

pub struct Game(World);

impl fmt::Debug for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "a game")
    }
}

impl Default for Game {
    fn default() -> Self {
        let mut world = World::default();
        create_cells(&mut world);
        Self(world)
    }
}

impl Game {
    pub unsafe fn draw(&mut self, rect: Rectangle) {
        let width = rect.width / 9.0;
        let height = rect.height / 9.0;
        let sw = width / 3.0;
        let sh = height / 3.0;
        let radius = *[width - 4.0, height - 4.0]
            .iter()
            .reduce(|a, b| if a < b { a } else { b })
            .unwrap()
            / 2.0;
        let sr = *[sw - 2.0, sh - 2.0]
            .iter()
            .reduce(|a, b| if a < b { a } else { b })
            .unwrap()
            / 2.0;
        for (_, (&position, &candidates, &value)) in self
            .0
            .query_mut::<With<(&Position, &Candidates, &Value), &Cell>>()
        {
            let value: Option<u8> = value.into();
            let rect = Rectangle {
                x: rect.x + width * position.x as f32,
                y: rect.y + height * position.y as f32,
                width,
                height,
            };
            match value {
                Some(value) => raylib::DrawCircle(
                    (rect.x + width / 2.0) as i32,
                    (rect.y + height / 2.0) as i32,
                    radius,
                    COLORS[value as usize],
                ),
                None => {
                    let mut set = false;
                    for i in 1_u8..=9 {
                        if candidates.is_set(i) {
                            set = true;
                            let rect = Rectangle {
                                x: rect.x + sw * ((i - 1) % 3) as f32,
                                y: rect.y + sh * (2 - (i - 1) / 3) as f32,
                                width: sw,
                                height: sh,
                            };
                            raylib::DrawCircle(
                                (rect.x + sw / 2.0) as i32,
                                (rect.y + sh / 2.0) as i32,
                                sr,
                                COLORS[i as usize],
                            );
                        }
                    }

                    if !set {
                        raylib::DrawRectangleRec(
                            rect,
                            if (raylib::GetTime() * 10.0) as i32 % 2 == 0 {
                                colors::RED
                            } else {
                                colors::YELLOW
                            },
                        )
                    }
                }
            }
        }
    }

    pub fn populate(&mut self, content: Vec<u8>) {
        create_empty_system(&mut self.0);
        for y in 0..9 {
            for x in 0..9 {
                let index = (x + y * 9) as usize;
                match content[index] {
                    0 => self.set_cell(x, y, None),
                    value => self.set_cell(x, y, Some(value)),
                };
            }
        }
    }

    pub unsafe fn shuffle(&mut self, level: Level) {
        match KennettConnector::generate(level) {
            Some(result) => self.populate(result),

            None => {
                self.shuffle_x();
                self.shuffle_y();
                self.shuffle_x_group();
                self.shuffle_y_group();
                self.hide_cells(level.count())
            }
        }
    }

    pub fn is_game_over(&mut self) -> bool {
        for (_, value) in self.0.query_mut::<With<&Value, &Cell>>() {
            if value.is_none() {
                return false;
            }
        }
        true
    }

    pub fn set_cell(&mut self, x: u8, y: u8, value: Option<u8>) {
        for (_, (&position, candidates, current)) in self
            .0
            .query_mut::<With<(&Position, &mut Candidates, &Value), &Cell>>()
        {
            if position.x == x && position.y == y {
                if let Some(value) = value {
                    if current.is_some() || !candidates.is_set(value) {
                        return;
                    }
                }
                break;
            }
        }

        let mut res = SetCell { x, y, value };
        for (_, (&position, candidates, value)) in self
            .0
            .query_mut::<With<(&Position, &mut Candidates, &mut Value), &Cell>>()
        {
            set_cell_system(position, candidates, value, &mut res);
        }
    }

    pub fn set_all(&mut self) {
        for (_, (candidates, value)) in self.0.query_mut::<With<(&Candidates, &mut Value), &Cell>>()
        {
            if let Some(candidate) = candidates.get_single_value() {
                value.insert(candidate);
            }
        }
    }

    pub fn toggle_candidate(&mut self, x: u8, y: u8, value: u8) {
        for (_, (&position, current)) in self.0.query_mut::<With<(&Position, &Value), &Cell>>() {
            if position.x == x && position.y == y {
                if current.is_some() {
                    return;
                }
                break;
            }
        }

        let res = ToggleCandidate { x, y, value };

        for (_, (&position, candidates)) in self
            .0
            .query_mut::<With<(&Position, &mut Candidates), &Cell>>()
        {
            toggle_candidate_system(&position, candidates, &res);
        }
    }

    unsafe fn shuffle_x(&mut self) {
        for _ in 0..10 {
            let g = get_random_value::<u8>(0, 2);
            let x1 = get_random_value::<u8>(0, 2);
            let x2 = (x1 + get_random_value::<u8>(1, 2)) % 3;
            let x1 = g * 3 + x1;
            let x2 = g * 3 + x2;
            for (_, position) in self.0.query_mut::<With<&mut Position, &Cell>>() {
                match position.x {
                    x if x == x1 => position.x = x2,
                    x if x == x2 => position.x = x1,
                    _ => (),
                }
            }
        }
    }

    unsafe fn shuffle_x_group(&mut self) {
        for _ in 0..10 {
            let g1 = get_random_value::<u8>(0, 2);
            let g2 = (g1 + get_random_value::<u8>(1, 2)) % 3;
            for (_, position) in self.0.query_mut::<With<&mut Position, &Cell>>() {
                match position.x {
                    x if x / 3 == g1 => position.x = (position.x + g2 * 3 - g1 * 3 + 9) % 9,
                    x if x / 3 == g2 => position.x = (position.x + g1 * 3 - g2 * 3 + 9) % 9,
                    _ => (),
                }
            }
        }
    }

    unsafe fn shuffle_y(&mut self) {
        for _ in 0..10 {
            let g = get_random_value::<u8>(0, 2);
            let y1 = get_random_value::<u8>(0, 2);
            let y2 = (y1 + get_random_value::<u8>(1, 2)) % 3;
            let y1 = g * 3 + y1;
            let y2 = g * 3 + y2;
            for (_, position) in self.0.query_mut::<With<&mut Position, &Cell>>() {
                match position.y {
                    y if y == y1 => position.y = y2,
                    y if y == y2 => position.y = y1,
                    _ => (),
                }
            }
        }
    }

    unsafe fn shuffle_y_group(&mut self) {
        for _ in 0..10 {
            let g1 = get_random_value::<u8>(0, 2);
            let g2 = (g1 + get_random_value::<u8>(1, 2)) % 3;
            for (_, position) in self.0.query_mut::<With<&mut Position, &Cell>>() {
                match position.y {
                    y if y / 3 == g1 => position.y = (position.y + g2 * 3 - g1 * 3 + 9) % 9,
                    y if y / 3 == g2 => position.y = (position.y + g1 * 3 - g2 * 3 + 9) % 9,
                    _ => (),
                }
            }
        }
    }

    unsafe fn hide_cells(&mut self, count: usize) {
        let mut values: HashMap<u8, &mut Value> = HashMap::default();
        for (_, (&position, value)) in self.0.query_mut::<With<(&Position, &mut Value), &Cell>>() {
            let i = position.x + position.y * 9;
            values.insert(i, value);
        }
        let mut indexes = (0_u8..81).collect::<Vec<_>>();
        for _ in 0..count {
            let i = get_random_value::<u8>(0, indexes.len() as i32 - 1);
            let i = indexes.remove(i.into());
            values.get_mut(&i).and_then(|value| {
                value.clean();
                None::<Option<()>>
            });
        }

        // Update candidates
        let mut set_cells: Vec<SetCell> = vec![];
        for (_, (&position, &value)) in self.0.query_mut::<With<(&Position, &Value), &Cell>>() {
            let value: Option<u8> = value.into();
            if value.is_some() {
                set_cells.push(SetCell {
                    x: position.x,
                    y: position.y,
                    value,
                });
            }
        }
        while let Some(res) = set_cells.pop() {
            for (_, (&position, candidates, value)) in self
                .0
                .query_mut::<With<(&Position, &mut Candidates, &mut Value), &Cell>>()
            {
                set_cell_system(position, candidates, value, &res);
            }
        }
    }

    #[cfg(test)]
    pub fn stringify(&mut self) -> String {
        let mut arr = [0_u8; 81];
        for (_, (&position, &value)) in self.0.query_mut::<With<(&Position, &Value), &Cell>>() {
            display_system(position, value, &mut arr);
        }
        let mut res = "".to_owned();
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
        res[1..].to_owned()
    }
}

pub unsafe fn get_random_value<T>(min: i32, max: i32) -> T
where
    T: TryFrom<i32>,
{
    let value = raylib::GetRandomValue(min as c_int, max as c_int) as i32;
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
