use std::path::{PathBuf,Path};
use std::fs;
use std::process;
use anyhow::Result;

use crate::games_data_dir;

fn project_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent().unwrap()
        .parent().unwrap()
        .to_path_buf()
}


fn project_data_dir() -> PathBuf {
    project_dir().join("data")
}


fn project_target_data_dir() -> PathBuf {
    project_data_dir()
        .join("target")
        .join("data")
}


pub fn run() -> Result<(u8, u8)> {
    let data_dir = compile_data()?;
    let games_data_dir = games_data_dir(&data_dir).canonicalize()?;

    let mut num_processed = (0, 0);
    for entry in fs::read_dir(games_data_dir)? {
        let dir = entry?.path();
        if !dir.is_dir() {
            continue;
        }

        let game_name = dir.file_name().unwrap().to_str().unwrap();
        num_processed.1 += process_game(game_name, &data_dir)?;
        num_processed.0 += 1;
    }

    Ok(num_processed)
}

fn compile_data() -> Result<PathBuf> {
    let build_script = project_data_dir().join("bin").join("build.py")
        .canonicalize()?;


    process::Command::new("python3")
        .arg(build_script.to_str().unwrap().to_string())
        .output()?;

    Ok(project_target_data_dir())
}

fn process_game(game_name: &str, data_dir: &Path) -> Result<u8> {
    let game = crate::spreadsheet::game::read_game(game_name, &data_dir)?;    
    crate::binary::game::write_game(&game, &data_dir).unwrap();

    let mut num_processed = 0;
    for character_name in game.character_names() {
        if !crate::character_filepath(character_name, game_name, data_dir, crate::spreadsheet::EXT_FODS).exists() {
            continue;
        }

        let character = crate::spreadsheet::game::character::read_character(character_name, &game, &data_dir)?;
        crate::binary::game::character::write_character(&character, &game, &data_dir)?;
        num_processed += 1;
    }

    Ok(num_processed)
}

