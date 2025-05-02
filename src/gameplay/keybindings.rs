use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::prelude::*;

use crate::events::NanpureEvent;

pub fn keybindings_system(
    mut keyboard: EventReader<KeyboardInput>,
    mut event_writer: EventWriter<NanpureEvent>,
) {
    for (input, _) in keyboard.par_read() {
        if input.state.is_pressed() && !input.repeat {
            match input.logical_key {
                Key::Escape => {
                    event_writer.write(NanpureEvent::AbortGame);
                    return;
                }

                Key::Pause => {
                    event_writer.write(NanpureEvent::PauseGame);
                    return;
                }

                _ => (),
            }
        }
    }
}
