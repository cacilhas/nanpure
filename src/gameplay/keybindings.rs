use bevy::ecs::error::Result;
use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::prelude::*;

use crate::events::NanpureEvent;
use crate::game::BoardWrapper;

use super::Gameplay;

pub fn keybindings_system(
    mut keyboard: EventReader<KeyboardInput>,
    mut event_writer: EventWriter<NanpureEvent>,
    mut board_query: Query<&mut BoardWrapper, With<Gameplay>>,
) -> Result<()> {
    let mut board = board_query.single_mut()?;
    for (input, _) in keyboard.par_read() {
        if input.state.is_pressed() && !input.repeat {
            match input.logical_key {
                Key::Escape => {
                    event_writer.write(NanpureEvent::AbortGame);
                    return Ok(());
                }

                Key::Pause => {
                    event_writer.write(NanpureEvent::PauseGame);
                    return Ok(());
                }

                Key::ArrowUp => {
                    let (xi, yi) = board.highlight()?;
                    board.set_highlight(xi, yi + 1);
                }

                Key::ArrowDown => {
                    let (xi, yi) = board.highlight()?;
                    board.set_highlight(xi, yi - 1);
                }

                Key::ArrowLeft => {
                    let (xi, yi) = board.highlight()?;
                    board.set_highlight(xi - 1, yi);
                }

                Key::ArrowRight => {
                    let (xi, yi) = board.highlight()?;
                    board.set_highlight(xi + 1, yi);
                }

                _ => (),
            }
        }
    }
    Ok(())
}
