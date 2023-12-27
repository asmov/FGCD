use std::{path::{PathBuf,Path}, str::FromStr};
use anyhow;
use model::Title;
use strum::IntoEnumIterator;
use fgcd_model as model;
use crate::{spreadsheet::*, submodels::{SubmodelParser, GameSubmodelSpreadsheetParserTrait}};
use fgcd_parse_macro::{SheetEnum,HeadingEnum};
use spreadsheet_ods as ods;



#[derive(SheetEnum, strum::EnumIter)]
enum Sheets {
    Profile, 
    Moves,
}

#[derive(HeadingEnum, strum::EnumIter)]
#[headings(sheet(Sheets::Moves))]
enum MovesHeading {
    #[heading(row(0), column(0))]
    Name,
    #[heading(row(0), column(1))]
    Category,
    #[heading(row(0), column(2))]
    During,
    #[heading(row(0), column(3))]
    Inputs,
    #[heading(row(0), column(4))]
    Notes,
}

#[derive(HeadingEnum, strum::EnumIter)]
#[headings(sheet(Sheets::Profile))]
enum ProfileHeading {
    #[heading(row(0), column(0))]
    Name,
}

/**
 * @param path Either the FGCD data directory or the file path for the character data file
 */
pub fn read_character(character_name: &str, game: &model::Game, path: &Path) -> anyhow::Result<model::Character> {
    let path = if path.is_file() {
        path.to_path_buf()
    } else {
        crate::character_filepath(character_name, game.name(), &path, EXT_FODS)
    };

    let workbook = ods::read_fods(path)?;

    let moves = read_character_moves(game, &workbook)?;
    let submodel = read_character_submodel(game, &workbook)?;

    Ok(model::Character::new(
        character_name.to_string(),
        moves,
        submodel
    ))
}

fn read_character_moves(game: &model::Game, workbook: &ods::WorkBook) -> anyhow::Result<Vec<model::input::NamedSequence>> {
    let sheet = sheet(Sheets::Moves.title(), &workbook)?;
    let mut moves: Vec<model::input::NamedSequence> = Vec::new();

    for index in 0.. {
        let name = match cell_string(MovesHeading::Name, index, sheet) {
            Ok(n)  => n,
            Err(_) => break
        };

        let category_name = cell_string(MovesHeading::Category, index, sheet)?;
        let category = game.find_input_category(&category_name)
            .context(format!("Move category not found for game `{}`: {}", game.name(), category_name))?;
        let sequence_str = cell_string(MovesHeading::Inputs, index, sheet)?;
        let sequence = model::input::Sequence::parse(&sequence_str, game, game)?;
        let notes = cell_string_optional(MovesHeading::Notes, index, sheet)?;

        moves.push(model::input::NamedSequence::new(name, sequence, category, notes));

    }

    Ok(moves)
}

fn read_character_submodel(game: &model::Game, workbook: &ods::WorkBook)
    -> anyhow::Result<model::CharacterSubmodel>
{
    if let Ok(title) = model::Title::from_str(game.name()) {
        SubmodelParser::from(title)
            .read_character_submodel_spreadsheet(game, workbook)
    } else {
        Ok(model::CharacterSubmodel::Generic)
    }
}

pub fn new_character(game: &model::Game, character_name: &str, data_dir: &Path) -> anyhow::Result<(ods::WorkBook, PathBuf)> {
    let characters_data_dir = crate::characters_data_dir(game.name(), data_dir);

    let mut builder = WorkbookBuilder::new();
    builder.append_sheets(Sheets::iter());

    if let Ok(title) = model::Title::from_str(character_name) {
        SubmodelParser::from(title)
            .new_character_submodel_spreadsheet(character_name, game, &mut builder)?
    }   

    builder.write(character_name, &characters_data_dir)
}

pub fn write_character(character: &model::Character, game: &model::Game, data_dir: &Path)
        -> anyhow::Result<(ods::WorkBook, PathBuf)> {
    let characters_data_dir = crate::characters_data_dir(game.name(), data_dir);
    
    let mut builder = WorkbookBuilder::new();
    builder.append_sheets(Sheets::iter());
    write_character_profile(character, game, &mut builder)?;
    write_character_moves(character, game, &mut builder)?;

    if let Ok(title) = Title::from_str(game.name()) {
        SubmodelParser::from(title)
            .write_character_submodel_spreadsheet(character, game, &mut builder)?;
    }


    builder.write(character.name(), &characters_data_dir)
}

fn write_character_profile(character: &model::Character, _game: &model::Game, builder: &mut WorkbookBuilder)
        -> anyhow::Result<()> {
    builder.cell(ProfileHeading::Name, 0, character.name())?;
    Ok(())
}

fn write_character_moves(character: &model::Character, game: &model::Game, builder: &mut WorkbookBuilder)
        -> anyhow::Result<()> {
    let mut i = 0;
    for char_move in character.moves() {
        builder
            .cell(MovesHeading::Name, i, char_move.name())?
            .cell(MovesHeading::Category, i, char_move.category(game).unwrap().name())?
            .cell(MovesHeading::Inputs, i, char_move.sequence().to_string())?
            .cell(MovesHeading::Notes, i, char_move.notes())?;
        i += 1;
    }

    Ok(())
}