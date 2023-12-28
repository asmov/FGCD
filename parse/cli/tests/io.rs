
#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use fgcd_parse::{self, spreadsheet};

    const MORTAL_KOMBAT_1: &str = "Mortal Kombat 1";
    const JOHNNY_CAGE: &str = "Johnny Cage";

    fn fgcd_data_dir() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent().unwrap()
            .parent().unwrap()
            .join("target")
            .join("data")
    }

    #[test]
    fn sample_spreadsheets_to_binary() {
        let data_dir = fgcd_data_dir();

        let mkone_dir = fgcd_parse::game_data_dir(MORTAL_KOMBAT_1, &data_dir);

        assert!(mkone_dir.exists(),
            "Game dir should exist");

        let mkone_game_filepath = fgcd_parse::game_filepath(
            MORTAL_KOMBAT_1,
            &data_dir,
            fgcd_parse::spreadsheet::EXT_FODS);

        assert!(mkone_game_filepath.exists(),
            "Game FODS file should exist: {}", mkone_game_filepath.to_str().unwrap());

        let game = fgcd_parse::spreadsheet::game::read_game(
                MORTAL_KOMBAT_1,
                &data_dir)
            .unwrap();    

        println!("Game from ODF: {:#?}", game);

        // Write Game from ODS to binary
        fgcd_parse::binary::game::write_game(
                &game,
                &data_dir)
            .unwrap();

        let game_read = fgcd_parse::binary::game::read_game(
                MORTAL_KOMBAT_1,
                &data_dir)
            .unwrap();

        println!("Game from binary: {:#?}", game_read);

        assert_eq!(game, game_read,
            "Game read from spreadsheet should match binary");

        let character = fgcd_parse::spreadsheet::game::character::read_character(
                JOHNNY_CAGE,
                &game,
                &data_dir)
            .unwrap();

        println!("Character from ODF: {:#?}", character);

        fgcd_parse::binary::game::character::write_character(&character, &game, &data_dir).unwrap();

        let character_binary = fgcd_parse::binary::game::character::read_character(
                JOHNNY_CAGE,
                &game,
                &data_dir)
            .unwrap();

        println!("Character from binary: {:#?}", character_binary);

        assert_eq!(character, character_binary,
            "Character read from spreadsheet should match binary");
    }

    #[test]
    fn write_fixture_spreadsheets() {
        let game_fixture = fgcd_model::game::test::game_fixture();
        let character_fixture = fgcd_model::game::test::character_fixture();
        let data_dir = fgcd_data_dir();

        spreadsheet::game::new_game(
                game_fixture.name(),
                &data_dir)
            .unwrap();

        let (_, game_filepath) = spreadsheet::game::write_game(
                &game_fixture,
                &data_dir)
            .unwrap();

        let game = spreadsheet::game::read_game(
                game_fixture.name(),
                &game_filepath)
            .unwrap();

        assert_eq!(game_fixture, game,
            "Game fixture and game read should match: {:#?}\n\nVERSUS\n\n{:#?}", game_fixture, game);

        let (_, character_question_filepath) = spreadsheet::game::character::write_character(
                &character_fixture,
                &game,
                &data_dir)
            .unwrap();

        let character = spreadsheet::game::character::read_character(
                character_fixture.name(),
                &game,
                &character_question_filepath)
            .unwrap();

        assert_eq!(character_fixture, character);
    }
}