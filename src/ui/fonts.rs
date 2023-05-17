#[cfg(target_os = "linux")]
use std::{env, process::Command};

use raylib::prelude::*;
#[cfg(target_os = "linux")]
use walkdir::WalkDir;

use crate::error;

pub fn get_font(handle: &mut RaylibHandle, thr: &RaylibThread) -> anyhow::Result<Font> {
    let font_name = find_gnome_font()?;
    let path = find_font_path(font_name)?;
    Ok(handle.load_font(thr, &path).map_err(error::Error)?)
}

#[cfg(target_os = "linux")]
fn find_font_path(font_name: Vec<String>) -> anyhow::Result<String> {
    for directory in FONT_DIRS.iter() {
        'entry: for entry in WalkDir::new(directory).into_iter().filter_map(Result::ok) {
            if let Some(current) = entry.file_name().to_str() {
                if current.ends_with(".ttf") {
                    for e in font_name.iter() {
                        if !current.contains(e) {
                            continue 'entry;
                        }
                    }
                    return Ok(current.to_owned());
                }
            }
        }
    }
    Err(error!("could not find {font_name:?}").into())
}

#[cfg(target_os = "macos")]
fn find_font_path(font_name: Vec<String>) -> anyhow::Result<String> {
    let font = font_name
        .first()
        .ok_or_else(|| error!("no font supplied"))?;
    Ok(format!("/System/Library/Fonts/{}", font))
}

#[cfg(target_os = "linux")]
fn find_gnome_font() -> anyhow::Result<Vec<String>> {
    let output = Command::new("gsettings")
        .arg("get")
        .arg("org.gnome.desktop.interface")
        .arg("font-name")
        .output()?;

    if output.status.success() {
        let font = output
            .stdout
            .into_iter()
            .filter(|e| e.is_ascii_digit() || e.is_ascii_lowercase() || e.is_ascii_uppercase())
            .collect::<Vec<u8>>();
        let font = String::from_utf8_lossy(&font).trim().to_owned();
        let font = font.split_whitespace().collect::<Vec<&str>>();
        let font = font[..font.len() - 1]
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<String>>();
        Ok(font)
    } else {
        let err = String::from_utf8_lossy(&output.stderr).trim().to_owned();
        Err(error!("error getting org.gnome.desktop.interface[font-name]: {err}").into())
    }
}
#[cfg(target_os = "macos")]
fn find_gnome_font() -> anyhow::Result<Vec<String>> {
    Ok(vec!["SFCompactRounded.ttf".to_owned()])
}

#[cfg(target_os = "linux")]
#[dynamic]
static FONT_DIRS: [String; 2] = [
    format!(
        "{}/fonts",
        env::var("XDG_DATA_HOME").unwrap_or(format!("{}/.local/share", env!["HOME"]))
    ),
    "/usr/share/fonts".to_owned(),
];
