use std::collections::HashMap;

use crate::collections::QueryResult;
use crate::models::{Game, Item};
use crate::utils::load_database;

/// Store the game database in different collections.
/// With the exception of the get_game_by_id query,
/// all queries performed on the database return a
/// QueryResult.
///
/// ## The game collection
/// The game collection is stored using a HashMap.
/// The id of each game is used as key while the 
/// value is the corresponding Game struct.
/// Most of the queries are performed using this
/// HashMap.
///
/// ## The item collections
/// Each item collection is stored using a HashMap.
/// The name of each item is used as a key while
/// the value is the corresponding Item struct.
///
/// The following item collections are available for 
/// searching:
/// - engines
/// - runtimes
/// - genres
/// - tags
/// - years
/// - devs
/// - publishers
///
/// Those collections are used to retrieve all games
/// associated with a specific item.
///
/// ## Limitations
/// In its current state, queries cannot be chained
/// to obtain complex queries. However, with the method
/// game_contains_and and game_contains_or most of the
/// useful queries can be performed.
///
#[derive(Serialize, Clone, Default, Debug, PartialEq, Eq)]
pub struct DataBase {
    pub(crate) games: HashMap<usize, Game>,
    pub(crate) engines: HashMap<String, Item>,
    pub(crate) runtimes: HashMap<String, Item>,
    pub(crate) genres: HashMap<String, Item>,
    pub(crate) tags: HashMap<String, Item>,
    pub(crate) years: HashMap<String, Item>,
    pub(crate) devs: HashMap<String, Item>,
    pub(crate) publis: HashMap<String, Item>,
}

impl DataBase {
    /// Create a database from a database file.
    /// Please see <https://github.com/playonbsd/OpenBSD-Games-Database>
    /// for more information.
    pub fn new(filename: &str) -> Self {
        let mut database = Self::default();
        load_database(&mut database, filename);
        database
    }
    /// Return all games of the database.
    pub fn get_all_games(&self) -> QueryResult<&Game> {
        let games = self.games.values().collect();
        QueryResult::new(games)
    }
    /// Return the game the the given id
    pub fn get_game_by_id(&self, id: usize) -> Option<&Game> {
        self.games.get(&id)
    }
    /// Return the games of the database with the given name.
    /// It preforms an exact matching.
    /// Note that nothing forbids two games to have the same name.
    /// Hence, it does not behave like get_game_by_id but 
    /// returns a QueryResult.
    pub fn get_game_by_name(&self, name: &str) -> QueryResult<&Game> {
        let gs = self.games.iter().filter(|&(_, item)| item.name == name);
        let mut games: Vec<&Game> = Vec::new();
        for (_, item) in gs {
            games.push(item);
        }
        QueryResult::new(games)
    }
    /// Return the games of the database using the given engine.
    /// It performs an exact matching.
    pub fn get_game_by_engine(&self, name: &str) -> QueryResult<&Game> {
        let mut games: Vec<&Game> = Vec::new();
        if let Some(engine) = self.engines.get(name) {
            for id in &engine.games {
                games.push(self.games.get(id).unwrap());
            }
        }
        QueryResult::new(games)
    }
    /// Return the games of the database using the givent runtime.
    /// It performs an exact matching.
    pub fn get_game_by_runtime(&self, name: &str) -> QueryResult<&Game> {
        let mut games: Vec<&Game> = Vec::new();
        if let Some(runtime) = self.runtimes.get(name) {
            for id in &runtime.games {
                games.push(self.games.get(id).unwrap());
            }
        }
        QueryResult::new(games)
    }
    /// Return the games of the database classified in the given genre.
    /// It performs an exact matching.
    pub fn get_game_by_genre(&self, name: &str) -> QueryResult<&Game> {
        let mut games: Vec<&Game> = Vec::new();
        if let Some(genre) = self.genres.get(name) {
            for id in &genre.games {
                games.push(self.games.get(id).unwrap());
            }
        }
        QueryResult::new(games)
    }
    /// Return the games of the database classified in the given tag.
    /// It performs an exact matching.
    pub fn get_game_by_tag(&self, name: &str) -> QueryResult<&Game> {
        let mut games: Vec<&Game> = Vec::new();
        if let Some(tag) = self.tags.get(name) {
            for id in &tag.games {
                games.push(self.games.get(id).unwrap());
            }
        }
        QueryResult::new(games)
    }
    /// Return the games of the database released in the given year.
    /// It performs an exact matching.
    pub fn get_game_by_year(&self, year: &str) -> QueryResult<&Game> {
        let mut games: Vec<&Game> = Vec::new();
        if let Some(year) = self.years.get(&year.to_string()) {
            for id in &year.games {
                games.push(self.games.get(id).unwrap());
            }
        }
        QueryResult::new(games)
    }
    /// Return the games of the database developped by the given developper.
    /// It performs an exact matching.
    pub fn get_game_by_dev(&self, name: &str) -> QueryResult<&Game> {
        let mut games: Vec<&Game> = Vec::new();
        if let Some(dev) = self.devs.get(name) {
            for id in &dev.games {
                games.push(self.games.get(id).unwrap());
            }
        }
        QueryResult::new(games)
    }
    /// Return the games of the database published by the given publisher.
    /// It performs an exact matching.
    pub fn get_game_by_publi(&self, name: &str) -> QueryResult<&Game> {
        let mut games: Vec<&Game> = Vec::new();
        if let Some(publi) = self.publis.get(name) {
            for id in &publi.games {
                games.push(self.games.get(id).unwrap());
            }
        }
        QueryResult::new(games)
    }
    /// Return the games that **contains**
    /// the given string for each field.
    /// When a field is not searched, it should
    /// be set to None.
    /// The search is done performing `AND` 
    /// between the fields.
    pub fn game_contains_and(
        &self,
        name: Option<&str>,
        engine: Option<&str>,
        runtime: Option<&str>,
        genre: Option<&str>,
        tag: Option<&str>,
        year: Option<&str>,
        dev: Option<&str>,
        publi: Option<&str>,
    ) -> QueryResult<&Game> {
        let gs = self.games.iter().filter(|&(_, item)| {
            item.name_contains(name)
                && item.engine_contains(engine)
                && item.runtime_contains(runtime)
                && item.genres_contains(genre)
                && item.tags_contains(tag)
                && item.year_contains(year)
                && item.dev_contains(dev)
                && item.publi_contains(publi)
        });
        let mut games: Vec<&Game> = Vec::new();
        for (_, item) in gs {
            games.push(item);
        }
        QueryResult::new(games)
    }
    /// Return the games that **contains**
    /// the given string for each field.
    /// When a field is not searched, it should
    /// be set to None.
    /// The search is done performing `OR` 
    /// between the fields.
    pub fn game_contains_or(
        &self,
        name: Option<&str>,
        engine: Option<&str>,
        runtime: Option<&str>,
        genre: Option<&str>,
        tag: Option<&str>,
        year: Option<&str>,
        dev: Option<&str>,
        publi: Option<&str>,
    ) -> QueryResult<&Game> {
        let gs = self.games.iter().filter(|&(_, item)| {
            item.name_contains(name)
                || item.engine_contains(engine)
                || item.runtime_contains(runtime)
                || item.genres_contains(genre)
                || item.tags_contains(tag)
                || item.year_contains(year)
                || item.dev_contains(dev)
                || item.publi_contains(publi)
        });
        let mut games: Vec<&Game> = Vec::new();
        for (_, item) in gs {
            games.push(item);
        }
        QueryResult::new(games)
    }
    /// Return all engines of the database.
    pub fn get_all_engines(&self) -> QueryResult<&Item> {
        let engines = self.engines.values().collect();
        QueryResult::new(engines)
    }
    /// Return all runtimes of the database.
    pub fn get_all_runtimes(&self) -> QueryResult<&Item> {
        let engines = self.runtimes.values().collect();
        QueryResult::new(engines)
    }
    /// Return all genres of the database.
    pub fn get_all_genres(&self) -> QueryResult<&Item> {
        let genres = self.genres.values().collect();
        QueryResult::new(genres)
    }
    /// Return all tags of the database.
    pub fn get_all_tags(&self) -> QueryResult<&Item> {
        let tags = self.tags.values().collect();
        QueryResult::new(tags)
    }
    /// Return all years of the database.
    pub fn get_all_years(&self) -> QueryResult<&Item> {
        let years = self.years.values().collect();
        QueryResult::new(years)
    }
    /// Return all developpers of the database.
    pub fn get_all_devs(&self) -> QueryResult<&Item> {
        let devs = self.devs.values().collect();
        QueryResult::new(devs)
    }
    /// Return all publishers of the database.
    pub fn get_all_publis(&self) -> QueryResult<&Item> {
        let publis = self.publis.values().collect();
        QueryResult::new(publis)
    }
}
