use std::{cell::RefCell, rc::Rc};

use crate::game::Level;

use super::{gameplay::GameplayScene, Scene, State};
use raylib::prelude::*;

#[derive(Debug)]
pub struct MainMenuScene {
    rect: Rectangle,
    font: Rc<Font>,
}

impl Default for MainMenuScene {
    fn default() -> Self {
        let font = unsafe { ffi::GetFontDefault() };
        let font = unsafe { Font::from_raw(font) };
        Self {
            rect: Rectangle::default(),
            font: font.into(),
        }
    }
}

impl Scene for MainMenuScene {
    fn init(&mut self, handle: &mut RaylibHandle, _: &RaylibThread, font: Rc<Font>) {
        self.update_rect(handle);
        self.font = font;
    }

    fn update(&mut self, _: chrono::Duration, handle: &mut RaylibDrawHandle) -> State {
        self.update_rect(handle);
        let clicked = handle.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT);
        let mouse = Vector2::new(handle.get_mouse_x() as f32, handle.get_mouse_y() as f32);

        let camera = Camera2D {
            zoom: 1.0,
            ..Default::default()
        };
        let mut draw = handle.begin_mode2D(camera);

        let background_color = Color::WHEAT;
        draw.clear_background(background_color);

        let size = measure_text_ex(self.font.as_ref(), "Nanpure", 84.0, 2.0);
        let position = Vector2::new((self.rect.width - size.x) / 2.0, 0.0);
        let bottom = size.y + 16.0;
        draw.draw_text_ex(
            self.font.as_ref(),
            "Nanpure",
            position,
            84.0,
            2.0,
            Color::DARKCYAN,
        );

        let size = measure_text_ex(self.font.as_ref(), "(Sudoku)", 32.0, 1.0);
        let position = Vector2::new((self.rect.width - size.x) / 2.0, bottom);
        let bottom = bottom + size.y + 64.0;
        draw.draw_text_ex(
            self.font.as_ref(),
            "(Sudoku)",
            position,
            32.0,
            2.0,
            Color::DARKCYAN,
        );

        let size = measure_text_ex(self.font.as_ref(), "Extremely Easy", 64.0, 1.0);
        let position = Vector2::new((self.rect.width - size.x) / 2.0, bottom);
        let bottom = bottom + 12.0 + size.y;
        let extremely_easy_bt = Rectangle {
            x: position.x,
            y: position.y,
            width: size.x,
            height: size.y,
        };
        let tint = if extremely_easy_bt.check_collision_point_rec(mouse) {
            Color::BLACK
        } else {
            Color::DARKGRAY
        };
        draw.draw_text_ex(
            self.font.as_ref(),
            "Extremely Easy",
            position,
            64.0,
            1.0,
            tint,
        );

        let size = measure_text_ex(self.font.as_ref(), "Easy", 64.0, 1.0);
        let position = Vector2::new((self.rect.width - size.x) / 2.0, bottom);
        let bottom = bottom + 12.0 + size.y;
        let easy_bt = Rectangle {
            x: position.x,
            y: position.y,
            width: size.x,
            height: size.y,
        };
        let tint = if easy_bt.check_collision_point_rec(mouse) {
            Color::BLACK
        } else {
            Color::DARKGRAY
        };
        draw.draw_text_ex(self.font.as_ref(), "Easy", position, 64.0, 1.0, tint);

        let size = measure_text_ex(self.font.as_ref(), "Medium", 64.0, 1.0);
        let position = Vector2::new((self.rect.width - size.x) / 2.0, bottom);
        let bottom = bottom + 12.0 + size.y;
        let medium_bt = Rectangle {
            x: position.x,
            y: position.y,
            width: size.x,
            height: size.y,
        };
        let tint = if medium_bt.check_collision_point_rec(mouse) {
            Color::BLACK
        } else {
            Color::DARKGRAY
        };
        draw.draw_text_ex(self.font.as_ref(), "Medium", position, 64.0, 1.0, tint);

        let size = measure_text_ex(self.font.as_ref(), "Hard", 64.0, 1.0);
        let position = Vector2::new((self.rect.width - size.x) / 2.0, bottom);
        let bottom = bottom + 12.0 + size.y;
        let hard_bt = Rectangle {
            x: position.x,
            y: position.y,
            width: size.x,
            height: size.y,
        };
        let tint = if hard_bt.check_collision_point_rec(mouse) {
            Color::BLACK
        } else {
            Color::DARKGRAY
        };
        draw.draw_text_ex(self.font.as_ref(), "Hard", position, 64.0, 1.0, tint);

        let size = measure_text_ex(self.font.as_ref(), "Fiendish", 64.0, 1.0);
        let position = Vector2::new((self.rect.width - size.x) / 2.0, bottom);
        let fiendish_bt = Rectangle {
            x: position.x,
            y: position.y,
            width: size.x,
            height: size.y,
        };
        let tint = if fiendish_bt.check_collision_point_rec(mouse) {
            Color::BLACK
        } else {
            Color::DARKGRAY
        };
        draw.draw_text_ex(self.font.as_ref(), "Fiendish", position, 64.0, 1.0, tint);

        let url = "https://github.com/cacilhas/nanpure";
        let size = measure_text_ex(self.font.as_ref(), url, 12.0, 1.0);
        let position = Vector2::new(
            self.rect.width - size.x - 12.0,
            self.rect.height - size.y - 12.0,
        );
        let doc_bt = Rectangle {
            x: position.x,
            y: position.y,
            width: size.x,
            height: size.y,
        };
        let tint = if doc_bt.check_collision_point_rec(mouse) {
            Color::BLACK
        } else {
            Color::DARKGRAY
        };
        draw.draw_text_ex(self.font.as_ref(), url, position, 12.0, 1.0, tint);

        if clicked {
            if extremely_easy_bt.check_collision_point_rec(mouse) {
                let gameplay = GameplayScene::new(&Level::ExtremelyEasy);
                return State::New(Rc::new(RefCell::new(gameplay)));
            }
            if easy_bt.check_collision_point_rec(mouse) {
                let gameplay = GameplayScene::new(&Level::Easy);
                return State::New(Rc::new(RefCell::new(gameplay)));
            }
            if medium_bt.check_collision_point_rec(mouse) {
                let gameplay = GameplayScene::new(&Level::Medium);
                return State::New(Rc::new(RefCell::new(gameplay)));
            }
            if hard_bt.check_collision_point_rec(mouse) {
                let gameplay = GameplayScene::new(&Level::Hard);
                return State::New(Rc::new(RefCell::new(gameplay)));
            }
            if fiendish_bt.check_collision_point_rec(mouse) {
                let gameplay = GameplayScene::new(&Level::Fiendish);
                return State::New(Rc::new(RefCell::new(gameplay)));
            }
            if doc_bt.check_collision_point_rec(mouse) {
                raylib::open_url(url);
            }
        } else {
            if draw.is_key_released(KeyboardKey::KEY_ONE) {
                let gameplay = GameplayScene::new(&Level::ExtremelyEasy);
                return State::New(Rc::new(RefCell::new(gameplay)));
            }
            if draw.is_key_released(KeyboardKey::KEY_TWO) {
                let gameplay = GameplayScene::new(&Level::Easy);
                return State::New(Rc::new(RefCell::new(gameplay)));
            }
            if draw.is_key_released(KeyboardKey::KEY_THREE) {
                let gameplay = GameplayScene::new(&Level::Medium);
                return State::New(Rc::new(RefCell::new(gameplay)));
            }
            if draw.is_key_released(KeyboardKey::KEY_FOUR) {
                let gameplay = GameplayScene::new(&Level::Hard);
                return State::New(Rc::new(RefCell::new(gameplay)));
            }
            if draw.is_key_released(KeyboardKey::KEY_FIVE) {
                let gameplay = GameplayScene::new(&Level::Fiendish);
                return State::New(Rc::new(RefCell::new(gameplay)));
            }
        }

        State::Keep
    }
}

impl MainMenuScene {
    fn update_rect(&mut self, handle: &RaylibHandle) {
        self.rect = Rectangle {
            width: handle.get_screen_width() as f32,
            height: handle.get_screen_height() as f32,
            ..Default::default()
        };
    }
}
