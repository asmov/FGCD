pub mod game;
pub mod platform;
pub mod input;
pub mod submodels;

pub use game::{Game, Character};
pub use platform::Platform;
pub use input::{Input, Symbol};
pub use submodels::{Title, GameSubmodel, CharacterSubmodel};
pub use submodels::mk12 as mk12;

pub const GAMES: &'static str = "games";
pub const CHARACTERS: &'static str = "characters";
pub const PLATFORMS: &'static str = "platforms";
pub const DEVICES: &'static str = "devices";

pub enum Models {
    Game,
    Character
}

impl Models {
    pub const fn name(&self) -> &'static str {
        match *self {
            Models::Game => "Game",
            Models::Character => "Character"
        }
    }
}

