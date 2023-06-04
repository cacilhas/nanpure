//! [COPYING]: https://github.com/cacilhas/nanpure/blob/master/COPYING
//! [email]: mailto:montegasppa@cacilhas.info
//! [The 3-Clause BSD License]: https://opensource.org/license/bsd-3-clause/
//! [Sudoku]: https://en.wikipedia.org/wiki/Sudoku
//!
//! # Nanpure
//!
//! Yet another (colourful) [Sudoku][] playing game.
//!
//! ## Installation
//!
//! ```sh
//! cargo install nanpure
//! ```
//!
//! ## Controls
//!
//! - Cursor keys or WASD: select cell
//! - Numbers: toggle candidates
//! - Control + number (or Shift + number): set cell value
//! - Control + 0 (or Shift + 0): clean cell value up
//! - Space: alias to toggle single-value cell
//! - F1: help
//! - Escape: back or quit
//!
//! No mouse during gameplay.
//!
//! ## Colours
//!
//! - 1: <span style="color: white; background-color: red;">red</span>
//! - 2: <span style="color: black; background-color: orange;">orange</span>
//! - 3: <span style="color: black; background-color: yellow;">yellow</span>
//! - 4: <span style="color: white; background-color: green;">green</span>
//! - 5: <span style="color: black; background-color: skyblue;">blue</span>
//! - 6: <span style="color: white; background-color: indigo;">indigo</span>
//! - 7: <span style="color: black; background-color: violet;">violet</span>
//! - 8: <span style="color: white; background-color: magenta;">magenta</span>
//! - 9: <span style="color: black: background-color: darkgray;">gray</span>
//!
//! ## License
//!
//! - Copyright 2023 [Rodrigo Cacilhας \<montegasppa@cacilhas.info\>][email]
//! - [The 3-Clause BSD License][]
//! - [COPYING][]

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
    manager.push_font("main", &Rc::new(font));
    manager.add_first_scene(Box::new(MainMenuScene::default()));
    manager.start()
}
