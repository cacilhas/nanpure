use raylib::prelude::*;

use crate::game::{Game, Level, COLORS};

use super::{
    player::{Move, Player},
    Scene, State,
};

#[derive(Debug, Default)]
pub struct GameplayScene {
    game: Game,
    player: Player,
    rect: Rectangle,
}

const SIZE: f32 = 630.0;

impl GameplayScene {
    pub fn new(level: &Level) -> Self {
        let mut gameplay = Self::default();
        gameplay.game.shuffle(level);
        gameplay
    }

    fn update_rect(&mut self, handle: &RaylibHandle) {
        let mut rect = Rectangle {
            width: handle.get_screen_width() as f32,
            height: handle.get_screen_height() as f32,
            ..Default::default()
        };
        if rect.width > SIZE {
            rect.x = (rect.width - SIZE) / 2.0;
            rect.width = SIZE;
        }
        if rect.height > SIZE {
            rect.y = (rect.height - SIZE) / 2.0;
            rect.height = SIZE;
        }
        self.rect = rect;
    }

    fn deal_with_keyboard(&mut self, draw: &mut RaylibMode2D<RaylibDrawHandle>) {
        let width = self.rect.width / 9.0;
        let height = self.rect.height / 9.0;
        for x in 0..=9 {
            draw.draw_line_ex(
                Vector2::new(self.rect.x + x as f32 * width, self.rect.y),
                Vector2::new(
                    self.rect.x + x as f32 * width,
                    self.rect.y + self.rect.height,
                ),
                if x % 3 == 0 { 2.0 } else { 1.0 },
                COLORS[0],
            );
        }
        for y in 0..=9 {
            draw.draw_line_ex(
                Vector2::new(self.rect.x, self.rect.y + y as f32 * height),
                Vector2::new(
                    self.rect.x + self.rect.width,
                    self.rect.y + y as f32 * height,
                ),
                if y % 3 == 0 { 2.0 } else { 1.0 },
                COLORS[0],
            );
        }
    }

    fn detect_keys(&mut self, handle: &mut RaylibDrawHandle) {
        if handle.is_key_pressed(KeyboardKey::KEY_LEFT) || handle.is_key_pressed(KeyboardKey::KEY_A)
        {
            self.player.move_to(&Move::Left);
        }
        if handle.is_key_pressed(KeyboardKey::KEY_RIGHT)
            || handle.is_key_pressed(KeyboardKey::KEY_D)
        {
            self.player.move_to(&Move::Right);
        }
        if handle.is_key_pressed(KeyboardKey::KEY_UP) || handle.is_key_pressed(KeyboardKey::KEY_W) {
            self.player.move_to(&Move::Up);
        }
        if handle.is_key_pressed(KeyboardKey::KEY_DOWN) || handle.is_key_pressed(KeyboardKey::KEY_S)
        {
            self.player.move_to(&Move::Down);
        }
    }
}

impl Scene for GameplayScene {
    fn init(
        &mut self,
        handle: &mut raylib::RaylibHandle,
        _: &raylib::RaylibThread,
        _: std::rc::Rc<raylib::text::Font>,
    ) {
        self.update_rect(handle);
    }

    fn update(&mut self, _: chrono::Duration, handle: &mut RaylibDrawHandle) -> State {
        self.update_rect(handle);
        self.detect_keys(handle);
        let camera = Camera2D {
            zoom: 1.0,
            ..Default::default()
        };
        let mut draw = handle.begin_mode2D(camera);
        let background_color = Color::WHEAT;
        draw.clear_background(background_color);
        self.player.draw(&mut draw, self.rect.to_owned());
        self.game.draw(&mut draw, self.rect.to_owned());
        self.deal_with_keyboard(&mut draw);

        State::Keep
    }
}
