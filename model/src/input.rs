use std::fmt::Display;

use serde;
use anyhow;
use crate::Game;

pub type Symbol = String;

mod parse;

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Input {
    name: String,
    pub(crate) symbol: Symbol
}

impl Input {
    pub fn new(name: String, symbol: Symbol) -> Self {
        Self { name, symbol }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn symbol(&self) -> &Symbol {
        &self.symbol
    }
}

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Wildcard {
    name: String,
    pub(crate) symbol: Symbol,
    pub(crate) matches: Vec<Symbol>, 
}

impl Wildcard {
    pub fn new(name: String, symbol: Symbol, matches: Vec<Symbol>) -> Self {
        Self { name, symbol, matches }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn symbol(&self) -> &Symbol {
        &self.symbol
    }

    pub fn matches(&self) -> &[Symbol] {
        &self.matches
    }
}
pub trait SymbolicLookup {
    fn lookup_symbol<'owner>(&'owner self, symbol: &str) -> Option<LookupResult<'owner>>;
}

pub trait ContextLookup {
    fn lookup_custom_context<'owner>(&'owner self, context: &str) -> Option<&'owner Context>;
}

#[derive(Debug, PartialEq)]
pub enum LookupResult<'owner> {
    Input(&'owner Input),
    Wildcard(&'owner Wildcard),
    SymbolicSequence(&'owner SymbolicSequence)
}

impl<'owner> LookupResult<'owner> {
    pub fn symbol(&self) -> &Symbol {
        match *self {
            LookupResult::Input(l) => &l.symbol,
            LookupResult::Wildcard(l) => &l.symbol,
            LookupResult::SymbolicSequence(l) => &l.symbol,
        }
    }
}

// The TokenPosition for each Technique is OPENING by default (left side of the entry).
// The one exception to this is a GroupEntry that is the first CombinationItem of a CombinationEntry. In this case,
// <technique> must be provided at the CLOSING position (right side) to prevent disambiguity between the
// CombinationEntry's <technique> and the GroupEntry's. 
#[derive(Copy, Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, strum::Display, strum::EnumString)]
pub enum Technique {
    // Hold the input(s) for a short duration.
    #[strum(serialize = "charge")]
    Charge,
    // Press the input(s) repeatedly. TokenPosition::CLOSING
    #[strum(serialize = "tap")]
    Tap,
    // Press the input(s) in quick succession.
    #[strum(serialize = "quick")]
    Quick
}

pub trait ContextTrait {
    fn token(&self) -> &str;
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Context {
    Air,
    Ground,
    Juggle,
    Close,
    Sweep,
    Mid,
    Far,
    Custom(String)
}

impl Display for Context {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.token())
    }
}

impl TryFrom<&str> for Context {
    type Error = anyhow::Error;

    fn try_from(token: &str) -> anyhow::Result<Self> {
        match token {
            "air" => Ok(Self::Air),
            "ground" => Ok(Self::Ground),
            "juggle" => Ok(Self::Juggle),
            "close" => Ok(Self::Close),
            "sweep" => Ok(Self::Sweep),
            "mid" => Ok(Self::Mid),
            "far" => Ok(Self::Far),
            _ => anyhow::bail!("Unknown context: {token}")
        }
    }
}


impl ContextTrait for Context {
    fn token(&self) -> &str {
        match self {
            Self::Air => "air",
            Self::Ground => "ground",
            Self::Juggle => "juggle",
            Self::Close => "close",
            Self::Sweep => "sweep",
            Self::Mid => "sweep",
            Self::Far => "far",
            Self::Custom(custom) => &custom,
        }
    }
}

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InputEntry {
    symbol: Symbol,
    technique: Option<Technique>,
    opening_note: Option<String>,
    closing_note: Option<String>
}

impl InputEntry {
    pub fn new(symbol: Symbol, technique: Option<Technique>, opening_note: Option<String>, closing_note: Option<String>)
            -> Self {
        Self {
            symbol,
            technique,
            opening_note,
            closing_note
        }
    }

    pub fn for_symbol(symbol: &str) -> Entry{ 
        Entry::Input(Self {
            symbol: symbol.to_string(),
            technique: None,
            opening_note: None,
            closing_note: None,
        })
    }

    pub fn symbol(&self) -> &Symbol {
        &self.symbol
    }
}

impl EntryTrait for Entry {
    fn technique(&self) -> Option<Technique> {
        match self {
            Self::Input(input_entry) => input_entry.technique(),
            Self::Combination(combination_entry) => combination_entry.technique(),
            Self::Group(group_entry) => group_entry.technique(),
            Self::Note(_) => None,
            Self::Context(_) => None,
        }
    }

    fn opening_note(&self) -> Option<&str> {
        match self {
            Self::Input(input_entry) => input_entry.opening_note(),
            Self::Combination(combination_entry) => combination_entry.opening_note(),
            Self::Group(group_entry) => group_entry.opening_note(),
            Self::Note(_) => None,
            Self::Context(_) => None,
        }
    }

    fn closing_note(&self) -> Option<&str> {
        match self {
            Self::Input(input_entry) => input_entry.closing_note(),
            Self::Combination(combination_entry) => combination_entry.closing_note(),
            Self::Group(group_entry) => group_entry.closing_note(),
            Self::Note(_) => None,
            Self::Context(_) => None,
        }
    }
}

impl EntryTrait for CombinationEntry {
    fn technique(&self) -> Option<Technique> {
        self.technique
    }

    fn opening_note(&self) -> Option<&str> {
        self.opening_note.as_deref()
    }

    fn closing_note(&self) -> Option<&str> {
        self.closing_note.as_deref()
    }
}

impl EntryTrait for GroupEntry {
    fn technique(&self) -> Option<Technique> {
        self.technique
    }

    fn opening_note(&self) -> Option<&str> {
        self.opening_note.as_deref()
    }

    fn closing_note(&self) -> Option<&str> {
        self.closing_note.as_deref()
    }
}

impl EntryTrait for InputEntry {
    fn technique(&self) -> Option<Technique> {
        self.technique
    }

    fn opening_note(&self) -> Option<&str> {
        self.opening_note.as_deref()
    }

    fn closing_note(&self) -> Option<&str> {
        self.closing_note.as_deref()
    }
}

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum CombinationItem {
    Symbol(Symbol),
    Group(GroupEntry)
}

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CombinationEntry {
    items: Vec<CombinationItem>,
    technique: Option<Technique>,
    opening_note: Option<String>,
    closing_note: Option<String>
}

impl CombinationEntry {
    pub fn new(items: Vec<CombinationItem>, technique: Option<Technique>, opening_note: Option<String>, closing_note: Option<String>)
            -> Self {
        Self {
            items,
            technique,
            opening_note,
            closing_note
        }
    }

    pub fn for_symbols(symbols: Vec<&str>) -> Entry {
        Entry::Combination(Self {
            items: symbols.iter()
                .map(|s| CombinationItem::Symbol(s.to_string()))
                .collect(),
            technique: None,
            opening_note: None,
            closing_note: None
        })
    }

    pub fn items(&self) -> &[CombinationItem] {
        &self.items
    }
}

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GroupEntry {
    entries: Vec<Entry>,
    technique: Option<Technique>,
    opening_note: Option<String>,
    closing_note: Option<String>
}

impl GroupEntry {
    pub fn new(entries: Vec<Entry>, technique: Option<Technique>, opening_note: Option<String>, closing_note: Option<String>)
            -> Self {
        Self {
            entries,
            technique,
            opening_note,
            closing_note
        }
    }

    pub fn entries(&self) -> &[Entry] {
        &self.entries
    }
}

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Entry {
    // A singly input: A, B, C, D
    Input(InputEntry),
    // A combination of inputs: A + B, C + D, E + F + G
    Combination(CombinationEntry),
    // A subgroup of entries: A, [B, C, D](tap), E
    Group(GroupEntry),
    // A standalone note: A, (something interesting), B
    Note(String),
    // Context for the next entry; close, far, juggle, etc. Can be game-specific. 
    Context(Vec<Context>)
}

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct NamedSequence {
    name: String,
    sequence: Sequence,
    category_ordinal: u8,
    notes: Option<String>

}

impl NamedSequence {
    pub fn new_raw(name: String, sequence: Sequence, category_ordinal: u8, notes: Option<String>) -> Self {
        Self {
            name,
            sequence,
            category_ordinal,
            notes
        }
    }

    pub fn new(name: String, sequence: Sequence, category: &Category, notes: Option<String>) -> Self {
        Self {
            name,
            sequence,
            category_ordinal: category.ordinal,
            notes
        }
    }

    pub fn category<'game>(&self, game: &'game Game) -> Option<&'game Category> {
            game.input_category(self.category_ordinal)
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn sequence(&self) -> &Sequence{
        &self.sequence
    }

    pub fn notes(&self) -> Option<&str> {
        self.notes.as_deref()
    }

}

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub struct Category {
    pub(crate) ordinal: u8,
    pub(crate) name: String,
}

impl Category {
    pub fn new(ordinal: u8, name: String) -> Self {
        Self { ordinal, name }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn ordinal(&self) -> u8 {
        self.ordinal
    }
}

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SymbolicSequence {
    name: String,
    pub(crate) symbol: Symbol,
    sequence: Sequence 
}

impl SymbolicSequence {
    pub fn new(name: String, symbol: Symbol, sequence: Sequence) -> Self {
        Self { name, symbol, sequence }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn symbol(&self) -> &Symbol {
        &self.symbol
    }

    pub fn sequence(&self) -> &Sequence {
        &self.sequence
    }
}

pub trait EntryTrait {
    fn opening_note(&self) -> Option<&str>;
    fn closing_note(&self) -> Option<&str>;
    fn technique(&self) -> Option<Technique>;
}
 

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Sequence {
    entries: Vec<Entry>
}

impl Sequence {
    pub fn new(entries: Vec<Entry>) -> Self {
        Self { entries }
    }

    pub fn parse(input_string: &str, symbol_lookup: &impl SymbolicLookup, context_lookup: &impl ContextLookup) -> anyhow::Result<Self> {
        let entries: Vec<Entry> = parse::parse_entries(input_string, symbol_lookup, context_lookup, 0, parse::TokenPosition::default())?;
        Ok(Sequence::new(entries))
    }
}

impl Display for Sequence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match parse::entries_to_string(&self.entries, 0) {
            Ok(result) => write!(f, "{result}"),
            Err(_e) => Err(std::fmt::Error) 
        }
    }
}
