use bevy::{input::{keyboard::KeyboardInput, ButtonState}, prelude::*};

pub fn exit_system(
    mut keyboard: EventReader<KeyboardInput>,
    mut exit: EventWriter<AppExit>,
) {
    for (input, _) in keyboard.par_read() {
        if input.state == ButtonState::Pressed && input.key_code == KeyCode::KeyQ {
            exit.write(AppExit::Success);
        }
    }
}
