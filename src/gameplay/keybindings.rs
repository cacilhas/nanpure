use bevy::ecs::error::Result;
use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::prelude::*;

use crate::events::NanpureEvent;
use crate::game::Board;
use crate::load::Ctrl;

use super::gameplay::Gameplay;

pub fn keybindings_system(
    mut keyboard: EventReader<KeyboardInput>,
    mut event_writer: EventWriter<NanpureEvent>,
    mut board_query: Query<&mut Board, With<Gameplay>>,
    ctrl: Res<Ctrl>,
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

                _ => if let Some(text) = &input.text {
                    let text = text.as_str();
                    if text == "u" || text == "U" {
                        if board.undo() {
                            event_writer.write(NanpureEvent::RenderBoard);
                            return Ok(());
                        }
                    }
                },
            }

            let rerender = match input.key_code {
                KeyCode::Numpad0 => board.set_value(0)?,
                KeyCode::Digit0 => board.set_value(0)?,
                KeyCode::Numpad1 if ctrl.0 => board.toggle_candidate(1)?,
                KeyCode::Numpad1 => board.set_value(1)?,
                KeyCode::Digit1 if ctrl.0 => board.toggle_candidate(1)?,
                KeyCode::Digit1 => board.set_value(1)?,
                KeyCode::Numpad2 if ctrl.0 => board.toggle_candidate(2)?,
                KeyCode::Numpad2 => board.set_value(2)?,
                KeyCode::Digit2 if ctrl.0 => board.toggle_candidate(2)?,
                KeyCode::Digit2 => board.set_value(2)?,
                KeyCode::Numpad3 if ctrl.0 => board.toggle_candidate(3)?,
                KeyCode::Numpad3 => board.set_value(3)?,
                KeyCode::Digit3 if ctrl.0 => board.toggle_candidate(3)?,
                KeyCode::Digit3 => board.set_value(3)?,
                KeyCode::Numpad4 if ctrl.0 => board.toggle_candidate(4)?,
                KeyCode::Numpad4 => board.set_value(4)?,
                KeyCode::Digit4 if ctrl.0 => board.toggle_candidate(4)?,
                KeyCode::Digit4 => board.set_value(4)?,
                KeyCode::Numpad5 if ctrl.0 => board.toggle_candidate(5)?,
                KeyCode::Numpad5 => board.set_value(5)?,
                KeyCode::Digit5 if ctrl.0 => board.toggle_candidate(5)?,
                KeyCode::Digit5 => board.set_value(5)?,
                KeyCode::Numpad6 if ctrl.0 => board.toggle_candidate(6)?,
                KeyCode::Numpad6 => board.set_value(6)?,
                KeyCode::Digit6 if ctrl.0 => board.toggle_candidate(6)?,
                KeyCode::Digit6 => board.set_value(6)?,
                KeyCode::Numpad7 if ctrl.0 => board.toggle_candidate(7)?,
                KeyCode::Numpad7 => board.set_value(7)?,
                KeyCode::Digit7 if ctrl.0 => board.toggle_candidate(7)?,
                KeyCode::Digit7 => board.set_value(7)?,
                KeyCode::Numpad8 if ctrl.0 => board.toggle_candidate(8)?,
                KeyCode::Numpad8 => board.set_value(8)?,
                KeyCode::Digit8 if ctrl.0 => board.toggle_candidate(8)?,
                KeyCode::Digit8 => board.set_value(8)?,
                KeyCode::Numpad9 if ctrl.0 => board.toggle_candidate(9)?,
                KeyCode::Numpad9 => board.set_value(9)?,
                KeyCode::Digit9 if ctrl.0 => board.toggle_candidate(9)?,
                KeyCode::Digit9 => board.set_value(9)?,

                _ => false,
            };

            if rerender {
                event_writer.write(NanpureEvent::RenderBoard);
            }
        }
    }
    Ok(())
}
