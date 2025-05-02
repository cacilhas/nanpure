use bevy::prelude::*;

use crate::{consts::CANDIDATE_SIZE, gameplay::Gameplay};

use super::{board::BoardCell, Colors, Shapes};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cell(u16);

impl Default for Cell {
    fn default() -> Self {
        Cell(0b0001_1111_1111_0000)
    }
}

impl Cell {

    pub fn get_value(&self) -> u8 {
        (self.0 & 0b0000_0000_0000_1111) as u8
    }

    pub fn set_value(&mut self, value: u8) -> bool {
        if value > 9 {
            return false;
        }
        if self.is_set_candidate(value) {
            self.0 |= value as u16;
            true
        } else {
            false
        }
    }

    pub fn toggle_candidate(&mut self, value: u8) -> bool {
        if value > 9 {
            return false;
        }
        if self.is_set_candidate(value) {
            self.clean_candidate(value)
        } else {
            self.set_candidate(value)
        }
    }

    pub fn is_set_candidate(&self, value: u8) -> bool {
        if value > 9 {
            return false;
        }
        self.0 & (1 << (value + 4)) != 0
    }

    pub fn set_candidate(&mut self, value: u8) -> bool {
        if value > 9 {
            return false;
        }
        self.0 |= 1 << (value + 4);
        true
    }

    pub fn clean_candidate(&mut self, value: u8) -> bool {
        if value > 9 {
            return false;
        }
        self.0 &= 0b0001_1111_1111_1111 ^ (1 << (value + 4));
        true
    }

    pub fn render(&self,
        x: f32,
        y: f32,
        commands: &mut Commands,
        shapes: &mut ResMut<Shapes>,
        colors: &mut ResMut<Colors>,
    ) {
        let value = self.get_value();
        if value == 0 {
            self.render_candidates(x, y, commands, shapes, colors);
        } else {
            self.render_value(value, x, y, commands, shapes, colors);
        }
    }

    fn render_value(
        &self,
        value: u8,
        x: f32,
        y: f32,
        commands: &mut Commands,
        shapes: &mut ResMut<Shapes>,
        colors: &mut ResMut<Colors>,
    ) {
        commands.spawn((
            BoardCell,
            Gameplay,
            Mesh2d(shapes.cell.clone()),
            MeshMaterial2d(colors.get(value as usize).clone()),
            Transform::from_xyz(x, y, 1.0),
        ));
    }

    fn render_candidates(
        &self,
        x: f32,
        y: f32,
        commands: &mut Commands,
        shapes: &mut ResMut<Shapes>,
        colors: &mut ResMut<Colors>,
    ) {
        let shape = &shapes.cell_candidate;
        for i in 0..9 {
            let ax = x + ((i % 3) as f32 - 1.5) * CANDIDATE_SIZE;
            let ay = y + ((i / 3) as f32 - 1.5) * CANDIDATE_SIZE;
            let color = colors.get(i);
            commands.spawn((
                BoardCell,
                Gameplay,
                Mesh2d(shape.clone()),
                MeshMaterial2d(color.clone()),
                Transform::from_xyz(ax, ay, 1.0),
            ));
        }
    }
}
