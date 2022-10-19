use std::cmp::{Ordering, PartialOrd};

/// # Represent a game
/// A Game is created by a line starting by 'Game' in the database.
/// Once created, the fields are set by the lines following the
/// game entry.
/// ```
/// use pobsdlib::models::{Field, Game};
/// // typical lines of a game in the database
/// let database="Game	AaaaaAAaaaAAAaaAAAAaAAAAA!!! for the Awesome
/// Cover	AaaaaA_for_the_Awesome_Cover.jpg
/// Engine
/// Setup
/// Runtime	HumblePlay
/// Store	https://www.humblebundle.com/store/aaaaaaaaaaaaaaaaaaaaaaaaa-for-the-awesome
/// Hints	Demo on HumbleBundle store page
/// Genre
/// Tags
/// Year	2011
/// Dev
/// Pub
/// Version
/// Status";
///
/// ```
#[derive(Serialize, Clone, Default, Debug, PartialEq)]
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
    pub store: Option<Vec<String>>,
    /// Hints (as the name imply).
    pub hints: Option<String>,
    /// A vector of genres associated with the game.
    pub genres: Option<Vec<String>>,
    /// A vector of tags associated with the game.
    pub tags: Option<Vec<String>>,
    /// Released year.
    pub year: Option<String>,
    /// Developer.
    pub dev: Option<String>,
    /// Publisher.
    pub publi: Option<String>,
    /// Version of the game.
    pub version: Option<String>,
    /// When tested on -current.
    pub status: Option<String>,
}

impl Game {
    pub fn name_contains(&self, pattern: Option<&str>) -> bool {
        match pattern {
            // case insensitive
            Some(pattern) => self
                .name
                .to_lowercase()
                .contains(pattern.to_lowercase().as_str()),
            //if there is no patter everything matches
            None => true,
        }
    }
    pub fn engine_contains(&self, pattern: Option<&str>) -> bool {
        match pattern {
            // case insensitive
            Some(pattern) => match &self.engine {
                Some(engine) => engine
                    .to_lowercase()
                    .contains(pattern.to_lowercase().as_str()),
                None => false,
            },
            //if there is no patter everything matches
            None => true,
        }
    }
    pub fn runtime_contains(&self, pattern: Option<&str>) -> bool {
        match pattern {
            // case insensitive
            Some(pattern) => match &self.runtime {
                Some(runtime) => runtime
                    .to_lowercase()
                    .contains(pattern.to_lowercase().as_str()),
                None => false,
            },
            //if there is no patter everything matches
            None => true,
        }
    }
    pub fn genres_contains(&self, pattern: Option<&str>) -> bool {
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
            None => true,
        }
    }
    pub fn tags_contains(&self, pattern: Option<&str>) -> bool {
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
            None => true,
        }
    }
    pub fn year_contains(&self, pattern: Option<&str>) -> bool {
        match pattern {
            // case insensitive
            Some(pattern) => match &self.year {
                Some(year) => year
                    .to_lowercase()
                    .contains(pattern.to_lowercase().as_str()),
                None => false,
            },
            //if there is no patter everything matches
            None => true,
        }
    }
    pub fn dev_contains(&self, pattern: Option<&str>) -> bool {
        match pattern {
            // case insensitive
            Some(pattern) => match &self.dev {
                Some(dev) => dev.to_lowercase().contains(pattern.to_lowercase().as_str()),
                None => false,
            },
            //if there is no patter everything matches
            None => true,
        }
    }
    pub fn publi_contains(&self, pattern: Option<&str>) -> bool {
        match pattern {
            // case insensitive
            Some(pattern) => match &self.publi {
                Some(publi) => publi
                    .to_lowercase()
                    .contains(pattern.to_lowercase().as_str()),
                None => false,
            },
            //if there is no patter everything matches
            None => true,
        }
    }
}

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

/* ------------------------- TESTS --------------------------*/

#[cfg(test)]
mod test_game_methods {
    use super::*;
    fn create_game() -> Game {
        let mut game = Game::default();
        let tags: Vec<String> = vec!["tag1".to_string(), "tag2".to_string()];
        let genres: Vec<String> = vec!["genre1".to_string(), "genre2".to_string()];
        let store: Vec<String> = vec!["store1".to_string(), "store2".to_string()];
        game.name = "game name".to_string();
        game.cover = Some("cover.jpg".to_string());
        game.engine = Some("game engine".to_string());
        game.setup = Some("game setup".to_string());
        game.runtime = Some("game runtime".to_string());
        game.store = Some(store);
        game.hints = Some("game hints".to_string());
        game.genres = Some(genres);
        game.tags = Some(tags);
        game.year = Some("1980".to_string());
        game.dev = Some("game dev".to_string());
        game.publi = Some("game publi".to_string());
        game.version = Some("game version".to_string());
        game.status = Some("game status".to_string());
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
        assert!(game.name_contains(None));
        assert!(game.name_contains(Some(&"name")));
        assert!(!game.name_contains(Some(&"not sure")));
    }
    #[test]
    fn engine_contains() {
        let mut game = create_game();
        assert!(game.engine_contains(None));
        assert!(game.engine_contains(Some(&"engine")));
        assert!(!game.engine_contains(Some(&"not sure")));
        game.engine = None;
        assert!(game.engine_contains(None));
        assert!(!game.engine_contains(Some(&"engine")));
    }
    #[test]
    fn runtime_contains() {
        let mut game = create_game();
        assert!(game.runtime_contains(None));
        assert!(game.runtime_contains(Some(&"runtime")));
        assert!(!game.runtime_contains(Some(&"not sure")));
        game.runtime = None;
        assert!(game.runtime_contains(None));
        assert!(!game.runtime_contains(Some(&"runtime")));
    }
    #[test]
    fn genres_contains() {
        let mut game = create_game();
        assert!(game.genres_contains(None));
        assert!(game.genres_contains(Some(&"genre")));
        assert!(!game.genres_contains(Some(&"not sure")));
        game.genres = None;
        assert!(game.genres_contains(None));
        assert!(!game.genres_contains(Some(&"genre")));
    }
    #[test]
    fn tags_contains() {
        let mut game = create_game();
        assert!(game.tags_contains(None));
        assert!(game.tags_contains(Some(&"tag")));
        assert!(!game.tags_contains(Some(&"not sure")));
        game.tags = None;
        assert!(game.tags_contains(None));
        assert!(!game.tags_contains(Some(&"tag")));
    }
    #[test]
    fn year_contains() {
        let mut game = create_game();
        assert!(game.year_contains(None));
        assert!(game.year_contains(Some(&"1980")));
        assert!(!game.year_contains(Some(&"not sure")));
        game.year = None;
        assert!(game.year_contains(None));
        assert!(!game.year_contains(Some(&"1980")));
    }
    #[test]
    fn dev_contains() {
        let mut game = create_game();
        assert!(game.dev_contains(None));
        assert!(game.dev_contains(Some(&"dev")));
        assert!(!game.dev_contains(Some(&"not sure")));
        game.dev = None;
        assert!(game.dev_contains(None));
        assert!(!game.dev_contains(Some(&"dev")));
    }
    #[test]
    fn publi_contains() {
        let mut game = create_game();
        assert!(game.publi_contains(None));
        assert!(game.publi_contains(Some(&"publi")));
        assert!(!game.publi_contains(Some(&"not sure")));
        game.publi = None;
        assert!(game.publi_contains(None));
        assert!(!game.publi_contains(Some(&"publi")));
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
}
