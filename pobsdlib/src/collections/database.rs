use std::collections::HashMap;

use crate::collections::QuerySet;
use crate::models::Game;
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
    pub(crate) engines: HashMap<String, Vec<usize>>,
    pub(crate) runtimes: HashMap<String, Vec<usize>>,
    pub(crate) genres: HashMap<String, Vec<usize>>,
    pub(crate) tags: HashMap<String, Vec<usize>>,
    pub(crate) years: HashMap<String, Vec<usize>>,
    pub(crate) devs: HashMap<String, Vec<usize>>,
    pub(crate) publis: HashMap<String, Vec<usize>>,
}

/// Public API
impl DataBase {
    /// Create a database from a file
    pub fn new(filename: &str) -> Self {
        let mut database = Self::default();
        load_database(&mut database, filename);
        database
    }
    pub fn get_game_by_id(&self, id: &usize) -> Option<&Game> {
        self.games.get(id)
    }
    pub fn get_all_games(&self) -> QuerySet<&Game> {
        let games = self.games.values().collect();
        QuerySet::new(games)
    }
    pub fn get_game_by_name(&self, name: &str) -> QuerySet<&Game> {
        let gs = self.games.iter().filter(|&(_, item)| item.name == name);
        let mut games: Vec<&Game> = Vec::new();
        for (_, item) in gs {
            games.push(item);
        }
        QuerySet::new(games)
    }
    pub fn get_game_by_name_contains(&self, search: &str) -> QuerySet<&Game> {
        let gs = self.games.iter().filter(|&(_, item)| {
            item.name
                .to_lowercase()
                .contains(search.to_lowercase().as_str())
        });
        let mut games: Vec<&Game> = Vec::new();
        for (_, item) in gs {
            games.push(item);
        }
        QuerySet::new(games)
    }
    pub fn get_game_by_engine(&self, name: &str) -> QuerySet<&Game> {
        let game_ids = self.engines.get(name).unwrap();
        let mut games: Vec<&Game> = Vec::new();
        for id in game_ids {
            games.push(self.games.get(id).unwrap());
        }
        QuerySet::new(games)
    }
    pub fn get_game_by_runtime(&self, name: &str) -> QuerySet<&Game> {
        let game_ids = self.runtimes.get(name).unwrap();
        let mut games: Vec<&Game> = Vec::new();
        for id in game_ids {
            games.push(self.games.get(id).unwrap());
        }
        QuerySet::new(games)
    }
    pub fn get_game_by_genre(&self, name: &str) -> QuerySet<&Game> {
        let game_ids = self.genres.get(name).unwrap();
        let mut games: Vec<&Game> = Vec::new();
        for id in game_ids {
            games.push(self.games.get(id).unwrap());
        }
        QuerySet::new(games)
    }
    pub fn get_game_by_tag(&self, name: &str) -> QuerySet<&Game> {
        let game_ids = self.tags.get(name).unwrap();
        let mut games: Vec<&Game> = Vec::new();
        for id in game_ids {
            games.push(self.games.get(id).unwrap());
        }
        QuerySet::new(games)
    }
    pub fn get_game_by_year(&self, year: String) -> QuerySet<&Game> {
        let game_ids = self.years.get(&year).unwrap();
        let mut games: Vec<&Game> = Vec::new();
        for id in game_ids {
            games.push(self.games.get(id).unwrap());
        }
        QuerySet::new(games)
    }
    pub fn get_game_by_dev(&self, name: &str) -> QuerySet<&Game> {
        let game_ids = self.devs.get(name).unwrap();
        let mut games: Vec<&Game> = Vec::new();
        for id in game_ids {
            games.push(self.games.get(id).unwrap());
        }
        QuerySet::new(games)
    }
    pub fn get_game_by_publi(&self, name: &str) -> QuerySet<&Game> {
        let game_ids = self.publis.get(name).unwrap();
        let mut games: Vec<&Game> = Vec::new();
        for id in game_ids {
            games.push(self.games.get(id).unwrap());
        }
        QuerySet::new(games)
    }
    pub fn game_contains(
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
}
