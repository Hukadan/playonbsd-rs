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
    pub cover: String,
    /// The engine used by the game.
    pub engine: String,
    /// Step(s) to setup the game.
    pub setup: String,
    /// The executable in the package.
    pub runtime: String,
    /// A vector with store urls.
    pub store: Vec<String>,
    /// Hints (as the name imply).
    pub hints: String,
    /// A vector of genres associated with the game.
    pub genres: Vec<String>,
    /// A vector of tags associated with the game.
    pub tags: Vec<String>,
    /// Released year.
    pub year: String,
    /// Developer.
    pub dev: String,
    /// Publisher.
    pub publi: String,
    /// Version of the game.
    pub version: String,
    /// When tested on -current.
    pub status: String,
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
    fn get_tags(&self) -> &Vec<String> {
        &self.tags
    }
    /// Returns the tag vector of the game.
    fn get_genres(&self) -> &Vec<String> {
        &self.genres
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
