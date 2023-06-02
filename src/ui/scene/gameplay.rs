use std::rc::Rc;

use rscenes::prelude::*;

use crate::game::{Game, Level, COLORS};

use super::player::{Move, Player};

#[derive(Debug, Default)]
pub struct GameplayScene {
    game: Game,
    player: Player,
    rect: Rectangle,
}

const SIZE: f32 = 630.0;

impl GameplayScene {
    pub fn new(level: Level) -> Self {
        let mut gameplay = Self::default();
        gameplay.game.shuffle(level);
        gameplay
    }

    fn update_rect(&mut self, screen: Rectangle) {
        let mut rect = screen.clone();
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

    fn detect_keys(&mut self, handle: &mut RaylibHandle) {
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
        let control = handle.is_key_down(KeyboardKey::KEY_LEFT_CONTROL)
            || handle.is_key_down(KeyboardKey::KEY_RIGHT_CONTROL)
            || handle.is_key_down(KeyboardKey::KEY_LEFT_SHIFT)
            || handle.is_key_down(KeyboardKey::KEY_RIGHT_SHIFT);
        for (i, keys) in CHANGE_KEYS.iter().enumerate() {
            if keys.iter().any(|key| handle.is_key_released(*key)) {
                if control {
                    let value = if i == 0 { None } else { Some(i as u8) };
                    self.game.set_cell(self.player.x, self.player.y, value);
                } else if i > 0 {
                    self.game
                        .toggle_candidate(self.player.x, self.player.y, i as u8);
                }
            }
        }
    }
}

impl Scene for GameplayScene {
    fn update(
        &mut self,
        (handle, _): (&mut RaylibHandle, &RaylibThread),
        _: f32,
        _: Option<Rc<&mut RaylibAudio>>,
    ) -> anyhow::Result<State> {
        if !self.game.is_game_over() {
            self.detect_keys(handle);
        }
        Ok(State::Keep)
    }

    fn draw(
        &mut self,
        handle: &mut RaylibDrawHandle,
        screen: Rectangle,
        _: Option<Rc<Font>>,
        _: Option<Rc<&mut RaylibAudio>>,
    ) -> anyhow::Result<()> {
        self.update_rect(screen);
        let camera = Camera2D {
            zoom: 1.0,
            ..Default::default()
        };
        let mut draw = handle.begin_mode2D(camera);
        let background_color = if self.game.is_game_over() {
            Color::new(0, 139, 139, 255)
        } else {
            Color::new(245, 222, 179, 255)
        };
        draw.clear_background(background_color);
        self.player.draw(&mut draw, self.rect.to_owned());
        self.game.draw(&mut draw, self.rect.to_owned());
        self.deal_with_keyboard(&mut draw);

        Ok(())
    }
}

static CHANGE_KEYS: [[KeyboardKey; 2]; 10] = [
    [KeyboardKey::KEY_ZERO, KeyboardKey::KEY_KP_0],
    [KeyboardKey::KEY_ONE, KeyboardKey::KEY_KP_1],
    [KeyboardKey::KEY_TWO, KeyboardKey::KEY_KP_2],
    [KeyboardKey::KEY_THREE, KeyboardKey::KEY_KP_3],
    [KeyboardKey::KEY_FOUR, KeyboardKey::KEY_KP_4],
    [KeyboardKey::KEY_FIVE, KeyboardKey::KEY_KP_5],
    [KeyboardKey::KEY_SIX, KeyboardKey::KEY_KP_6],
    [KeyboardKey::KEY_SEVEN, KeyboardKey::KEY_KP_7],
    [KeyboardKey::KEY_EIGHT, KeyboardKey::KEY_KP_8],
    [KeyboardKey::KEY_NINE, KeyboardKey::KEY_KP_9],
];
