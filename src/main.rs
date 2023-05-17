#[macro_use]
extern crate static_init;

mod error;
mod game;
mod ui;

use crate::ui::{
    fonts,
    scene::{main_menu::MainMenuScene, Scene, State},
};
use chrono::prelude::*;
use raylib::prelude::*;

#[cfg(any(target_os = "linux", target_os = "macos"))]
fn main() -> anyhow::Result<()> {
    use std::{cell::RefCell, rc::Rc};

    let (mut handle, thr) = raylib::init()
        .size(640, 720) // desired dimensions
        .title("nanpure") // WM_CLASS
        .build();

    handle.set_target_fps(30);
    handle.set_window_title(&thr, "Kodumaro Nanpure");
    handle.set_exit_key(None);

    let font: Rc<Font> = fonts::get_font(&mut handle, &thr)?.into();
    let mut main_scene = MainMenuScene::default();
    main_scene.init(&mut handle, &thr, font.clone());
    let mut scenes: Vec<Rc<RefCell<dyn Scene>>> = vec![Rc::new(RefCell::new(main_scene))];
    let mut tick = Utc::now();

    while !handle.window_should_close() {
        let new_tick = Utc::now();
        let state = {
            let mut draw = handle.begin_drawing(&thr);
            if draw.is_key_released(KeyboardKey::KEY_ESCAPE) {
                scenes.pop();
            }
            let scene = match scenes.last() {
                Some(scene) => scene,
                None => break,
            };
            scene
                .borrow_mut()
                .update(new_tick.signed_duration_since(tick), &mut draw)
        };
        match state {
            State::New(scene) => {
                scene.borrow_mut().init(&mut handle, &thr, font.clone());
                scenes.push(scene);
            }
            State::Keep => (),
        }
        tick = new_tick;
    }
    Ok(())
}
