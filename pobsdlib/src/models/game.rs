use std::cmp::{Ordering, PartialOrd};
use std::fmt;

/// # Represent a game
/// The Game struct represents a game from the database
/// with an additional id which represents the position
/// in the database.
/// See <https://github.com/playonbsd/OpenBSD-Games-Database>
/// for details.
/// The name of some fields differs from the one used
/// in the database itself: Genre and Store are plural
/// since there can be more than one item for each
/// and Pub translate to publi since pub is a reserved
/// keyword in Rust.
/// A String type is used for Year since sometimes the
/// release date can only be described by textw (e.g.
/// "early acess").
#[derive(Serialize, Clone, Default, Debug, PartialEq, Eq)]
pub struct Game {
    /// The id of the game.
    pub id: usize,
    /// The name of the game.
    pub name: String,
    /// The cover of the game.
    pub cover: Option<String>,
    /// The engine used by the game.
    pub engine: Option<String>,
    /// Step(s) to setup the game.
    pub setup: Option<String>,
    /// The executable in the package.
    pub runtime: Option<String>,
    /// A vector with store urls.
    pub stores: Option<Vec<String>>,
    /// Hints (as the name imply).
    pub hints: Option<String>,
    /// A vector of genres associated with the game.
    pub genres: Option<Vec<String>>,
    /// A vector of tags associated with the game.
    pub tags: Option<Vec<String>>,
    /// Released year (can be text such as "early access".
    pub year: Option<String>,
    /// Developer.
    pub dev: Option<String>,
    /// Publisher.
    #[serde(rename = "pub")]
    pub publi: Option<String>,
    /// Version of the game.
    pub version: Option<String>,
    /// When tested on -current.
    pub status: Option<String>,
    /// When added
    pub added: Option<String>,
    /// When updated
    pub updated: Option<String>,
}

/// For now games are ordered by id.
/// This will probably be dropped in the
/// future in favor of alphabetical ordering
/// on the name.
impl PartialOrd for Game {
    fn partial_cmp(&self, other: &Game) -> Option<Ordering> {
        self.id.partial_cmp(&other.id)
    }
    fn lt(&self, other: &Game) -> bool {
        self.id.lt(&other.id)
    }
    fn le(&self, other: &Game) -> bool {
        self.id.le(&other.id)
    }
    fn gt(&self, other: &Game) -> bool {
        self.id.gt(&other.id)
    }
    fn ge(&self, other: &Game) -> bool {
        self.id.ge(&other.id)
    }
}

/// Display the game as it would appears in the database.
/// See <https://github.com/playonbsd/OpenBSD-Games-Database>
/// for details.
impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let game = format!("Game\t{}", self.name);
        let cover = match &self.cover {
            Some(cover) => format!("Cover\t{}", cover),
            None => "Cover".to_string(),
        };
        let engine = match &self.engine {
            Some(engine) => format!("Engine\t{}", engine),
            None => "Engine".to_string(),
        };
        let setup = match &self.setup {
            Some(setup) => format!("Setup\t{}", setup),
            None => "Setup".to_string(),
        };
        let runtime = match &self.runtime {
            Some(runtime) => format!("Runtime\t{}", runtime),
            None => "Runtime".to_string(),
        };
        let stores = match &self.stores {
            Some(stores) => format!("Store\t{}", stores.join(" ")),
            None => "Store".to_string(),
        };
        let hints = match &self.hints {
            Some(hints) => format!("Hints\t{}", hints),
            None => "Hints".to_string(),
        };
        let genres = match &self.genres {
            Some(genres) => format!("Genre\t{}", genres.join(", ")),
            None => "Genre".to_string(),
        };
        let tags = match &self.tags {
            Some(tags) => format!("Tags\t{}", tags.join(", ")),
            None => "Tags".to_string(),
        };
        let year = match &self.year {
            Some(year) => format!("Year\t{}", year),
            None => "Year".to_string(),
        };
        let dev = match &self.dev {
            Some(dev) => format!("Dev\t{}", dev),
            None => "Dev".to_string(),
        };
        let publi = match &self.publi {
            Some(publi) => format!("Pub\t{}", publi),
            None => "Pub".to_string(),
        };
        let version = match &self.version {
            Some(version) => format!("Version\t{}", version),
            None => "Version".to_string(),
        };
        let status = match &self.status {
            Some(status) => format!("Status\t{}", status),
            None => "Status".to_string(),
        };
        let added = match &self.added {
            Some(added) => format!("Added\t{}", added),
            None => "Added".to_string(),
        };
        let updated = match &self.updated {
            Some(updated) => format!("Updated\t{}", updated),
            None => "Updated".to_string(),
        };
        write!(
            f,
            "{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}",
            game,
            cover,
            engine,
            setup,
            runtime,
            stores,
            hints,
            genres,
            tags,
            year,
            dev,
            publi,
            version,
            status,
            added,
            updated,
        )
    }
}

impl Game {
    /// Return true if the name of the game contains the
    /// given pattern, false otherwise. It is not case
    /// sensitive.
    /// If None is given, returns true.
    pub fn name_contains(&self, pattern: Option<&str>, default: bool) -> bool {
        match pattern {
            // case insensitive
            Some(pattern) => self
                .name
                .to_lowercase()
                .contains(pattern.to_lowercase().as_str()),
            //if there is no patter everything matches
            None => default,
        }
    }
    /// Return true if the engine of the game contains the
    /// given pattern, false otherwise. It is not case
    /// sensitive.
    /// If None is given, returns true.
    pub fn engine_contains(&self, pattern: Option<&str>, default: bool) -> bool {
        match pattern {
            // case insensitive
            Some(pattern) => match &self.engine {
                Some(engine) => engine
                    .to_lowercase()
                    .contains(pattern.to_lowercase().as_str()),
                None => false,
            },
            //if there is no patter everything matches
            None => default,
        }
    }
    /// Return true if the runtime of the game contains the
    /// given pattern, false otherwise. It is not case
    /// sensitive.
    /// If None is given, returns true.
    pub fn runtime_contains(&self, pattern: Option<&str>, default: bool) -> bool {
        match pattern {
            // case insensitive
            Some(pattern) => match &self.runtime {
                Some(runtime) => runtime
                    .to_lowercase()
                    .contains(pattern.to_lowercase().as_str()),
                None => false,
            },
            //if there is no patter everything matches
            None => default,
        }
    }
    /// Return true if the genres of the game contains the
    /// given pattern, false otherwise. It is not case
    /// sensitive.
    /// If None is given, returns true.
    pub fn genres_contains(&self, pattern: Option<&str>, default: bool) -> bool {
        match pattern {
            // case insensitive
            Some(pattern) => match &self.genres {
                Some(genres) => genres
                    .join(" ~~ ")
                    .to_lowercase()
                    .contains(pattern.to_lowercase().as_str()),
                None => false,
            },
            //if there is no patter everything matches
            None => default,
        }
    }
    /// Return true if the tags of the game contains the
    /// given pattern, false otherwise. It is not case
    /// sensitive.
    /// If None is given, returns true.
    pub fn tags_contains(&self, pattern: Option<&str>, default: bool) -> bool {
        match pattern {
            // case insensitive
            Some(pattern) => match &self.tags {
                Some(tags) => tags
                    .join(" ~~ ")
                    .to_lowercase()
                    .contains(pattern.to_lowercase().as_str()),
                None => false,
            },
            //if there is no patter everything matches
            None => default,
        }
    }
    /// Return true if the years of the game contains the
    /// given pattern, false otherwise. It is not case
    /// sensitive.
    /// If None is given, returns true.
    pub fn year_contains(&self, pattern: Option<&str>, default: bool) -> bool {
        match pattern {
            // case insensitive
            Some(pattern) => match &self.year {
                Some(year) => year
                    .to_lowercase()
                    .contains(pattern.to_lowercase().as_str()),
                None => false,
            },
            //if there is no patter everything matches
            None => default,
        }
    }
    /// Return true if the devs of the game contains the
    /// given pattern, false otherwise. It is not case
    /// sensitive.
    /// If None is given, returns true.
    pub fn dev_contains(&self, pattern: Option<&str>, default: bool) -> bool {
        match pattern {
            // case insensitive
            Some(pattern) => match &self.dev {
                Some(dev) => dev.to_lowercase().contains(pattern.to_lowercase().as_str()),
                None => false,
            },
            //if there is no patter everything matches
            None => default,
        }
    }
    /// Return true if the pub of the game contains the
    /// given pattern, false otherwise. It is not case
    /// sensitive.
    /// If None is given, returns true.
    pub fn publi_contains(&self, pattern: Option<&str>, default: bool) -> bool {
        match pattern {
            // case insensitive
            Some(pattern) => match &self.publi {
                Some(publi) => publi
                    .to_lowercase()
                    .contains(pattern.to_lowercase().as_str()),
                None => false,
            },
            //if there is no patter everything matches
            None => default,
        }
    }
}

/* ------------------------- TESTS --------------------------*/

#[cfg(test)]
mod test_game_methods {
    use super::*;
    fn create_game() -> Game {
        let mut game = Game::default();
        let tags: Vec<String> = vec!["tag1".to_string(), "tag2".to_string()];
        let genres: Vec<String> = vec!["genre1".to_string(), "genre2".to_string()];
        let stores: Vec<String> = vec!["store1".to_string(), "store2".to_string()];
        game.name = "game name".to_string();
        game.cover = Some("cover.jpg".to_string());
        game.engine = Some("game engine".to_string());
        game.setup = Some("game setup".to_string());
        game.runtime = Some("game runtime".to_string());
        game.stores = Some(stores);
        game.hints = Some("game hints".to_string());
        game.genres = Some(genres);
        game.tags = Some(tags);
        game.year = Some("1980".to_string());
        game.dev = Some("game dev".to_string());
        game.publi = Some("game publi".to_string());
        game.version = Some("game version".to_string());
        game.status = Some("game status".to_string());
        game.added = Some("2012-12-03".to_string());
        game.updated = Some("2014-12-03".to_string());
        game
    }
    #[test]
    fn default_is_new() {
        let game = Game::default();
        let game_bis = Game::default();
        assert!(game == game_bis);
    }
    #[test]
    fn name_contains() {
        let game = create_game();
        assert!(game.name_contains(None, true));
        assert!(game.name_contains(Some(&"name"), true));
        assert!(!game.name_contains(Some(&"not sure"), true));
    }
    #[test]
    fn engine_contains() {
        let mut game = create_game();
        assert!(game.engine_contains(None, true));
        assert!(game.engine_contains(Some(&"engine"), true));
        assert!(!game.engine_contains(Some(&"not sure"), true));
        game.engine = None;
        assert!(game.engine_contains(None, true));
        assert!(!game.engine_contains(Some(&"engine"), true));
    }
    #[test]
    fn runtime_contains() {
        let mut game = create_game();
        assert!(game.runtime_contains(None, true));
        assert!(game.runtime_contains(Some(&"runtime"), true));
        assert!(!game.runtime_contains(Some(&"not sure"), true));
        game.runtime = None;
        assert!(game.runtime_contains(None, true));
        assert!(!game.runtime_contains(Some(&"runtime"), true));
    }
    #[test]
    fn genres_contains() {
        let mut game = create_game();
        assert!(game.genres_contains(None, true));
        assert!(game.genres_contains(Some(&"genre"), true));
        assert!(!game.genres_contains(Some(&"not sure"), true));
        game.genres = None;
        assert!(game.genres_contains(None, true));
        assert!(!game.genres_contains(Some(&"genre"), true));
    }
    #[test]
    fn tags_contains() {
        let mut game = create_game();
        assert!(game.tags_contains(None, true));
        assert!(game.tags_contains(Some(&"tag"), true));
        assert!(!game.tags_contains(Some(&"not sure"), true));
        game.tags = None;
        assert!(game.tags_contains(None, true));
        assert!(!game.tags_contains(Some(&"tag"), true));
    }
    #[test]
    fn year_contains() {
        let mut game = create_game();
        assert!(game.year_contains(None, true));
        assert!(game.year_contains(Some(&"1980"), true));
        assert!(!game.year_contains(Some(&"not sure"), true));
        game.year = None;
        assert!(game.year_contains(None, true));
        assert!(!game.year_contains(Some(&"1980"), true));
    }
    #[test]
    fn dev_contains() {
        let mut game = create_game();
        assert!(game.dev_contains(None, true));
        assert!(game.dev_contains(Some(&"dev"), true));
        assert!(!game.dev_contains(Some(&"not sure"), true));
        game.dev = None;
        assert!(game.dev_contains(None, true));
        assert!(!game.dev_contains(Some(&"dev"), true));
    }
    #[test]
    fn publi_contains() {
        let mut game = create_game();
        assert!(game.publi_contains(None, true));
        assert!(game.publi_contains(Some(&"publi"), true));
        assert!(!game.publi_contains(Some(&"not sure"), true));
        game.publi = None;
        assert!(game.publi_contains(None, true));
        assert!(!game.publi_contains(Some(&"publi"), true));
    }
    #[test]
    fn test_ordering() {
        let mut game1 = create_game();
        let mut game2 = create_game();
        game1.id = 1;
        game2.id = 2;
        assert!(game2.gt(&game1));
        assert!(game2.ge(&game1));
        assert!(game1.le(&game2));
        assert!(game1.lt(&game2));
    }
    #[test]
    fn test_display() {
        let game_str = "Game\tAaaaaAAaaaAAAaaAAAAaAAAAA!!! for the Awesome
Cover\tAaaaaA_for_the_Awesome_Cover.jpg
Engine
Setup
Runtime\tHumblePlay
Store\thttps://www.humblebundle.com/store/aaaaaaaaaaaaaaaaaaaaaaaaa-for-the-awesome
Hints\tDemo on HumbleBundle store page
Genre
Tags
Year\t2011
Dev
Pub
Version
Status
Added
Updated";
        let game = Game {
            id: 1,
            name: "AaaaaAAaaaAAAaaAAAAaAAAAA!!! for the Awesome".to_string(),
            cover: Some("AaaaaA_for_the_Awesome_Cover.jpg".to_string()),
            engine: None,
            setup: None,
            runtime: Some("HumblePlay".to_string()),
            stores: Some(vec![
                "https://www.humblebundle.com/store/aaaaaaaaaaaaaaaaaaaaaaaaa-for-the-awesome"
                    .to_string(),
            ]),
            hints: Some("Demo on HumbleBundle store page".to_string()),
            genres: None,
            tags: None,
            year: Some("2011".to_string()),
            dev: None,
            publi: None,
            version: None,
            status: None,
            added: None,
            updated: None,
        };
        assert_eq!(format!("{}", game), game_str);
    }
}
