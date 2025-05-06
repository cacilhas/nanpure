use bevy::prelude::*;

use crate::{consts::CANDIDATE_SIZE, gameplay::Gameplay};

use super::board_cell::BoardCell;
use super::colors::Colors;
use super::shapes::Shapes;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cell(u16);

impl Default for Cell {
    fn default() -> Self {
        Cell(0b0001_1111_1111_0000)
    }
}

impl Cell {

    pub fn value(&self) -> u8 {
        (self.0 & 0b0000_0000_0000_1111) as u8
    }

    pub fn set_value(&mut self, value: u8) -> bool {
        if value > 9 || value == self.value() {
            false

        } else if value == 0 {
            self.0 &= 0b0001_1111_1111_0000;
            true

        } else if self.is_candidate_set(value) {
            self.0 |= value as u16;
            true

        } else {
            false
        }
    }

    pub fn toggle_candidate(&mut self, value: u8) -> bool {
        if value == 0 || value > 9 {
            false

        } else if self.is_candidate_set(value) {
            self.do_clean_candidate(value)

        } else {
            self.do_set_candidate(value)
        }
    }

    pub fn is_candidate_set(&self, value: u8) -> bool {
        if value == 0 || value > 9 {
            false
        } else {
            self.0 & (1 << (value + 3)) != 0
        }
    }

    pub fn set_candidate(&mut self, value: u8) -> bool {
        if self.is_candidate_set(value) {
            false
        } else {
            self.do_set_candidate(value)
        }
    }

    pub fn clean_candidate(&mut self, value: u8) -> bool {
        if self.is_candidate_set(value) {
            self.do_clean_candidate(value)
        } else {
            false
        }
    }

    fn do_set_candidate(&mut self, value: u8) -> bool {
        if value == 0 || value > 9 {
            false
        } else {
            self.0 |= 1 << (value + 3);
            true
        }
    }

    fn do_clean_candidate(&mut self, value: u8) -> bool {
        if value == 0 || value > 9 {
            false
        } else {
            self.0 &= 0b0001_1111_1111_1111 ^ (1 << (value + 3));
            true
        }
    }

    pub fn render(&self,
        x: f32,
        y: f32,
        commands: &mut Commands,
        shapes: &Res<Shapes>,
        colors: &Res<Colors>,
    ) {
        let value = self.value();
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
        shapes: &Res<Shapes>,
        colors: &Res<Colors>,
    ) {
        commands.spawn((
            BoardCell,
            Gameplay,
            Mesh2d(shapes.cell.clone_weak()),
            MeshMaterial2d(colors.get(value as usize).clone_weak()),
            Transform::from_xyz(x, y, 1.0),
        ));
    }

    fn render_candidates(
        &self,
        x: f32,
        y: f32,
        commands: &mut Commands,
        shapes: &Res<Shapes>,
        colors: &Res<Colors>,
    ) {
        let shape = &shapes.cell_candidate;
        for i in 0..9 {
            if self.is_candidate_set(i as u8 + 1) {
                let ax = x + ((i % 3) as f32 - 1.0) * CANDIDATE_SIZE;
                let ay = y + ((i / 3) as f32 - 1.0) * CANDIDATE_SIZE;
                let color = colors.get(i + 1);
                commands.spawn((
                    BoardCell,
                    Gameplay,
                    Mesh2d(shape.clone_weak()),
                    MeshMaterial2d(color.clone_weak()),
                    Transform::from_xyz(ax, ay, 1.0),
                ));
            }
        }
    }
}
