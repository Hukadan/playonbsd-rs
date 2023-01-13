use crate::field::Field;
use crate::game::Game;

pub trait State {}

enum ParserState {
    Game,
    Cover,
    Engine,
    Setup,
    Runtime,
    Store,
    Hints,
    Genre,
    Tags,
    Year,
    Dev,
    Pub,
    Version,
    Status,
    Added,
    Updated,
    Error,
}

pub struct Parser {
    state: ParserState,
    games: Vec<Game>,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            state: ParserState::Game,
            games: Vec::new(),
        }
    }
    pub fn load_from_string(mut self, data: &str) -> Vec<Game> {
        for line in data.lines() {
            self.parse(line);
        }
        self.games
    }
    impl_parse![ParserState::Game, Field::Game, name, ParserState::Cover;
         (ParserState::Cover, Field::Cover, cover, ParserState::Engine);
         (ParserState::Engine, Field::Engine, engine, ParserState::Setup);
         (ParserState::Setup, Field::Setup, setup, ParserState::Runtime);
         (ParserState::Runtime, Field::Runtime, runtime, ParserState::Store);
         (ParserState::Store, Field::Store, stores, ParserState::Hints);
         (ParserState::Hints, Field::Hints, hints, ParserState::Genre);
         (ParserState::Genre, Field::Genres, genres, ParserState::Tags);
         (ParserState::Tags, Field::Tags, tags, ParserState::Year);
         (ParserState::Year, Field::Year, year, ParserState::Dev);
         (ParserState::Dev, Field::Dev, dev, ParserState::Pub);
         (ParserState::Pub, Field::Publi, publi, ParserState::Version);
         (ParserState::Version, Field::Version, version, ParserState::Status);
         (ParserState::Status, Field::Status, status, ParserState::Added);
         (ParserState::Added, Field::Added, added, ParserState::Updated);
         (ParserState::Updated, Field::Updated, updated, ParserState::Game)
    ];
}

#[cfg(test)]
mod test_parser {
    use super::*;

    #[test]
    fn test_parser() {
        let data = "Game	AaaaaAAaaaAAAaaAAAAaAAAAA!!! for the Awesome
Cover	AaaaaA_for_the_Awesome_Cover.jpg
Engine
Setup
Runtime	HumblePlay
Store	https://www.humblebundle.com/store/aaaaaaaaaaaaaaaaaaaaaaaaa-for-the-awesome
Hints	Demo on HumbleBundle store page
Genre
Tags
Year	2011
Dev
Pub
Version
Status
Added	1970-01-01
Updated	1970-01-01
Game	The Adventures of Mr. Hat
Cover
Engine	godot
Setup
Runtime	godot
Store	https://store.steampowered.com/app/1869200/The_Adventures_of_Mr_Hat/
Hints
Genre	Puzzle Platformer
Tags	indie
Year
Dev	AX-GAME
Pub	Fun Quarter
Version	Early Access
Status	runs (2022-05-13)
Added	2022-05-13
Updated	2022-05-13";
        let games = Parser::new().load_from_string(data);
        assert_eq!(games.len(), 2);
        if let Some(game) = games.get(0) {
            assert_eq!(game.name, "AaaaaAAaaaAAAaaAAAAaAAAAA!!! for the Awesome");
            assert_eq!(game.engine, None);
            assert_eq!(game.setup, None);
            assert_eq!(game.runtime, Some("HumblePlay".to_string()));
            assert_eq!(
                game.stores,
                Some(vec![
                    "https://www.humblebundle.com/store/aaaaaaaaaaaaaaaaaaaaaaaaa-for-the-awesome"
                        .to_string()
                ])
            );
            assert_eq!(game.genres, None);
            assert_eq!(game.year, Some("2011".to_string()));
            assert_eq!(game.dev, None);
            assert_eq!(game.publi, None);
            assert_eq!(game.version, None);
            assert_eq!(game.status, None);
            assert_eq!(game.added, Some("1970-01-01".to_string()));
            assert_eq!(game.updated, Some("1970-01-01".to_string()));
        }
    }
    #[test]
    fn test_parser_error() {
        let data_error = "Game	AaaaaAAaaaAAAaaAAAAaAAAAA!!! for the Awesome
Cover	AaaaaA_for_the_Awesome_Cover.jpg
Engine
Setup
Runtime	HumblePlay
Store	https://www.humblebundle.com/store/aaaaaaaaaaaaaaaaaaaaaaaaa-for-the-awesome
Hints	Demo on HumbleBundle store page
Genre
Tags
Year	2011
Dev
Pub
Version
//Status
Added	1970-01-01
Updated	1970-01-01
Game	The Adventures of Mr. Hat
Cover
Engine	godot
Setup
Runtime	godot
Store	https://store.steampowered.com/app/1869200/The_Adventures_of_Mr_Hat/
Hints
Genre	Puzzle Platformer
Tags	indie
Year
Dev	AX-GAME
Pub	Fun Quarter
Version	Early Access
Status	runs (2022-05-13)
Added	2022-05-13
Updated	2022-05-13";
        let games = Parser::new().load_from_string(data_error);
        assert_eq!(games.len(), 2);
        if let Some(game) = games.get(0) {
            assert_eq!(game.name, "AaaaaAAaaaAAAaaAAAAaAAAAA!!! for the Awesome");
            assert_eq!(game.engine, None);
            assert_eq!(game.setup, None);
            assert_eq!(game.runtime, Some("HumblePlay".to_string()));
            assert_eq!(
                game.stores,
                Some(vec![
                    "https://www.humblebundle.com/store/aaaaaaaaaaaaaaaaaaaaaaaaa-for-the-awesome"
                        .to_string()
                ])
            );
            assert_eq!(game.genres, None);
            assert_eq!(game.year, Some("2011".to_string()));
            assert_eq!(game.dev, None);
            assert_eq!(game.publi, None);
            assert_eq!(game.version, None);
            assert_eq!(game.status, None);
            assert_eq!(game.added, None);
            assert_eq!(game.updated, None);
        } 
        if let Some(game) = games.get(1) {
            assert_eq!(game.name, "The Adventures of Mr. Hat");
        }
    }
}
