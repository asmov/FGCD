use std::{path::PathBuf, io::{BufWriter, BufReader}, fs::File, ffi::OsStr};
use bincode;
use fgcd_model::game::{Game, Character};
use super::super::*;
use super::*;

/**
 * @param path Path to the FGCD data directory or the Game data file
 */
pub fn read_character<P>(character_name: &str, game: &Game, path: &P) -> Result<Character>
where
    P: ?Sized + AsRef<OsStr>
{
    let path = PathBuf::from(path);
    let path = if path.is_file() { path } else { character_filepath(character_name, game.name(), &path, EXT_BIN) };
 
    let bufreader = BufReader::new(File::open(path)?);
    let character = bincode::deserialize_from(bufreader)?;
    Ok(character)
}

/**
 * @param path Path to the FGCD data directory or the Game data file
 */
pub fn write_character<P>(character: &Character, game: &Game, path: &P) -> Result<()>
where
    P: ?Sized + AsRef<OsStr>
{
    let path = PathBuf::from(path);
    let path = if path.is_file() { path } else { character_filepath(character.name(), game.name(), &path, EXT_BIN) };
 
    let mut bufwriter = BufWriter::new(File::create(path)?);
    bincode::serialize_into(&mut bufwriter, &character)?;
    Ok(())
}

pub fn read_character_bytes(_game: &Game, bytes: &[u8]) -> Result<Character> {
    Ok(bincode::deserialize(bytes)?)
}
