use crate::CharacterSubmodel;
use crate::input::ContextTrait;
use crate::submodels::GameSubmodel;

use super::input;
use super::submodels;
use serde;
use chrono;

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Game {
    profile: Profile,
    character_names: Vec<String>,
    inputs: Vec<input::Input>,
    wildcard_inputs: Vec<input::Wildcard>,
    move_contexts: Vec<input::Context>,
    symbolic_sequences: Vec<input::SymbolicSequence>,
    input_categories: Vec<input::Category>,
    universal_moves: Vec<input::NamedSequence>,
    submodel: submodels::GameSubmodel
}

pub struct StagedBuilder {
    has_symbolic_sequences: bool,
    has_universal_moves: bool,
    has_specifics: bool,
    game: Game
}

// Separates the construction process into phases for use when parsing user input
// Input sequence parsing relies on the Game lookup_symbol() function, which uses
// constructed data. Each field cannot self-reference during parsing because of this.
impl StagedBuilder {
    pub fn new(
        profile: Profile,
        character_names: Vec<String>,
        inputs: Vec<input::Input>,
        wildcard_inputs: Vec<input::Wildcard>,
        move_contexts: Vec<input::Context>,
        input_categories: Vec<input::Category>) -> Self
    {
        Self {
            has_symbolic_sequences: false,
            has_universal_moves: false,
            has_specifics: false,
            game: Game {
                profile,
                character_names,
                inputs,
                wildcard_inputs,
                move_contexts,
                input_categories,
                symbolic_sequences: Vec::new(),
                universal_moves: Vec::new(),
                submodel: submodels::GameSubmodel::Generic
            }
        }
    }

    pub fn current(&self) -> &Game {
        &self.game
    }

    // Must be called first
    pub fn symbolic_sequences(&mut self, symbolic_sequences: Vec<input::SymbolicSequence>) -> &mut Self {
        self.game.symbolic_sequences = symbolic_sequences;
        self.has_symbolic_sequences = true;
        self
    }

    // Must be called after symbolic sequences
    pub fn universal_moves(&mut self, universal_moves: Vec<input::NamedSequence>) -> &mut Self {
        if !self.has_symbolic_sequences {
            panic!("fgcd::model::input::Game::Builder: Cannot set universal_moves before symbolic sequences");
        }

        self.game.universal_moves = universal_moves;
        self.has_universal_moves = true;
        self
    }

    pub fn submodel(&mut self, submodel: GameSubmodel) -> &mut Self {
        if !self.has_universal_moves {
            panic!("Cannot set specifics before universal moves");
        }

        self.game.submodel = submodel;
        self.has_specifics = true;
        self
    }

    pub fn build(self) -> Game {
        if !self.has_specifics {
            panic!("Cannot build without specifics")
        }

        self.game
    }
}

impl Game {
    pub fn new(
        profile: Profile,
        character_names: Vec<String>,
        inputs: Vec<input::Input>,
        wildcard_inputs: Vec<input::Wildcard>,
        move_contexts: Vec<input::Context>,
        input_categories: Vec<input::Category>,
        symbolic_sequences: Vec<input::SymbolicSequence>,
        universal_moves: Vec<input::NamedSequence>,
        submodel: submodels::GameSubmodel) -> Self
    {
        Self {
            profile,
            character_names,
            inputs,
            wildcard_inputs,
            move_contexts,
            input_categories,
            symbolic_sequences,
            universal_moves,
            submodel
        }
    }

    pub fn name(&self) -> &str {
        &self.profile.name
    }

    pub fn profile(&self) -> &Profile {
        &self.profile
    }

    pub fn character_names(&self) -> &[String] {
        &self.character_names
    }

    pub fn inputs(&self) -> &[input::Input] {
        &self.inputs
    }

    pub fn wildcard_inputs(&self) -> &[input::Wildcard] {
        &self.wildcard_inputs
    }

    pub fn  move_contexts(&self) -> &[input::Context] {
        &self.move_contexts
    }

    pub fn symbolic_sequences(&self) -> &[input::SymbolicSequence] {
        &self.symbolic_sequences
    }

    pub fn input_categories(&self) -> &[input::Category] {
        &self.input_categories
    }

    pub fn universal_moves(&self) -> &[input::NamedSequence] {
        &self.universal_moves
    }

    pub fn input_category(&self, category_ordinal: u8) -> Option<&input::Category> {
        self.input_categories.iter().find(|c| c.ordinal == category_ordinal)
    }

    pub fn find_input(&self, symbol: &str) -> Option<&input::Input> {
        self.inputs.iter().find(|i| symbol == i.symbol)
    }

    pub fn find_wildcard_input(&self, symbol: &str) -> Option<&input::Wildcard> {
        self.wildcard_inputs.iter().find(|i| symbol == i.symbol)
    }

    pub fn find_move_context(&self, context_token: &str) -> Option<&input::Context> {
        self.move_contexts.iter().find(|i| i.token() == context_token)
    }

    pub fn find_symbolic_sequence(&self, symbol: &str) -> Option<&input::SymbolicSequence> {
        self.symbolic_sequences.iter().find(|i| symbol == i.symbol)
    }

    pub fn find_input_category(&self, name: &str) -> Option<&input::Category> {
        self.input_categories.iter().find(|c| name == c.name)
    }
}

impl input::SymbolicLookup for Game {
    fn lookup_symbol<'g>(&'g self, symbol: &str) -> Option<input::LookupResult<'g>> {
        if let Some(input) = self.find_input(symbol) {
            return Some(input::LookupResult::Input(input));
        } else if let Some(wildcard) = self.find_wildcard_input(symbol) {
            return Some(input::LookupResult::Wildcard(wildcard));
        } else if let Some(symbol_sequence) = self.find_symbolic_sequence(symbol) {
            return Some(input::LookupResult::SymbolicSequence(symbol_sequence));
        }

        None
    }
}

impl input::ContextLookup for Game {
    fn lookup_custom_context<'owner>(&'owner self, context_token: &str) -> Option<&'owner input::Context> {
        self.find_move_context(context_token)
    }
}

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Profile {
    name: String,
    developer: String,
    publisher: String,
    release_date: chrono::NaiveDate,
    website_url: String,
    wikipedia_page_url: String,
    platform_names: Vec<String>
}

impl Profile {
    pub fn new(
        name: String,
        developer: String,
        publisher: String,
        release_date: chrono::NaiveDate,
        website_url: String,
        wikipedia_page_url: String,
        platform_names: Vec<String>) -> Self
    {
        Self {
            name,
            developer,
            publisher,
            release_date,
            website_url,
            wikipedia_page_url,
            platform_names
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
    
    pub fn developer(&self) -> &str {
        &self.developer
    }
    
    pub fn publisher(&self) -> &str {
        &self.publisher
    }

    pub fn release_date(&self) -> &chrono::NaiveDate{
        &self.release_date
    }

    pub fn website_url(&self) -> &str {
        &self.website_url
    }

    pub fn wikipedia_page_url(&self) -> &str {
        &self.wikipedia_page_url
    }

    pub fn platform_names(&self) -> &Vec<String> {
        &self.platform_names
    }
}

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Character {
    name: String,
    moves: Vec<input::NamedSequence>,
    submodel: CharacterSubmodel
}

impl Character {
    pub fn new(name: String, moves: Vec<input::NamedSequence>, submodel: CharacterSubmodel) -> Self {
        Self { name, moves, submodel }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn moves(&self) -> &[input::NamedSequence] {
        &self.moves
    }

    pub fn submodel(&self) -> &CharacterSubmodel {
        &self.submodel
    }
}

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InputBinding {
    name: String,
    input: input::Input
}

impl InputBinding {
    pub fn new(name: String, input: input::Input) -> Self {
        Self { name, input }
    }
}

pub mod test {
    use crate::{self as model, game, input, Game, Input, Symbol, GameSubmodel};
    use chrono::NaiveDate;

    pub fn game_fixture() -> Game {
        Game::new(
            game::Profile::new(
                String::from("Ternary Op 3"),
                String::from("Asmov LLC"),
                String::from("Asmov"),
                NaiveDate::from_ymd_opt(2023, 10, 4).unwrap(),
                String::from("https://asmov.software"),
                String::from("https://en.wikipedia.org/wiki/Ternary_conditional_operator"),
                vec![String::from("Linux")]
            ),
            vec![String::from("If"), String::from("Then"), String::from("Else")],
            vec![
                Input::new(String::from("Up"), String::from("U")),
                Input::new(String::from("Up-Forward"), String::from("UF")),
                Input::new(String::from("Forward"), String::from("F")),
                Input::new(String::from("Down-Forward"), String::from("DF")),
                Input::new(String::from("Down"), String::from("D")),
                Input::new(String::from("Down-Back"), String::from("DB")),
                Input::new(String::from("Back"), String::from("B")),
                Input::new(String::from("Up-Back"), String::from("UB")),
                Input::new(String::from("Block"), String::from("BLK")),
                Input::new(String::from("Alternate"), String::from("ALT")),
                Input::new(String::from("Special 1"), String::from("SP1")),
                Input::new(String::from("Special 2"), String::from("SP2")),
                Input::new(String::from("Punch 1"), String::from("P1")),
                Input::new(String::from("Punch 2"), String::from("P2")),
                Input::new(String::from("Punch 3"), String::from("P3")),
                Input::new(String::from("Kick 1"), String::from("K1")),
                Input::new(String::from("Kick 2"), String::from("K2")),
                Input::new(String::from("Kick 3"), String::from("K3"))
            ],
            vec![
                input::Wildcard::new(String::from("Any Punch"), String::from("P"),
                    vec![Symbol::from("P1"), Symbol::from("P2"), Symbol::from("P3")]),
                input::Wildcard::new(String::from("Any Kick"), String::from("K"),
                    vec![Symbol::from("P1"), Symbol::from("P2"), Symbol::from("P3")]),
                input::Wildcard::new(String::from("Any Two Punches"), String::from("2P"),
                    vec![Symbol::from("P1"), Symbol::from("P2"), Symbol::from("P3")]),
                input::Wildcard::new(String::from("Any Two Kicks"), String::from("2K"),
                    vec![Symbol::from("P1"), Symbol::from("P2"), Symbol::from("P3")]),
                input::Wildcard::new(String::from("Any Three Punches"), String::from("3P"),
                    vec![Symbol::from("P1"), Symbol::from("P2"), Symbol::from("P3")]),
                input::Wildcard::new(String::from("Any Three Kicks"), String::from("3K"),
                    vec![Symbol::from("P1"), Symbol::from("P2"), Symbol::from("P3")]),
            ],
            vec![
                input::Context::Custom(String::from("exceptional"))
            ],
            vec![
                input::Category::new(0, String::from("Normal Move")),
                input::Category::new(1, String::from("Special Move")),
            ],
            vec![
                input::SymbolicSequence::new(String::from("Quarter Circle Forward"), Symbol::from("QCF"),
                    input::Sequence::new(vec![
                        input::InputEntry::for_symbol("D"),
                        input::InputEntry::for_symbol("DF"),
                        input::InputEntry::for_symbol("F") ] )),
                input::SymbolicSequence::new(String::from("Quarter Circle Back"), Symbol::from("QCB"),
                    input::Sequence::new(vec![
                        input::InputEntry::for_symbol("D"),
                        input::InputEntry::for_symbol("DB"),
                        input::InputEntry::for_symbol("B") ] )),
                input::SymbolicSequence::new(String::from("Z-Motion Forward"), Symbol::from("ZF"),
                    input::Sequence::new(vec![
                        input::InputEntry::for_symbol("F"),
                        input::InputEntry::for_symbol("D"),
                        input::InputEntry::for_symbol("DF"),
                        input::InputEntry::for_symbol("F") ] )),
                input::SymbolicSequence::new(String::from("Z-Motion Back"), Symbol::from("ZB"),
                    input::Sequence::new(vec![
                        input::InputEntry::for_symbol("B"),
                        input::InputEntry::for_symbol("D"),
                        input::InputEntry::for_symbol("DB"),
                        input::InputEntry::for_symbol("B") ] )),
            ],
            vec![
                input::NamedSequence::new_raw(
                    String::from("Combo Breaker"),
                    input::Sequence::new(vec![
                        input::CombinationEntry::for_symbols(vec!["F", "BLK"])]),
                    0,
                    Some(String::from("Perform during an opponent's combo")) )
            ],
            GameSubmodel::Generic
        )
    }

    pub fn character_fixture() -> model::Character {
        model::Character::new(
            String::from("Question Mark"),
            vec![
                input::NamedSequence::new_raw(
                    String::from("Exclaim"),
                    input::Sequence::new(vec![
                        input::CombinationEntry::for_symbols(vec![ "QCF", "P1"])]),
                    0,
                    Some(String::from("It is excited")) )

            ],
            model::CharacterSubmodel::Generic
        )
    }
}
