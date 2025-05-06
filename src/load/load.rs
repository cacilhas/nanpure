use bevy::prelude::*;
use bevy::ecs::error::Result;

use crate::consts::{BACKGROUND_COLOR, CANDIDATE_SIZE, CELL_SIZE, WIN_COLOR};
use crate::fonts::{MonospaceFont, RegularFont, TitleFont};
use crate::game::{Colors, Shapes};
use crate::gameover::GameOverCheck;
use crate::gameplay::{Clock, Paused};
use crate::states::GameState;

use super::ctrl::Ctrl;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct Load;

impl Load {
    pub fn init(
        mut commands: Commands,
        assets: Res<AssetServer>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
    ) -> Result<()> {
        let font = TitleFont::new(&assets)?;
        commands.insert_resource(font);

        let font = RegularFont::new(&assets)?;
        commands.insert_resource(font);

        let font = MonospaceFont::new(&assets)?;
        commands.insert_resource(font);

        commands.insert_resource(Paused(false));
        commands.insert_resource(GameOverCheck(false));

        let cell_shapes = Shapes {
            rect: meshes.add(Rectangle::new(CELL_SIZE, CELL_SIZE)),
            full_bg_rect: meshes.add(Rectangle::new(CELL_SIZE * 10.0, CELL_SIZE * 9.5)),
            cell: meshes.add(Circle::new(CELL_SIZE / 2.0 - 4.0)),
            cell_candidate: meshes.add(Circle::new(CANDIDATE_SIZE / 2.0 - 2.0)),
            vertical_line: meshes.add(Rectangle::new(9.0, CELL_SIZE * 9.0 + 4.5)),
            horizontal_line: meshes.add(Rectangle::new(CELL_SIZE * 9.0 + 4.5, 9.0)),
        };
        commands.insert_resource(cell_shapes);
        commands.insert_resource(Ctrl(false));
        commands.insert_resource(Clock::default());

        let colors = COLORS.iter()
            .map(|color| materials.add(*color))
            .collect::<Vec<_>>();
        commands.insert_resource(Colors::new(colors));

        Ok(())
    }

    pub fn load_title(mut next_state: ResMut<NextState<GameState>>) {
        next_state.set(GameState::Title);
    }
}

pub static COLORS: [Color; 14] = [
    Color::BLACK,
    Color::Srgba(Srgba { red: 1.0, green: 0.0, blue: 0.0, alpha: 1.0 }),
    Color::Srgba(Srgba { red: 1.0, green: 0.5, blue: 0.0, alpha: 1.0 }),
    Color::Srgba(Srgba { red: 1.0, green: 1.0, blue: 0.0, alpha: 1.0 }),
    Color::Srgba(Srgba { red: 0.0, green: 1.0, blue: 0.0, alpha: 1.0 }),
    Color::Srgba(Srgba { red: 0.0, green: 1.0, blue: 1.0, alpha: 1.0 }),
    Color::Srgba(Srgba { red: 0.0, green: 0.0, blue: 1.0, alpha: 1.0 }),
    Color::Srgba(Srgba { red: 0.8, green: 0.2, blue: 1.0, alpha: 1.0 }),
    Color::Srgba(Srgba { red: 1.0, green: 0.0, blue: 1.0, alpha: 1.0 }),
    Color::Srgba(Srgba { red: 0.5, green: 0.5, blue: 0.5, alpha: 1.0 }),
    Color::WHITE,
    BACKGROUND_COLOR,
    Color::Srgba(Srgba { red: 1.0, green: 0.75, blue: 0.875, alpha: 1.0 }),
    WIN_COLOR,
];
