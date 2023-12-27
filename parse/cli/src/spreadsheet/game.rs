use std::{fs, path::{PathBuf,Path}, str::FromStr};
use fgcd_model as model;
use anyhow;
use model::{Title, input::ContextTrait};
use strum::IntoEnumIterator;
use spreadsheet_ods as ods;
use crate::{spreadsheet::*, submodels::{SubmodelParser, GameSubmodelSpreadsheetParserTrait}};
use fgcd_parse_macro::{SheetEnum, HeadingEnum};

pub mod character;

#[derive(SheetEnum, strum::EnumIter)]
enum Sheets {
    #[sheet(horizontal)]
    Profile, 
    Characters,
    Inputs,
    #[sheet(title("Wildcard Inputs"))]
    WildcardInputs,
    #[sheet(title("Input Sequences"))]
    InputSequences,
    #[sheet(title("Move Contexts"))]
    MoveContexts,
    #[sheet(title("Move Categories"))]
    MoveCategories,
    #[sheet(title("Universal Moves"))]
    UniversalMoves
}

#[derive(HeadingEnum, strum::EnumIter)]
#[headings(sheet(Sheets::Profile))]
enum ProfileHeading {
    #[heading(column(0), row(0))]
    Name,
    #[heading(column(0), row(1))]
    Developer,
    #[heading(column(0), row(2))]
    Publisher,
    #[heading(column(0), row(3), title("Release Date"))]
    ReleaseDate,
    #[heading(column(0), row(4))]
    Website,
    #[heading(column(0), row(5))]
    Wikipedia,
    #[heading(column(0), row(6))]
    Platforms
}

#[derive(HeadingEnum, strum::EnumIter)]
#[headings(sheet(Sheets::Characters))]
enum CharactersHeading {
    #[heading(row(0), column(0))]
    Name
}

#[derive(HeadingEnum, strum::EnumIter)]
#[headings(sheet(Sheets::Inputs))]
enum InputsHeading {
    #[heading(row(0), column(0))]
    Name,
    #[heading(row(0), column(1))]
    Symbol
}

#[derive(HeadingEnum, strum::EnumIter)]
#[headings(sheet(Sheets::WildcardInputs))]
enum WildcardInputsHeading {
    #[heading(row(0), column(0))]
    Name,
    #[heading(row(0), column(1))]
    Symbol,
    #[heading(row(0), column(2))]
    Matches 
}

#[derive(HeadingEnum, strum::EnumIter)]
#[headings(sheet(Sheets::MoveContexts))]
enum MoveContextsHeading {
    #[heading(row(0), column(0))]
    Token
}


#[derive(HeadingEnum, strum::EnumIter)]
#[headings(sheet(Sheets::MoveCategories))]
enum MoveCategoriesHeading {
    #[heading(row(0), column(0))]
    Name
}


#[derive(HeadingEnum, strum::EnumIter)]
#[headings(sheet(Sheets::InputSequences))]
enum InputSequencesHeading {
    #[heading(row(0), column(0))]
    Name,
    #[heading(row(0), column(1))]
    Symbol,
    #[heading(row(0), column(2))]
    Sequence 
}

#[derive(HeadingEnum, strum::EnumIter)]
#[headings(sheet(Sheets::UniversalMoves))]
enum UniversalMovesHeading {
    #[heading(row(0), column(0))]
    Name,
    #[heading(row(0), column(1))]
    Category,
    #[heading(row(0), column(2))]
    Inputs,
    #[heading(row(0), column(3))]
    Notes
}

/**
 * @param path Path to the FCCD data directory or the Game data file
 */
pub fn read_game(game_name: &str, path: &Path) -> anyhow::Result<model::Game> {
    let filepath = if path.is_file() {
        path.to_path_buf()
    } else {
        crate::game_filepath(game_name, &path, EXT_FODS)
    };

    let workbook = ods::read_fods(filepath)?;

    let profile = read_game_profile(&workbook)?;
    let character_names = read_game_character_names(&workbook)?;
    let inputs = read_game_inputs(&workbook)?;
    let wildcard_inputs = read_game_wildcard_inputs(&workbook)?;
    let move_contexts = read_game_move_contexts(&workbook)?;
    let move_categories = read_game_move_categories(&workbook)?;

    let mut game_builder = model::game::StagedBuilder::new(
        profile,
        character_names,
        inputs,
        wildcard_inputs,
        move_contexts,
        move_categories);

    let symbolic_sequences = read_game_symbolic_sequences(&game_builder.current(), &workbook)?;
    game_builder.symbolic_sequences(symbolic_sequences);

    let universal_moves = read_game_universal_moves(&game_builder.current(), &workbook)?;
    game_builder.universal_moves(universal_moves);

    let submodel = read_game_submodel(&game_builder.current(), &workbook)?;
    game_builder.submodel(submodel);


    Ok(game_builder.build())
}

fn read_game_profile(workbook: &ods::WorkBook) -> anyhow::Result<model::game::Profile> {
    let sheet = sheet(Sheets::Profile.title(), &workbook)?;
        
    let name = cell_string(ProfileHeading::Name, 0, sheet)?;
    let developer = cell_string(ProfileHeading::Developer, 0, sheet)?;
    let publisher = cell_string(ProfileHeading::Publisher, 0, sheet)?;
    let release_date = cell_date(ProfileHeading::ReleaseDate, 0, sheet)?;
    let website_url = cell_string(ProfileHeading::Website, 0, sheet)?;
    let wikipedia_page_url = cell_string(ProfileHeading::Wikipedia, 0, sheet)?;
    let platforms = cell_csv(ProfileHeading::Platforms, 0, sheet)?;

    Ok(model::game::Profile::new(
        name,
        developer,
        publisher,
        release_date,
        website_url,
        wikipedia_page_url,
        platforms
    ))
}

fn read_game_character_names(workbook: &ods::WorkBook) -> anyhow::Result<Vec<String>> {
    let sheet = sheet(Sheets::Characters.title(), &workbook)?;
    let mut character_names: Vec<String> = Vec::new();

    for index in 0.. {
        let name = match cell_string(CharactersHeading::Name, index, sheet) {
            Ok(n)  => n,
            Err(_) => break
        };

        character_names.push(name);
    }

    Ok(character_names)
}

fn read_game_inputs(workbook: &ods::WorkBook) -> anyhow::Result<Vec<model::Input>> {
    let sheet = sheet(Sheets::Inputs.title(), &workbook)?;
    let mut inputs: Vec<model::input::Input> = Vec::new();

    for index in 0.. {
        let name = match cell_string(InputsHeading::Name, index, sheet) {
            Ok(n)  => n,
            Err(_) => break
        };

        let symbol = cell_string(InputsHeading::Symbol, index, sheet)?;
        inputs.push(model::Input::new(name, symbol));
    }

    Ok(inputs)
}

fn read_game_wildcard_inputs(workbook: &ods::WorkBook) -> anyhow::Result<Vec<model::input::Wildcard>> {
    let sheet = sheet(Sheets::WildcardInputs.title(), &workbook)?;
    let mut wildcard_inputs: Vec<model::input::Wildcard> = Vec::new();

    for index in 0.. {
        let name = match cell_string(WildcardInputsHeading::Name, index, sheet) {
            Ok(n)  => n,
            Err(_) => break
        };

        let symbol = cell_string(WildcardInputsHeading::Symbol, index, sheet)?;
        let possibilities: Vec<model::Symbol> = cell_csv(WildcardInputsHeading::Matches, index, sheet)?
            .iter()
            .map(|s| model::Symbol::from(s))
            .collect();

        wildcard_inputs.push(model::input::Wildcard::new(name, symbol, possibilities));
    }

    Ok(wildcard_inputs)
}

fn read_game_move_contexts(workbook: &ods::WorkBook) -> anyhow::Result<Vec<model::input::Context>> {
    let sheet = sheet(Sheets::MoveContexts.title(), &workbook)?;
    let mut move_contexts: Vec<model::input::Context> = Vec::new();

    for index in 0.. {
        let token = match cell_string(MoveContextsHeading::Token, index, sheet) {
            Ok(n)  => n,
            Err(_) => break
        };

        move_contexts.push(model::input::Context::Custom(token));
    }

    Ok(move_contexts)
}


fn read_game_move_categories(workbook: &ods::WorkBook) -> anyhow::Result<Vec<model::input::Category>> {
    let sheet = sheet(Sheets::MoveCategories.title(), &workbook)?;
    let mut move_categories: Vec<model::input::Category> = Vec::new();

    for index in 0.. {
        let name = match cell_string(MoveCategoriesHeading::Name, index, sheet) {
            Ok(n)  => n,
            Err(_) => break
        };

        move_categories.push(model::input::Category::new(index as u8, name));
    }

    Ok(move_categories)
}

fn read_game_symbolic_sequences(game: &model::Game, workbook: &ods::WorkBook)
    -> anyhow::Result<Vec<model::input::SymbolicSequence>>
{
    let sheet = sheet(Sheets::InputSequences.title(), &workbook)?;
    let mut symbolic_sequences: Vec<model::input::SymbolicSequence> = Vec::new();

    for index in 0.. {
        let name = match cell_string(InputSequencesHeading::Name, index, sheet) {
            Ok(n)  => n,
            Err(_) => break
        };

        let symbol = cell_string(InputSequencesHeading::Symbol, index, sheet)?;
        let sequence_str = cell_string(InputSequencesHeading::Sequence, index, sheet)?;
        let sequence = model::input::Sequence::parse(&sequence_str, game, game)?;

        symbolic_sequences.push(model::input::SymbolicSequence::new(name, symbol, sequence));
    }

    Ok(symbolic_sequences)
}

fn read_game_universal_moves(game: &model::Game, workbook: &ods::WorkBook)
    -> anyhow::Result<Vec<model::input::NamedSequence>>
{
    let sheet = sheet(Sheets::UniversalMoves.title(), &workbook)?;
    let mut moves: Vec<model::input::NamedSequence> = Vec::new();

    for index in 0.. {
        let name = match cell_string(UniversalMovesHeading::Name, index, sheet) {
            Ok(n)  => n,
            Err(_) => break
        };

        let category_name = cell_string(UniversalMovesHeading::Category, index, sheet)?;
        let category = game.find_input_category(&category_name)
            .context(format!("Universal move category not found for game `{}`: {}", game.name(), category_name))?;
        let sequence_str = cell_string(UniversalMovesHeading::Inputs, index, sheet)?;
        let sequence = model::input::Sequence::parse(&sequence_str, game, game)?;
        let notes = cell_string_optional(UniversalMovesHeading::Notes, index, sheet)?;

        moves.push(model::input::NamedSequence::new(name, sequence, category, notes));
    }

    Ok(moves)
}

fn read_game_submodel(game: &model::Game, workbook: &ods::WorkBook)
    -> anyhow::Result<model::GameSubmodel>
{
    if let Ok(title) = model::Title::from_str(game.name()) {
        SubmodelParser::from(title)
            .read_game_submodel_spreadsheet(game, workbook)
    } else {
        Ok(model::GameSubmodel::Generic)
    }
}

pub fn new_game(game_name: &str, data_dir: &Path) -> anyhow::Result<(ods::WorkBook, PathBuf)> {
    let game_data_dir = crate::game_data_dir(game_name, data_dir);
    if !game_data_dir.exists() {
        fs::create_dir(&game_data_dir)?;
    }

    let characters_data_dir = game_data_dir.join(model::CHARACTERS);
    if !characters_data_dir.exists() {
        fs::create_dir(characters_data_dir)?;
    }

    let mut builder = WorkbookBuilder::new();
    builder.append_sheets(Sheets::iter());

    if let Ok(title) = model::Title::from_str(game_name) {
        SubmodelParser::from(title)
            .new_game_submodel_spreadsheet(game_name, &mut builder)?
    }   

    builder.write(model::Models::Game.name(), &game_data_dir)
}

pub fn write_game(game: &model::Game, data_dir: &Path)
    -> anyhow::Result<(ods::WorkBook, PathBuf)>
{
    let game_data_dir = crate::game_data_dir(game.name(), data_dir);
    let mut builder = WorkbookBuilder::new();
    builder.append_sheets(Sheets::iter());

    write_game_profile(game, &mut builder)?;
    write_game_character_names(game, &mut builder)?;
    write_game_inputs(game, &mut builder)?;
    write_game_wildcard_inputs(game, &mut builder)?;
    write_game_input_sequences(game, &mut builder)?;
    write_game_move_contexts(game, &mut builder)?;
    write_game_move_categories(game, &mut builder)?;
    write_game_universal_moves(game, &mut builder)?;

    if let Ok(title) = Title::from_str(game.name()) {
        let submodel_parser = SubmodelParser::from(title);
        submodel_parser.write_game_submodel_spreadsheet(game, &mut builder)?;
    }

    builder.write(model::Models::Game.name(), &game_data_dir)
}

pub fn write_game_profile<'b>(game: &model::Game, builder: &'b mut WorkbookBuilder) -> anyhow::Result<&'b mut WorkbookBuilder> {
    let profile = game.profile();
    builder
        .cell(ProfileHeading::Name, 0, profile.name())?
        .cell(ProfileHeading::Developer, 0, profile.developer())?
        .cell(ProfileHeading::Publisher, 0, profile.publisher())?
        .cell(ProfileHeading::ReleaseDate, 0, *profile.release_date())?
        .cell(ProfileHeading::Website, 0, profile.website_url())?
        .cell(ProfileHeading::Wikipedia, 0, profile.wikipedia_page_url())?
        .cell(ProfileHeading::Platforms, 0, profile.platform_names().join(", "))
}

pub fn write_game_character_names<'b>(game: &model::Game, builder: &'b mut WorkbookBuilder) -> anyhow::Result<&'b mut WorkbookBuilder> {
    let mut i = 0;
    for character_name in game.character_names() {
        builder.cell(CharactersHeading::Name, i, character_name)?;
        i += 1;
    }

    Ok(builder)
}

pub fn write_game_inputs<'b>(game: &model::Game, builder: &'b mut WorkbookBuilder) -> anyhow::Result<&'b mut WorkbookBuilder> {
    let mut i = 0;
    for input in game.inputs() {
        builder
            .cell(InputsHeading::Name, i, input.name())?
            .cell(InputsHeading::Symbol, i, input.symbol())?;

        i += 1;
    }

    Ok(builder)
}

pub fn write_game_wildcard_inputs<'b>(game: &model::Game, builder: &'b mut WorkbookBuilder) -> anyhow::Result<&'b mut WorkbookBuilder> {
    let mut i = 0;
    for wildcard_input in game.wildcard_inputs() {
        builder
            .cell(WildcardInputsHeading::Name, i, wildcard_input.name())?
            .cell(WildcardInputsHeading::Symbol, i, wildcard_input.symbol())?
            .cell(WildcardInputsHeading::Matches, i, wildcard_input.matches().join(", "))?;

        i += 1;
    }

    Ok(builder)
}

pub fn write_game_input_sequences<'b>(game: &model::Game, builder: &'b mut WorkbookBuilder) -> anyhow::Result<&'b mut WorkbookBuilder> {
    let mut i = 0;
    for sequence in game.symbolic_sequences() {
        builder
            .cell(InputSequencesHeading::Name, i, sequence.name())?
            .cell(InputSequencesHeading::Symbol, i, sequence.symbol())?
            .cell(InputSequencesHeading::Sequence, i, "")?
            .cell(InputSequencesHeading::Sequence, i, sequence.sequence().to_string())?;

        i += 1;
    }

    Ok(builder)
}

pub fn write_game_move_categories<'b>(game: &model::Game, builder: &'b mut WorkbookBuilder) -> anyhow::Result<&'b mut WorkbookBuilder> {
    let mut i = 0;
    for category in game.input_categories() {
        builder.cell(MoveCategoriesHeading::Name, i, category.name())?;
        i += 1;
    }

    Ok(builder)
}

pub fn write_game_move_contexts<'b>(game: &model::Game, builder: &'b mut WorkbookBuilder) -> anyhow::Result<&'b mut WorkbookBuilder> {
    let mut i = 0;
    for context in game.move_contexts() {
        builder.cell(MoveContextsHeading::Token, i, context.token())?;
        i += 1;
    }

    Ok(builder)
}


pub fn write_game_universal_moves<'b>(game: &model::Game, builder: &'b mut WorkbookBuilder) -> anyhow::Result<&'b mut WorkbookBuilder> {
    let mut i = 0;
    for named_sequence in game.universal_moves() {
        builder
            .cell(UniversalMovesHeading::Name, i, named_sequence.name())?
            .cell(UniversalMovesHeading::Category, i, named_sequence.category(game).unwrap().name())?
            .cell(UniversalMovesHeading::Inputs, i, named_sequence.sequence().to_string())?
            .cell(UniversalMovesHeading::Notes, i, named_sequence.notes())?;
        i += 1;
    }

    Ok(builder)
}

