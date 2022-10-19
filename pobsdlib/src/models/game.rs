use crate::traits::{BasicItem, GameItem};

/// # Represent a game
/// A Game is created by a line starting by 'Game' in the database.
/// Once created, the fields are set by the lines following the
/// game entry.
/// ```
/// use pobsdlib::models::{Field, Game};
/// use pobsdlib::traits::{BasicItem, GameItem};
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

impl BasicItem for Game {
    /// Is equivalent to Game::Default().
    fn new() -> Self {
        Self::default()
    }
    /// Returns the name of the item
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_id(&self) -> usize {
        self.id
    }
    /// Sets the id of the game.
    fn set_id(&mut self, id: usize) {
        self.id = id;
    }
}

impl GameItem for Game {
    /// Returns the tag vector of the game.
    fn get_tags(&self) -> Option<&Vec<String>> {
        match &self.tags {
            Some(tags) => Some(&tags),
            None => None,
        }
    }
    /// Returns the tag vector of the game.
    fn get_genres(&self) -> Option<&Vec<String>> {
        match &self.genres {
            Some(genres) => Some(&genres),
            None => None,
        }
    }
}

/* ------------------------- TESTS --------------------------*/

#[cfg(test)]
mod test_game_methods {
    use super::*;
    #[test]
    fn default_is_new() {
        let game = Game::new();
        let game_bis = Game::default();
        assert!(game == game_bis);
    }
}
