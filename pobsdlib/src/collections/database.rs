use std::collections::HashMap;

use crate::collections::QuerySet;
use crate::models::{Game, Item};
use crate::utils::load_database;

/// # DataBase
/// Store the game database in three different collection:
/// - a games collection
/// - a tags collection
/// - a genres collection
///
/// The games collection also stores a vector of Game.
/// being described using the following struct:
///
#[derive(Serialize, Clone, Default, Debug, PartialEq)]
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

/// Public API
impl DataBase {
    /// Create a database from a file
    pub fn new(filename: &str) -> Self {
        let mut database = Self::default();
        load_database(&mut database, filename);
        database
    }
    /// Return all games
    pub fn get_all_games(&self) -> QuerySet<&Game> {
        let games = self.games.values().collect();
        QuerySet::new(games)
    }
    /// Return the game the the given id
    pub fn get_game_by_id(&self, id: usize) -> Option<&Game> {
        self.games.get(&id)
    }
    /// Return the game with the given name
    /// Note that nothing forbids two games
    /// to have the same name. Hence, it
    /// returns a QuerySet.
    pub fn get_game_by_name(&self, name: &str) -> QuerySet<&Game> {
        let gs = self.games.iter().filter(|&(_, item)| item.name == name);
        let mut games: Vec<&Game> = Vec::new();
        for (_, item) in gs {
            games.push(item);
        }
        QuerySet::new(games)
    }
    /// Return the games associated with a given
    /// engine.
    pub fn get_game_by_engine(&self, name: &str) -> QuerySet<&Game> {
        let engine = self.engines.get(name).unwrap();
        let mut games: Vec<&Game> = Vec::new();
        for id in &engine.games {
            games.push(self.games.get(id).unwrap());
        }
        QuerySet::new(games)
    }
    /// Return the games associated with a given
    /// runtime.
    pub fn get_game_by_runtime(&self, name: &str) -> QuerySet<&Game> {
        let runtime = self.runtimes.get(name).unwrap();
        let mut games: Vec<&Game> = Vec::new();
        for id in &runtime.games {
            games.push(self.games.get(id).unwrap());
        }
        QuerySet::new(games)
    }
    /// Return the games associated with a given
    /// genre.
    pub fn get_game_by_genre(&self, name: &str) -> QuerySet<&Game> {
        let genre = self.genres.get(name).unwrap();
        let mut games: Vec<&Game> = Vec::new();
        for id in &genre.games {
            games.push(self.games.get(id).unwrap());
        }
        QuerySet::new(games)
    }
    /// Return the games associated with a given
    /// tag.
    pub fn get_game_by_tag(&self, name: &str) -> QuerySet<&Game> {
        let tag = self.tags.get(name).unwrap();
        let mut games: Vec<&Game> = Vec::new();
        for id in &tag.games {
            games.push(self.games.get(id).unwrap());
        }
        QuerySet::new(games)
    }
    /// Return the games associated with a given
    /// year.
    pub fn get_game_by_year(&self, year: &str) -> QuerySet<&Game> {
        let year = self.years.get(&year.to_string()).unwrap();
        let mut games: Vec<&Game> = Vec::new();
        for id in &year.games {
            games.push(self.games.get(id).unwrap());
        }
        QuerySet::new(games)
    }
    /// Return the games associated with a given
    /// dev.
    pub fn get_game_by_dev(&self, name: &str) -> QuerySet<&Game> {
        let dev = self.devs.get(name).unwrap();
        let mut games: Vec<&Game> = Vec::new();
        for id in &dev.games {
            games.push(self.games.get(id).unwrap());
        }
        QuerySet::new(games)
    }
    /// Return the games associated with a given
    /// publisher.
    pub fn get_game_by_publi(&self, name: &str) -> QuerySet<&Game> {
        let publi = self.publis.get(name).unwrap();
        let mut games: Vec<&Game> = Vec::new();
        for id in &publi.games {
            games.push(self.games.get(id).unwrap());
        }
        QuerySet::new(games)
    }
    /// Return the games associated that contains
    /// the given string for each field. When a
    /// field is not searched, it should be set to
    /// None. The search is done performing AND
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
    ) -> QuerySet<&Game> {
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
        QuerySet::new(games)
    }
    /// Return the games associated that contains
    /// the given string for each field. When a
    /// field is not searched, it should be set to
    /// None. The search is done performing OR
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
    ) -> QuerySet<&Game> {
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
        QuerySet::new(games)
    }
    /// Return all engines
    pub fn get_all_engines(&self) -> QuerySet<&Item> {
        let engines = self.engines.values().collect();
        QuerySet::new(engines)
    }
    /// Return all runtimes
    pub fn get_all_runtimes(&self) -> QuerySet<&Item> {
        let engines = self.runtimes.values().collect();
        QuerySet::new(engines)
    }
    /// Return all genres
    pub fn get_all_genres(&self) -> QuerySet<&Item> {
        let genres = self.genres.values().collect();
        QuerySet::new(genres)
    }
    /// Return all tags
    pub fn get_all_tags(&self) -> QuerySet<&Item> {
        let tags = self.tags.values().collect();
        QuerySet::new(tags)
    }
    /// Return all years
    pub fn get_all_years(&self) -> QuerySet<&Item> {
        let years = self.years.values().collect();
        QuerySet::new(years)
    }
    /// Return all devs
    pub fn get_all_devs(&self) -> QuerySet<&Item> {
        let devs = self.devs.values().collect();
        QuerySet::new(devs)
    }
    /// Return all publis
    pub fn get_all_publis(&self) -> QuerySet<&Item> {
        let publis = self.publis.values().collect();
        QuerySet::new(publis)
    }
}
