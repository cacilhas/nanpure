use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::prelude::*;

use crate::events::NanpureEvent;
use crate::game::Level;

pub fn keybindings_system(
    mut keyboard: EventReader<KeyboardInput>,
    mut exit: EventWriter<AppExit>,
    mut event_writer: EventWriter<NanpureEvent>,
) {
    for (input, _) in keyboard.par_read() {
        if input.state.is_pressed() {
            if !input.repeat && input.logical_key == Key::Escape {
                exit.write(AppExit::Success);
                return;
            }

            'levels: for num in 1..=5 {
                if let Some(text) = &input.text {
                    if text.to_string() == num.to_string() {
                        let level: Level = (num as u8).into();
                        event_writer.write(NanpureEvent::StartGame(level));
                        break 'levels;
                    }
                }
            }
        }
    }
}
