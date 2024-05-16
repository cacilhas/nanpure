use raylib::{Rectangle, Vector2};

pub(super) unsafe fn get_screen() -> Rectangle {
    let screen = Vector2 {
        x: raylib::GetScreenWidth() as f32,
        y: raylib::GetScreenHeight() as f32,
    };

    if screen.x > screen.y {
        Rectangle {
            x: (screen.x - screen.y) / 2.0,
            y: 0.0,
            width: screen.y,
            height: screen.y,
        }
    } else {
        Rectangle {
            x: 0.0,
            y: 0.0,
            width: screen.x,
            height: screen.y,
        }
    }
}
