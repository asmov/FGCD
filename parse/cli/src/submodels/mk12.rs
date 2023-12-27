use std::str::FromStr;

use fgcd_model::{self as model};
use anyhow;
use strum;
use strum::IntoEnumIterator;
use spreadsheet_ods as ods;
use fgcd_parse_macro::{SheetEnum, HeadingEnum};
use crate::spreadsheet::{self, HeadingTrait, SheetTrait, cell_string, cell_float, cell_u8};

use super::GameSubmodelSpreadsheetParserTrait;

#[derive(strum::EnumIter, SheetEnum)]
pub enum CharacterSheet {
    #[sheet(title("Move Frame Data (MK12)"), headings(CharacterMoveFrameDataHeading))]
    MoveFrameData
}


#[derive(strum::EnumIter, HeadingEnum)]
#[headings(sheet(CharacterSheet::MoveFrameData))]
pub enum CharacterMoveFrameDataHeading {
    #[heading(row(0), column(0))]
    Move,
    #[heading(row(0), column(1), title("Hit Damage"))]
    HitDamage,
    #[heading(row(0), column(2), title("Block Damage"))]
    BlockDamage,
    #[heading(row(0), column(3), title("Block Type"))]
    BlockType,
    #[heading(row(0), column(4), title("Flawless Block Damage"))]
    FlawlessBlockDamage,
    #[heading(row(0), column(5), title("Startup Frames"))]
    StartupFrames,
    #[heading(row(0), column(6), title("Hit Advantage Frames"))]
    HitAdvantageFrames,
    #[heading(row(0), column(7), title("Active Frames"))]
    ActiveFrames,
    #[heading(row(0), column(8), title("Flawless Block Advantage Frames"))]
    BlockAdvantageFrames,
    #[heading(row(0), column(9), title("Recovery Frames"))]
    RecoveryFrames,
    #[heading(row(0), column(10), title("Flawless Block Advantage Frames"))]
    FlawlessBlockAdvantageFrames,
    #[heading(row(0), column(11), title("Cancel Frames"))]
    CancelFrames
}
pub struct Parser {}

impl Parser {
    pub fn new() -> Self { Self{} }
}

impl GameSubmodelSpreadsheetParserTrait for Parser {
    fn read_game_submodel_spreadsheet(&self, _game: &model::Game, _workbook: &ods::WorkBook)
            -> anyhow::Result<model::GameSubmodel> {
        Ok(model::submodels::GameSubmodel::MortalKombat1(model::mk12::GameSubmodel::new()))
    }

    fn read_character_submodel_spreadsheet(&self, game: &model::Game, workbook: &ods::WorkBook)
            -> anyhow::Result<model::CharacterSubmodel> {
        let move_frame_data = read_character_move_frame_data(game, workbook)?;

        Ok(model::CharacterSubmodel::MortalKombat1(
            model::mk12::CharacterSubmodel::new(
                move_frame_data
        )))
    }

    fn write_game_submodel_spreadsheet(&self, _game: &model::Game, _builder: &mut spreadsheet::WorkbookBuilder)
            -> anyhow::Result<()> {
        Ok(())
    }

    fn new_game_submodel_spreadsheet(&self, _game_name: &str, _builder: &mut spreadsheet::WorkbookBuilder)
            -> anyhow::Result<()> {
        Ok(())
    }

    fn new_character_submodel_spreadsheet(&self,
            _character_name: &str,
            _game: &model::Game,
            builder: &mut spreadsheet::WorkbookBuilder) -> anyhow::Result<()> {
        builder.append_sheets(CharacterSheet::iter());
        Ok(())
    }

    fn write_character_submodel_spreadsheet(&self,
            _character: &model::Character,
            _game: &model::Game,
            _builder: &mut spreadsheet::WorkbookBuilder) -> anyhow::Result<()> {
        todo!() // mk12 frame data sheet
    }
}

fn read_character_move_frame_data(_game: &model::Game, workbook: &ods::WorkBook)
    -> anyhow::Result<Vec<model::mk12::MoveFrameData>>
{
    let sheet = spreadsheet::sheet(CharacterSheet::MoveFrameData.title(), &workbook)?;
    let mut moves_frame_data = Vec::new();

    for index in 0.. {
        let move_name = match cell_string(CharacterMoveFrameDataHeading::Move, index, sheet) {
            Ok(n)  => n,
            Err(_) => break
        };

        let hit_damage = cell_float(CharacterMoveFrameDataHeading::HitDamage, index, sheet)?;
        let block_damage = cell_float(CharacterMoveFrameDataHeading::BlockDamage, index, sheet)?;
        let block_type_str = cell_string(CharacterMoveFrameDataHeading::BlockType, index, sheet)?;
        let block_type = model::mk12::BlockType::from_str(&block_type_str)?;
        let flawless_block_damage = cell_float(CharacterMoveFrameDataHeading::FlawlessBlockDamage, index, sheet)?;
        let startup_frames = cell_u8(CharacterMoveFrameDataHeading::StartupFrames, index, sheet)?;
        let hit_advantage_frames = cell_u8(CharacterMoveFrameDataHeading::HitAdvantageFrames, index, sheet)?;
        let active_frames = cell_u8(CharacterMoveFrameDataHeading::ActiveFrames, index, sheet)?;
        let block_advantage_frames = cell_u8(CharacterMoveFrameDataHeading::BlockAdvantageFrames, index, sheet)?;
        let recovery_frames = cell_u8(CharacterMoveFrameDataHeading::RecoveryFrames, index, sheet)?;
        let flawless_block_advantage_frames = cell_u8( CharacterMoveFrameDataHeading::FlawlessBlockAdvantageFrames, index, sheet)?;
        let cancel_frames = cell_u8(CharacterMoveFrameDataHeading::CancelFrames, index, sheet)?;

        moves_frame_data.push(model::mk12::MoveFrameData::new(
            move_name,
            hit_damage as f32,
            block_damage as f32,
            block_type,
            flawless_block_damage as f32,
            startup_frames,
            hit_advantage_frames,
            active_frames,
            block_advantage_frames,
            recovery_frames,
            flawless_block_advantage_frames,
            cancel_frames ))
    }

    Ok(moves_frame_data)
}

#[cfg(test)]
mod tests {

}