use raylib::{draw, rl_str};
use raylib_ffi as raylib;

#[cfg(any(target_os = "linux", target_os = "macos"))]
fn main() -> Result<(), String> {
    unsafe {
        raylib::InitWindow(640, 720, rl_str!("nanpure")); // WM_CLASS
        raylib::SetWindowTitle(rl_str!("Kodumaro Nanpūrë"));
        raylib::SetTargetFPS(30);
        raylib::SetConfigFlags(0);

        while !raylib::WindowShouldClose() {
            draw! {
                print!(".")
            }
        }
    }
    Ok(())
}
