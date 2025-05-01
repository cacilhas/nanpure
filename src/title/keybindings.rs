use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::prelude::*;
use bevy_rand::commands;

use crate::game::Level;
use crate::states::GameState;


pub fn keybindings_system(
    mut commands: Commands,
    mut keyboard: EventReader<KeyboardInput>,
    mut exit: EventWriter<AppExit>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (input, _) in keyboard.par_read() {
        if input.state.is_pressed() {
            if !input.repeat && input.logical_key == Key::Escape {
                exit.write(AppExit::Success);
            }

            for num in 1..=5 {
                if let Some(text) = &input.text {
                    if text.to_string() == num.to_string() {
                        let level: Level = (num as u8).into();
                        next_state.set(GameState::Playing);
                        commands.spawn((
                            // TODO: gameplay object
                            level,
                        ));
                        break;
                    }
                }
            }
        }
    }
}
