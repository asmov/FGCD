use std::{path::PathBuf, io::{BufWriter, BufReader}, fs::File, ffi::OsStr};
use anyhow::Result;
use bincode;
use fgcd_model::game::Game as Game;
use super::super::*;
use super::*;

pub mod character;


/**
 * @param path Path to the FGCD data directory or the Game data file
 */
pub fn read_game<P>(game_name: &str, path: &P) -> Result<Game>
where
    P: ?Sized + AsRef<OsStr>
{
    let path = PathBuf::from(path);
    let path = if path.is_file() { path } else { game_filepath(game_name, &path, EXT_BIN) };
 
    let bufreader = BufReader::new(File::open(path)?);
    let game = bincode::deserialize_from(bufreader)?;
    Ok(game)
}

/**
 * @param path Path to the FGCD data directory or the Game data file
 */
pub fn write_game<P>(game: &Game, path: &P) -> Result<()>
where
    P: ?Sized + AsRef<OsStr>
{
    let path = PathBuf::from(path);
    let path = if path.is_file() { path } else { game_filepath(game.name(), &path, EXT_BIN) };
 
    let mut bufwriter = BufWriter::new(File::create(path)?);
    bincode::serialize_into(&mut bufwriter, &game)?;
    Ok(())
}

pub fn read_game_bytes(bytes: &[u8]) -> Result<Game> {
    Ok(bincode::deserialize(bytes)?)
}