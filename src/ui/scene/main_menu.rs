use std::borrow::Borrow;

use crate::game::Level;

use super::super::resources::Resources;
use super::gameplay::GameplayScene;
use super::help::HelpScene;
use rscenes::prelude::*;

#[derive(Debug, Default)]
pub struct MainMenuScene {
    extremely_easy: bool,
    easy: bool,
    medium: bool,
    hard: bool,
    fiendish: bool,
    help: bool,
    url: Option<String>,
}

impl Scene<Resources> for MainMenuScene {
    fn init(&mut self, _: &mut RaylibHandle, _: &RaylibThread) -> anyhow::Result<()> {
        self.extremely_easy = false;
        self.easy = false;
        self.medium = false;
        self.hard = false;
        self.fiendish = false;
        self.help = false;
        self.url = None;
        Ok(())
    }

    fn update(
        &mut self,
        (handle, _): (&mut RaylibHandle, &RaylibThread),
        _: f32,
        _: &mut Resources,
    ) -> anyhow::Result<State<Resources>> {
        if handle.is_key_released(KeyboardKey::KEY_ONE) {
            self.extremely_easy = true;
        }
        if handle.is_key_released(KeyboardKey::KEY_TWO) {
            self.easy = true;
        }
        if handle.is_key_released(KeyboardKey::KEY_THREE) {
            self.medium = true;
        }
        if handle.is_key_released(KeyboardKey::KEY_FOUR) {
            self.hard = true;
        }
        if handle.is_key_released(KeyboardKey::KEY_FIVE) {
            self.fiendish = true;
        }
        if handle.is_key_released(KeyboardKey::KEY_F1) {
            self.help = true;
        }

        if let Some(url) = &self.url {
            raylib::open_url(&url);
        }
        self.url = None;
        if self.extremely_easy {
            let gameplay = GameplayScene::new(Level::ExtremelyEasy);
            return Ok(State::New(Box::new(gameplay)));
        }
        if self.easy {
            let gameplay = GameplayScene::new(Level::Easy);
            return Ok(State::New(Box::new(gameplay)));
        }
        if self.medium {
            let gameplay = GameplayScene::new(Level::Medium);
            return Ok(State::New(Box::new(gameplay)));
        }
        if self.hard {
            let gameplay = GameplayScene::new(Level::Hard);
            return Ok(State::New(Box::new(gameplay)));
        }
        if self.fiendish {
            let gameplay = GameplayScene::new(Level::Fiendish);
            return Ok(State::New(Box::new(gameplay)));
        }
        if self.help {
            let gameplay = HelpScene::default();
            return Ok(State::New(Box::new(gameplay)));
        }
        Ok(State::Keep)
    }
    fn draw(
        &mut self,
        handle: &mut RaylibDrawHandle,
        screen: Rectangle,
        resources: &Resources,
    ) -> anyhow::Result<()> {
        let font: &Font = resources.borrow();
        let clicked =
            handle.is_mouse_button_released(raylib::consts::MouseButton::MOUSE_LEFT_BUTTON);
        let mouse = Vector2::new(handle.get_mouse_x() as f32, handle.get_mouse_y() as f32);

        let camera = Camera2D {
            zoom: 1.0,
            ..Default::default()
        };
        let mut draw = handle.begin_mode2D(camera);

        let background_color = colors::WHEAT;
        draw.clear_background(background_color);

        let size = measure_text_ex(font, "Nanpure", 84.0, 2.0);
        let position = Vector2::new((screen.width - size.x) / 2.0, 0.0);
        let bottom = size.y + 16.0;
        draw.draw_text_ex(font, "Nanpure", position, 84.0, 2.0, colors::DARKCYAN);

        let size = measure_text_ex(font, "(Sudoku)", 32.0, 1.0);
        let position = Vector2::new((screen.width - size.x) / 2.0, bottom);
        let bottom = bottom + size.y + 64.0;
        draw.draw_text_ex(font, "(Sudoku)", position, 32.0, 2.0, colors::DARKCYAN);

        let size = measure_text_ex(font, "Extremely Easy", 64.0, 1.0);
        let position = Vector2::new((screen.width - size.x) / 2.0, bottom);
        let bottom = bottom + 12.0 + size.y;
        let extremely_easy_bt = Rectangle {
            x: position.x,
            y: position.y,
            width: size.x,
            height: size.y,
        };
        let tint = if extremely_easy_bt.check_collision_point_rec(mouse) {
            colors::BLACK
        } else {
            colors::DARKGRAY
        };
        draw.draw_text_ex(font, "Extremely Easy", position, 64.0, 1.0, tint);

        let size = measure_text_ex(font, "Easy", 64.0, 1.0);
        let position = Vector2::new((screen.width - size.x) / 2.0, bottom);
        let bottom = bottom + 12.0 + size.y;
        let easy_bt = Rectangle {
            x: position.x,
            y: position.y,
            width: size.x,
            height: size.y,
        };
        let tint = if easy_bt.check_collision_point_rec(mouse) {
            colors::BLACK
        } else {
            colors::DARKGRAY
        };
        draw.draw_text_ex(font, "Easy", position, 64.0, 1.0, tint);

        let size = measure_text_ex(font, "Medium", 64.0, 1.0);
        let position = Vector2::new((screen.width - size.x) / 2.0, bottom);
        let bottom = bottom + 12.0 + size.y;
        let medium_bt = Rectangle {
            x: position.x,
            y: position.y,
            width: size.x,
            height: size.y,
        };
        let tint = if medium_bt.check_collision_point_rec(mouse) {
            colors::BLACK
        } else {
            colors::DARKGRAY
        };
        draw.draw_text_ex(font, "Medium", position, 64.0, 1.0, tint);

        let size = measure_text_ex(font, "Hard", 64.0, 1.0);
        let position = Vector2::new((screen.width - size.x) / 2.0, bottom);
        let bottom = bottom + 12.0 + size.y;
        let hard_bt = Rectangle {
            x: position.x,
            y: position.y,
            width: size.x,
            height: size.y,
        };
        let tint = if hard_bt.check_collision_point_rec(mouse) {
            colors::BLACK
        } else {
            colors::DARKGRAY
        };
        draw.draw_text_ex(font, "Hard", position, 64.0, 1.0, tint);

        let size = measure_text_ex(font, "Fiendish", 64.0, 1.0);
        let position = Vector2::new((screen.width - size.x) / 2.0, bottom);
        let fiendish_bt = Rectangle {
            x: position.x,
            y: position.y,
            width: size.x,
            height: size.y,
        };
        let tint = if fiendish_bt.check_collision_point_rec(mouse) {
            colors::BLACK
        } else {
            colors::DARKGRAY
        };
        draw.draw_text_ex(font, "Fiendish", position, 64.0, 1.0, tint);

        let url = "https://crates.io/crates/nanpure";
        let size = measure_text_ex(font, url, 12.0, 1.0);
        let position = Vector2::new(screen.width - size.x - 12.0, screen.height - size.y - 12.0);
        let doc_bt = Rectangle {
            x: position.x,
            y: position.y,
            width: size.x,
            height: size.y,
        };
        let tint = if doc_bt.check_collision_point_rec(mouse) {
            colors::BLACK
        } else {
            colors::DARKGRAY
        };
        draw.draw_text_ex(font, url, position, 12.0, 1.0, tint);

        let size = measure_text_ex(font, "Help", 24.0, 1.0);
        let position = Vector2::new(screen.width - size.x - 12.0, position.y - size.y - 12.0);
        let help_bt = Rectangle {
            x: position.x,
            y: position.y,
            width: size.x,
            height: size.y,
        };
        let tint = if help_bt.check_collision_point_rec(mouse) {
            colors::BLACK
        } else {
            colors::DARKGRAY
        };
        draw.draw_text_ex(font, "Help", position, 24.0, 1.0, tint);

        if clicked {
            if extremely_easy_bt.check_collision_point_rec(mouse) {
                self.extremely_easy = true;
            }
            if easy_bt.check_collision_point_rec(mouse) {
                self.easy = true;
            }
            if medium_bt.check_collision_point_rec(mouse) {
                self.medium = true;
            }
            if hard_bt.check_collision_point_rec(mouse) {
                self.hard = true;
            }
            if fiendish_bt.check_collision_point_rec(mouse) {
                self.fiendish = true;
            }
            if help_bt.check_collision_point_rec(mouse) {
                self.help = true;
            }
            if doc_bt.check_collision_point_rec(mouse) {
                self.url = Some(url.to_owned());
            }
        }

        Ok(())
    }
}
