use bevy::prelude::*;

use crate::{consts::CELL_SIZE, kennett::KennettConnector};

use super::cell::Cell;
use super::Colors;
use super::Level;
use super::Shapes;

#[derive(Debug, Clone)]
pub struct Board([Cell; 81], usize);

#[derive(Debug, Clone, Default, Component)]
pub struct BoardWrapper(Vec<Board>);

#[derive(Debug, Clone, Copy, Component)]
pub struct BoardCell;

impl Board {

    pub fn size(&self) -> Vec2 {
        Vec2::new(CELL_SIZE * 9.0, CELL_SIZE * 9.0)
    }

    pub fn toggle_candidate(&self, x: usize, y: usize, value: u8) -> Option<Self> {
        let mut new_board = self.clone();
        if new_board.0[x + y * 9].toggle_candidate(value) {
            Some(new_board)
        } else {
            None
        }
    }

    pub fn set_value(&self, x: usize, y: usize, value: u8) -> Option<Self> {
        let mut new_board = self.clone();

        if new_board.0[x + y * 9].set_value(value) {
            new_board.clean_row(x, y, value);
            new_board.clean_column(x, y, value);
            new_board.clean_group(x, y, value);
            Some(new_board)

        } else {
            None
        }
    }

    pub fn render(
        &self,
        x: f32,
        y: f32,
        commands: &mut Commands,
        query: &Query<Entity, With<BoardCell>>,
        shapes: &mut ResMut<Shapes>,
        colors: &mut ResMut<Colors>,
    ) {
        // Clean up before populate
        for entity in query.iter() {
            commands.entity(entity).despawn();
        }

        for i in 0..81 {
            if self.1 == i {
                commands.spawn((
                    BoardCell,
                    Mesh2d(shapes.rect.clone()),
                    MeshMaterial2d(colors.get(10).clone())
                ));
            }
            self.0[i].render(
                x + ((i % 9) as f32 - 4.0) * CELL_SIZE,
                y + ((i / 9) as f32 - 4.0) * CELL_SIZE,
                commands,
                shapes,
                colors,
            );
        }
    }

    fn clean_row(&mut self, x: usize, y: usize, value: u8) {
        for ax in 0..9 {
            if ax != x {
                self.0[ax + y * 9].clean_candidate(value);
            }
        }
    }

    fn clean_column(&mut self, x: usize, y: usize, value: u8) {
        for ay in 0..9 {
            if ay != y {
                self.0[x + ay * 9].clean_candidate(value);
            }
        }
    }

    fn clean_group(&mut self, x: usize, y: usize, value: u8) {
        let gx = (x / 3) * 3;
        let gy = (y / 3) * 3;
        for ax in gx..(gx + 2) {
            for ay in gy..(gy + 2) {
                if ax != x || ay != y {
                    self.0[ax + ay * 9].clean_candidate(value);
                }
            }
        }
    }
}

impl Default for Board {
    fn default() -> Self {
        Self(
            std::array::from_fn(|_| Cell::default()),
            90,
        )
    }
}

impl TryFrom<Level> for Board {
    type Error = std::io::Error;

    fn try_from(level: Level) -> Result<Board, Self::Error> {
        let cells = KennettConnector::generate(level)?;
        let mut board = Board::default();
        for (i, cell) in cells.into_iter().enumerate() {
            let x = i % 9;
            let y = i / 9;
            board = match board.set_value(x, y, cell) {
                Some(new_board) => new_board,
                None => board,
            };
        }
        Ok(board)
    }
}

impl BoardWrapper {

    pub fn size(&self) -> Result<Vec2, std::io::Error> {
        Ok(self.current()?.size())
    }

    pub fn add(&mut self, board: Board) {
        self.0.push(board);
    }

    pub fn render(
        &self,
        x: f32,
        y: f32,
        commands: &mut Commands,
        query: &Query<Entity, With<BoardCell>>,
        shapes: &mut ResMut<Shapes>,
        colors: &mut ResMut<Colors>,
    ) -> Result<(), std::io::Error> {
        self.current()?.render(x, y, commands, query, shapes, colors);
        Ok(())
    }

    pub fn current(&self) -> Result<&Board, std::io::Error> {
        if self.0.is_empty() {
            Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "No board available",
            ))
        } else {
            Ok(&self.0[self.0.len() - 1])
        }
    }

    pub fn undo(&mut self) -> bool {
        if self.0.is_empty() {
            false
        } else {
            self.0.pop();
            true
        }
    }
}
