#[macro_use]
extern crate static_init;

mod error;
mod game;
mod ui;

use crate::ui::{fonts, scene::main_menu::MainMenuScene};
use rscenes::prelude::*;

#[cfg(any(target_os = "linux", target_os = "macos"))]
fn main() -> anyhow::Result<()> {
    use std::rc::Rc;

    let mut builder = raylib::init();
    builder
        .size(640, 720) // desired dimensions
        .title("nanpure"); // WM_CLASS
    let mut manager = SceneManager::new(builder);
    let font = manager.config(|handle, thread| {
        handle.set_window_title(thread, "Kodumaro Nanpure");
        fonts::get_font(handle, thread)
    })?;
    let font = Rc::new(font);
    manager.set_font(&font);
    manager.add_first_scene(Box::new(MainMenuScene::default()));
    manager.start()
}
