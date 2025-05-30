use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::prelude::*;

use crate::consts::{BACKGROUND_COLOR, WIN_COLOR};
use crate::game::{Colors, Cursor};
use crate::gameplay::{BGFlag, ClockDisplay, Gameplay};
use crate::states::GameState;

#[derive(Debug, Clone, Resource)]
pub struct GameOverCheck(pub bool);

#[derive(Debug, Clone, Copy)]
pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::GameOver), GameOver::load)
            .add_systems(OnExit(GameState::GameOver), GameOver::unload)
            .add_systems(Update, GameOver::keybinding.run_if(in_state(GameState::GameOver)))
        ;
    }
}

#[derive(Debug, Clone, Copy, Component)]
struct GameOver;

impl GameOver {

    fn load(
        mut commands: Commands,
        mut bg_query: Query<&mut BackgroundColor>,
        mut fg_query: Query<&mut MeshMaterial2d<ColorMaterial>, With<BGFlag>>,
        mut clock_query: Query<&mut Visibility, With<ClockDisplay>>,
        cursor_query: Query<Entity, With<Cursor>>,
        colors: Res<Colors>,
    ) {
        if let Ok(cursor) = cursor_query.single() {
            commands.entity(cursor).despawn();
        }
        if let Ok(mut color) = bg_query.single_mut() {
            color.0 = WIN_COLOR.clone();
        }
        if let Ok(mut clock) = clock_query.single_mut() {
            *clock = Visibility::Visible;
        }
        for mut color in &mut fg_query {
            color.0 = colors.win().clone_weak();
        }
    }

    fn unload(
        mut commands: Commands,
        mut query: Query<Entity, With<Gameplay>>,
        mut bg_query: Query<&mut BackgroundColor>,
    ) {
        if let Ok(mut color) = bg_query.single_mut() {
            color.0 = BACKGROUND_COLOR.clone();
        }
        for entity in query.iter_mut() {
            commands.entity(entity).despawn();
        }
    }


    fn keybinding(
        mut keyboard: EventReader<KeyboardInput>,
        mut next_state: ResMut<NextState<GameState>>,
    ) {
        for (input, _) in keyboard.par_read() {
            if input.state.is_pressed() && !input.repeat {
                match input.logical_key {
                    Key::Escape => {
                        next_state.set(GameState::Title);
                        return;
                    },
                    _ => (),
                }
            }
        }
    }}
