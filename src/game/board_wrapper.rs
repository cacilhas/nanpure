use bevy::prelude::*;

use super::board_cell::BoardCell;
use super::colors::Colors;
use super::cursor::Cursor;
use super::inner_board::InnerBoard;
use super::shapes::Shapes;
use super::Level;

/**
 * Wrapper around the InnerBoard struct in order to supply undo functionality.
 */
#[derive(Debug, Clone, Default, Component)]
pub struct Board(Vec<InnerBoard>);

impl Board {

    pub fn update(
        &self,
        query: &mut Query<&mut Transform, With<Cursor>>,
    ) -> bevy::ecs::error::Result<()> {
        self.current()?.update(query);
        Ok(())
    }

    pub fn highlight(&self) -> Result<(i32, i32), std::io::Error> {
        let (ix, iy) = self.current()?.highlight();
        Ok((ix as i32, iy as i32))
    }

    pub fn set_highlight(&mut self, xi: i32, yi: i32) {
        let xi = ((9 + xi) % 9) as usize;
        let yi = ((9 + yi) % 9) as usize;
        for board in &mut self.0 {
            board.set_highlight(xi, yi);
        }
    }

    pub fn size(&self) -> Result<Vec2, std::io::Error> {
        Ok(self.current()?.size())
    }

    pub fn set_value(&mut self, value: u8) -> Result<bool, std::io::Error> {
        let (x, y) = self.highlight()?;
        if let Some(board) = self.current()?.set_value(x as usize, y as usize, value) {
            self.0.push(board);
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn toggle_candidate(&mut self, candidate: u8) -> Result<bool, std::io::Error> {
        let (x, y) = self.highlight()?;
        if let Some(board) = self.current()?.toggle_candidate(x as usize, y as usize, candidate) {
            self.0.push(board);
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn undo(&mut self) -> bool {
        if self.0.len() < 2 {
            false
        } else {
            self.0.pop();
            true
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
    ) -> Result<(), std::io::Error> {
        self.current()?.render(x, y, commands, cell_query, cursor_query, shapes, colors);
        Ok(())
    }

    fn current(&self) -> Result<&InnerBoard, std::io::Error> {
        if self.0.is_empty() {
            Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "No board available",
            ))
        } else {
            Ok(&self.0[self.0.len() - 1])
        }
    }

    fn add(&mut self, board: InnerBoard) {
        self.0.push(board);
    }
}

impl TryFrom<Level> for Board {
    type Error = std::io::Error;

    fn try_from(level: Level) -> Result<Self, Self::Error> {
        let mut board = Board::default();
        board.add(InnerBoard::try_from(level)?);
        Ok(board)
    }
}
