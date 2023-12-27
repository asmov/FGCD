pub mod mk12;

#[derive(strum::EnumString)]
pub enum Title {
    #[strum(serialize = "Mortal Kombat 1", serialize = "mk12")]
    MortalKombat1,
}

#[derive(Default, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum GameSubmodel {
    #[default]
    Generic,
    MortalKombat1 (mk12::GameSubmodel)
}

#[derive(Default, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum CharacterSubmodel {
    #[default]
    Generic,
    MortalKombat1 (mk12::CharacterSubmodel)
}
