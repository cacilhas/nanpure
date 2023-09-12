// Michael Kennett Sudoku connector
use crate::error;
use crate::game::level::Level;
use std::process::Command;

pub struct KennettConnector;

impl KennettConnector {
    pub fn generate(level: Level) -> Option<Vec<u8>> {
        Self::do_generate(level).ok()
    }

    fn do_generate(level: Level) -> eyre::Result<Vec<u8>> {
        let output = Command::new("sudoku")
            .args(["-g", level.kennet_flag()])
            .output()?
            .stdout
            .into_iter()
            .filter_map(|c| {
                if c == 46_u8 {
                    Some(0_u8)
                } else if c >= 48_u8 && c < 58_u8 {
                    Some(c - 48_u8)
                } else {
                    None
                }
            })
            .collect::<Vec<u8>>();
        if output.len() != 81 {
            Err(error!("wrong size").into())
        } else {
            Ok(output)
        }
    }
}
