extern crate raylib_ffi as raylib;
mod colors;
mod scene;

use std::os::raw::c_int;

use crate::scene::{Action, Scene, StartMenu};
use raylib::{enums::KeyboardKey, rl_str};

#[cfg(any(target_os = "linux", target_os = "macos"))]
fn main() -> eyre::Result<()> {
    unsafe {
        raylib::InitWindow(640, 720, rl_str!("nanpure")); // WM_CLASS
        raylib::SetWindowTitle(rl_str!("Kodumaro Nanpūrë"));
        raylib::SetTargetFPS(30);
        raylib::SetConfigFlags(0);
        raylib::SetExitKey(KeyboardKey::Null as c_int);

        let mut scenes: Vec<Box<dyn Scene>> = vec![Box::new(StartMenu::default())];

        while !raylib::WindowShouldClose() {
            match scenes.last_mut() {
                Some(scene) => match scene.run_step() {
                    Action::Keep => (),
                    Action::Pop => {
                        scenes.pop();
                    }
                    Action::Push(next_scene) => scenes.push(next_scene),
                },
                None => raylib::CloseWindow(),
            }
        }
    }
    Ok(())
}
