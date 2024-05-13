extern crate raylib_ffi as raylib;
#[macro_use]
extern crate static_init;

mod colors;
mod fonts;
mod game;
mod kennett;
mod scene;
mod themes;

use std::os::raw::c_int;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::scene::{Action, Scene, StartMenu};
use raylib::{draw, enums::KeyboardKey, rl_str};

#[cfg(any(target_os = "linux", target_os = "macos"))]
fn main() -> eyre::Result<()> {
    unsafe {
        raylib::InitWindow(640, 768, rl_str!("nanpure")); // WM_CLASS
        raylib::SetWindowTitle(rl_str!("Kodumaro Nanpūrë"));
        raylib::SetTargetFPS(30);
        raylib::SetConfigFlags(0);
        raylib::SetExitKey(KeyboardKey::Null as c_int);
        raylib::SetRandomSeed((get_seed()? as c_int).try_into()?);

        let mut scenes: Vec<Box<dyn Scene>> = vec![Box::new(StartMenu::default())];

        while !raylib::WindowShouldClose() {
            draw! {
                match scenes.last_mut() {
                    Some(scene) => match scene.run_step()? {
                        Action::Keep => (),
                        Action::Pop(count) => {
                            for _ in 0..count {
                                scenes.pop();
                            }
                        }
                        Action::Push(next_scene) => scenes.push(next_scene),
                    },
                    None => raylib::CloseWindow(),
                }
            }
        }
    }
    Ok(())
}

fn get_seed() -> eyre::Result<i32> {
    let seed = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
    let max: u64 = u32::max_value().into();
    let seed: u32 = (seed % max).try_into()?;
    let seed: i32 = if seed > i32::max_value() as u32 {
        max - (seed as u64)
    } else {
        seed.into()
    }
    .try_into()?;
    Ok(seed)
}
