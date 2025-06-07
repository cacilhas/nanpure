use bevy::prelude::*;

use crate::consts::MAGICAL_AJUSTMENT_NUMBER;
use crate::gameplay::Gameplay;
use crate::{consts::CELL_SIZE, kennett::KennettConnector};

use super::anim::Anim;
use super::board_cell::BoardCell;
use super::cell::Cell;
use super::cursor::Cursor;
use super::Colors;
use super::Level;
use super::Shapes;

#[derive(Debug)]
pub struct InnerBoard([Cell; 81], usize, bool);

impl InnerBoard {

    fn recalculate(mut self) -> Self {
        for idx in 0..81 {
            if self.0[idx].value() == 0 {
                self.2 = false;
                return self;
            }
        }
        self.2 = true;
        self
    }

    pub fn is_done(&self) -> bool {
        self.2
    }

    pub fn update(
        &self,
        query: &mut Query<&mut Transform, With<Cursor>>,
    ) {
        for idx in 0..81 {
            if idx == self.1 {
                if let Ok(mut transform) = query.single_mut() {
                    transform.translation.x = ((idx % 9) as f32 - 4.0) * CELL_SIZE;
                    transform.translation.y = ((idx / 9) as f32 - 4.0) * CELL_SIZE + MAGICAL_AJUSTMENT_NUMBER;
                }
                return;
            }
        }
    }

    pub fn size(&self) -> Vec2 {
        Vec2::new(CELL_SIZE * 9.0, CELL_SIZE * 9.0)
    }

    pub(super) fn toggle_candidate(&self, x: usize, y: usize, value: u8, actions: &mut Vec<Anim>) -> Option<Self> {
        let mut board = self.clone();
        if board.0[x + y * 9].toggle_candidate(value) {
            actions.push(if board.cell(x, y).is_candidate_set(value) {
                Anim::SetCandidate { x, y, value }
            } else {
                Anim::UnsetCandidate { x, y, value }
            });
            Some(board)
        } else {
            None
        }
    }

    pub(super) fn set_value(&self, x: usize, y: usize, value: u8, actions: &mut Vec<Anim>) -> Option<Self> {
        let mut new_board = self.clone();

        if new_board.cell_mut(x, y).set_value(value) {
            new_board.clean_row(x, y, value, actions);
            new_board.clean_column(x, y, value, actions);
            new_board.clean_group(x, y, value, actions);
            Some(new_board.recalculate())

        } else {
            None
        }
    }

    pub fn render(
        &self,
        x: f32,
        y: f32,
        commands: &mut Commands,
        cell_query: &Query<Entity, With<BoardCell>>,
        cursor_query: &Query<Entity, With<Cursor>>,
        shapes: &Res<Shapes>,
        colors: &Res<Colors>,
        anims: &Vec<Anim>,
    ) {
        // Clean up before populate
        for entity in cell_query.iter() {
            commands.entity(entity).despawn();
        }
        for entity in cursor_query.iter() {
            commands.entity(entity).despawn();
        }

        commands.spawn((
            Gameplay,
            Cursor,
            Mesh2d(shapes.rect.clone_weak()),
            MeshMaterial2d(colors.highlight().clone_weak()),
            Transform::from_xyz(x, y, 0.0),
        ));

        for iy in 0..9 {
            for ix in 0..9 {
                let mut anims: Vec<Anim> = anims.iter()
                    .filter_map(|anim|
                        if anim.is_at(ix, iy) {
                            Some(anim.clone())
                        } else {
                            None
                        }
                    )
                    .collect()
                ;
                anims.sort();
                if let Some(anim) = anims.first() {
                    if !anim.is_candidate() {
                        let anim = anim.clone();
                        anims.clear();
                        anims.push(anim);
                    }
                }
                self.cell(ix, iy).render(
                    x + (ix as f32 - 4.0) * CELL_SIZE,
                    y + (iy as f32 - 4.0) * CELL_SIZE,
                    commands,
                    shapes,
                    colors,
                    anims,
                );
            }
        }
    }

    pub fn cell(&self, x: usize, y: usize) -> &Cell {
        &self.0[x + y * 9]
    }

    fn cell_mut(&mut self, x: usize, y: usize) -> &mut Cell {
        &mut self.0[x + y * 9]
    }

    pub fn highlight(&self) -> (usize, usize) {
        (
            self.1 % 9,
            self.1 / 9,
        )
    }

    pub fn set_highlight(&mut self, x: usize, y: usize) {
        self.1 = (x % 9) + (y % 9) * 9;
    }

    fn clean_row(&mut self, x: usize, y: usize, value: u8, actions: &mut Vec<Anim>) {
        for ax in 0..9 {
            if ax != x {
                if self.cell_mut(ax, y).clean_candidate(value) {
                    actions.push(Anim::UnsetCandidate { x: ax, y, value });
                }
            }
        }
    }

    fn clean_column(&mut self, x: usize, y: usize, value: u8, actions: &mut Vec<Anim>) {
        for ay in 0..9 {
            if ay != y {
                if self.cell_mut(x, ay).clean_candidate(value) {
                    actions.push(Anim::UnsetCandidate { x, y: ay, value });
                }
            }
        }
    }

    fn clean_group(&mut self, x: usize, y: usize, value: u8, actions: &mut Vec<Anim>) {
        let gx = (x / 3) * 3;
        let gy = (y / 3) * 3;
        for ax in gx..(gx + 3) {
            for ay in gy..(gy + 3) {
                if ax != x || ay != y {
                    if self.cell_mut(ax, ay).clean_candidate(value) {
                        actions.push(Anim::UnsetCandidate { x: ax, y: ay, value });
                    }
                }
            }
        }
    }
}

impl Default for InnerBoard {
    fn default() -> Self {
        Self(
            std::array::from_fn(|_| Cell::default()),
            40,
            false,
        )
    }
}

impl Clone for InnerBoard {
    fn clone(&self) -> Self {
        Self(
            std::array::from_fn(|idx| self.0[idx].clone()),
            self.1,
            self.2,
        )
    }
}

impl TryFrom<Level> for InnerBoard {
    type Error = std::io::Error;

    fn try_from(level: Level) -> Result<InnerBoard, Self::Error> {
        let cells = KennettConnector::generate(level)?;
        let mut board = InnerBoard::default();
        for (i, cell) in cells.into_iter().enumerate() {
            let x = i % 9;
            let y = i / 9;
            board = match board.set_value(x, y, cell, &mut Vec::new()) {
                Some(new_board) => new_board,
                None => board,
            };
        }
        Ok(board)
    }
}
