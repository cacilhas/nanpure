use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::prelude::*;


pub fn keybindings_system(
    mut keyboard: EventReader<KeyboardInput>,
    mut exit: EventWriter<AppExit>,
) {
    for (input, _) in keyboard.par_read() {
        if input.state.is_pressed() && !input.repeat && input.logical_key == Key::Escape {
            exit.write(AppExit::Success);
        }
    }
}
