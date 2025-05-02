use bevy::prelude::*;
use bevy::ecs::error::Result;

use crate::consts::{CANDIDATE_SIZE, CELL_SIZE};
use crate::fonts::{RegularFont, TitleFont};
use crate::game::{Colors, Shapes};
use crate::gameplay::Paused;
use crate::states::GameState;

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

        commands.insert_resource(Paused(false));

        let cell_shapes = Shapes {
            rect: meshes.add(Rectangle::new(CELL_SIZE, CELL_SIZE)),
            cell: meshes.add(Circle::new(CELL_SIZE/ 2.0 - 2.0)),
            cell_candidate: meshes.add(Circle::new(CANDIDATE_SIZE / 2.0 - 2.0)),
        };
        commands.insert_resource(cell_shapes);

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

pub static COLORS: [Color; 11] = [
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
];
