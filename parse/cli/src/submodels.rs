use fgcd_model::{self as model};
use spreadsheet_ods as ods;

use crate::spreadsheet;

pub mod mk12;


pub trait GameSubmodelSpreadsheetParserTrait {
    fn new_game_submodel_spreadsheet(&self, game_name: &str, builder: &mut spreadsheet::WorkbookBuilder)
            -> anyhow::Result<()>;
    fn new_character_submodel_spreadsheet(&self,
            character_name: &str,
            game: &model::Game,
            builder: &mut spreadsheet::WorkbookBuilder) -> anyhow::Result<()>;
    fn read_game_submodel_spreadsheet(&self, game: &model::Game, workbook: &ods::WorkBook)
            -> anyhow::Result<model::GameSubmodel>;
    fn write_game_submodel_spreadsheet(&self, game: &model::Game, builder: &mut spreadsheet::WorkbookBuilder)
            -> anyhow::Result<()>;
    fn read_character_submodel_spreadsheet(&self, game: &model::Game, workbook: &ods::WorkBook)
            -> anyhow::Result<model::CharacterSubmodel>;
    fn write_character_submodel_spreadsheet(&self,
            character: &model::Character,
            game: &model::Game,
            builder: &mut spreadsheet::WorkbookBuilder) -> anyhow::Result<()>;
}

pub enum SubmodelParser {
    MortalKombat1,
}

impl GameSubmodelSpreadsheetParserTrait for SubmodelParser {
    fn new_game_submodel_spreadsheet(&self, game_name: &str, builder: &mut spreadsheet::WorkbookBuilder)
        -> anyhow::Result<()>
    {
        match self {
            Self::MortalKombat1 => mk12::Parser::new().new_game_submodel_spreadsheet(game_name, builder)
        }
    }

    fn new_character_submodel_spreadsheet(&self,
            character_name: &str,
            game: &model::Game,
            builder: &mut spreadsheet::WorkbookBuilder) -> anyhow::Result<()> {
        match self {
            Self::MortalKombat1 => mk12::Parser::new()
               .new_character_submodel_spreadsheet(character_name, game, builder),
        }
    }

    fn read_game_submodel_spreadsheet(&self, game: &model::Game, workbook: &ods::WorkBook) -> anyhow::Result<model::GameSubmodel> {
        match self {
            Self::MortalKombat1 => mk12::Parser::new().read_game_submodel_spreadsheet(game, workbook)
        }
    }

    fn write_game_submodel_spreadsheet(&self, game: &model::Game, builder: &mut spreadsheet::WorkbookBuilder)
        -> anyhow::Result<()>
    {
        match self {
            Self::MortalKombat1 => mk12::Parser::new().write_game_submodel_spreadsheet(game, builder)
        }
    }

    fn read_character_submodel_spreadsheet(&self, game: &model::Game, workbook: &ods::WorkBook)
        -> anyhow::Result<model::CharacterSubmodel> {
        match self {
            Self::MortalKombat1 => mk12::Parser::new().read_character_submodel_spreadsheet(game, workbook)
        }
    }

    fn write_character_submodel_spreadsheet(&self,
            character: &model::Character,
            game: &model::Game,
            builder: &mut spreadsheet::WorkbookBuilder) -> anyhow::Result<()> {
        match self {
            Self::MortalKombat1 => mk12::Parser::new().write_character_submodel_spreadsheet(character, game, builder)
        }
    }
}

impl From<model::Title> for SubmodelParser {
    fn from(title: model::Title) -> Self {
        match title {
            model::Title::MortalKombat1 => Self::MortalKombat1 
        }
    }
}