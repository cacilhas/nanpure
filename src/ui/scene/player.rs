use rscenes::prelude::*;

pub enum Move {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug)]
pub struct Player {
    pub x: u8,
    pub y: u8,
}

impl Default for Player {
    fn default() -> Self {
        Self { x: 4, y: 4 }
    }
}

impl Player {
    pub fn draw(&self, draw: &mut RaylibMode2D<RaylibDrawHandle>, rect: Rectangle) {
        let width = rect.width / 9.0;
        let height = rect.height / 9.0;
        draw.draw_rectangle(
            (self.x as f32 * width + rect.x) as i32,
            (self.y as f32 * height + rect.y) as i32,
            width as i32,
            height as i32,
            colors::DARKCYAN,
        );
    }

    pub fn move_to(&mut self, direction: &Move) {
        match direction {
            Move::Left => self.x = (self.x + 8) % 9,
            Move::Right => self.x = (self.x + 1) % 9,
            Move::Up => self.y = (self.y + 8) % 9,
            Move::Down => self.y = (self.y + 1) % 9,
        }
    }
}
