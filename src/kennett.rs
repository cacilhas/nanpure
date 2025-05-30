// Michael Kennett Sudoku connector
use crate::game::Level;
use std::{
    io::{Error, ErrorKind},
    process::Command,
};

pub struct KennettConnector;

impl KennettConnector {
    pub fn generate(level: Level) -> Result<[u8; 81], std::io::Error> {
        let output = Command::new("sudoku")
            .args(["-g", level.kennett_flag()])
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
            Err(Error::new(ErrorKind::Unsupported, "wrong size").into())
        } else {
            let mut res = [0u8; 81];
            for (i, &c) in output.iter().enumerate() {
                res[i] = c;
            }
            Ok(res)
        }
    }
}
