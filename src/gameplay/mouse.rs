use bevy::ecs::error::Result;
use bevy::prelude::*;

use crate::consts::CELL_SIZE;
use crate::game::Board;

use super::clock::ClockDisplay;

pub fn mouse_system(
    window_query: Query<&Window>,
    click_event: Res<ButtonInput<MouseButton>>,
    mut board_query: Query<&mut Board>,
    mut clock_query: Query<&mut Visibility, With<ClockDisplay>>,
) -> Result<()> {
    let window = window_query.single()?;
    if let Some(mouse) = window.cursor_position() {
        let x = (mouse.x / CELL_SIZE - 0.5) as i32;
        let y = 9 - (mouse.y / CELL_SIZE + 0.75) as i32;

        if let Ok(mut clock) = clock_query.single_mut() {
            *clock = if y < 0 {
                Visibility::Visible
            } else {
                Visibility::Hidden
            };
        }

        if click_event.just_pressed(MouseButton::Left) {
            if x >= 0 && x < 9 && y >= 0 && y < 9 {
                let mut board = board_query.single_mut()?;
                board.set_highlight(x, y);
                return Ok(());
            }
        }
    }
    Ok(())
}
