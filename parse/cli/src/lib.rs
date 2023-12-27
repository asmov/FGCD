use std::{path::PathBuf, ffi::OsStr};
use fgcd_model::{Models, CHARACTERS,GAMES};

pub mod spreadsheet;
pub mod binary;
pub mod cli;
pub mod submodels;

pub fn games_data_dir<P>(data_dir: &P) -> PathBuf
where
    P: ?Sized + AsRef<OsStr>
{
    PathBuf::from(data_dir)
        .join(GAMES)
}

pub fn game_data_dir<P>(game_name: &str, data_dir: &P) -> PathBuf
where
    P: ?Sized + AsRef<OsStr>
{
    games_data_dir(data_dir).join(game_name)
}

pub fn game_filepath<P>(game_name: &str, data_dir: &P, extension: &str) -> PathBuf
where
    P: ?Sized + AsRef<OsStr>
{
    game_data_dir(game_name, data_dir)
        .join(Models::Game.name().to_string() + extension)
}

pub fn characters_data_dir<P>(game_name: &str, data_dir: &P) -> PathBuf
where
    P: ?Sized + AsRef<OsStr>
{
    game_data_dir(game_name, data_dir)
        .join(CHARACTERS)
}

pub fn character_filepath<P>(character_name: &str, game_name: &str, data_dir: &P, extension: &str) -> PathBuf
where
    P: ?Sized + AsRef<OsStr>
{
    characters_data_dir(game_name, data_dir)
        .join(character_name.to_string() + extension)
}