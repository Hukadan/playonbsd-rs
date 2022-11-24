use crate::collections::DataBase;
use crate::models::{Field, Game, Item};
use crate::utils::get_app_id;
use chrono::NaiveDate;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub fn game_dispatch(
    field: Field,
    database: &mut DataBase,
    // expand the cover to complete url
    expand_cover: bool,
    // fetch steam cover if possible
    steam_cover: bool,
    counter: usize,
) -> usize {
    match field {
        Field::Game(name) => {
            if let Some(name) = name {
                let mut game = Game::default();
                let game_id = counter + 1;
                game.name = name.to_string();
                game.id = game_id;
                let mut hasher = DefaultHasher::new();
                game.name.hash(&mut hasher);
                game.uiid = hasher.finish();
                database.games.insert(game_id, game);
            } else {
                eprintln!(
                    "Error trying to insert game with id: {}.",
                    database.games.len() + 1
                );
            };
            counter
        }
        Field::Cover(name) => {
            if let Some(name) = name {
                let last_game_id = database.games.len();
                if let Some(game) = database.games.get_mut(&last_game_id) {
                    if !name.is_empty() {
                        if expand_cover {
                            game.cover = Some(format!(
                                "https://playonbsd.com/legacy/shopping_guide/pics/originals/{}",
                                name
                            ));
                        } else {
                            game.cover = Some(name.to_string());
                        }
                    }
                };
            };
            counter
        }
        Field::Engine(name) => {
            if let Some(name) = name {
                let last_game_id = database.games.len();
                if let Some(game) = database.games.get_mut(&last_game_id) {
                    game.engine = Some(name.to_string());
                };
                database
                    .engines
                    .entry(name.to_string())
                    .and_modify(|e| e.games.push(last_game_id))
                    .or_insert(Item {
                        name: name.to_string(),
                        games: vec![last_game_id],
                    });
            };
            counter
        }
        Field::Setup(name) => {
            if let Some(name) = name {
                let last_game_id = database.games.len();
                if let Some(game) = database.games.get_mut(&last_game_id) {
                    game.setup = Some(name.to_string());
                };
            };
            counter
        }
        Field::Runtime(name) => {
            if let Some(name) = name {
                let last_game_id = database.games.len();
                if let Some(game) = database.games.get_mut(&last_game_id) {
                    game.runtime = Some(name.to_string());
                };
                database
                    .runtimes
                    .entry(name.to_string())
                    .and_modify(|e| e.games.push(last_game_id))
                    .or_insert(Item {
                        name: name.to_string(),
                        games: vec![last_game_id],
                    });
            };
            counter
        }
        Field::Hints(name) => {
            if let Some(name) = name {
                let last_game_id = database.games.len();
                if let Some(game) = database.games.get_mut(&last_game_id) {
                    game.hints = Some(name.to_string());
                };
            }
            counter
        }
        Field::Dev(name) => {
            if let Some(name) = name {
                let last_game_id = database.games.len();
                if let Some(game) = database.games.get_mut(&last_game_id) {
                    game.dev = Some(name.to_string());
                };
                database
                    .devs
                    .entry(name.to_string())
                    .and_modify(|e| e.games.push(last_game_id))
                    .or_insert(Item {
                        name: name.to_string(),
                        games: vec![last_game_id],
                    });
            };
            counter
        }
        Field::Publi(name) => {
            if let Some(name) = name {
                let last_game_id = database.games.len();
                if let Some(game) = database.games.get_mut(&last_game_id) {
                    game.publi = Some(name.to_string());
                };
                database
                    .publis
                    .entry(name.to_string())
                    .and_modify(|e| e.games.push(last_game_id))
                    .or_insert(Item {
                        name: name.to_string(),
                        games: vec![last_game_id],
                    });
            };
            counter
        }
        Field::Version(name) => {
            if let Some(name) = name {
                let last_game_id = database.games.len();
                if let Some(game) = database.games.get_mut(&last_game_id) {
                    game.version = Some(name.to_string());
                };
            };
            counter
        }
        Field::Status(name) => {
            if let Some(name) = name {
                let last_game_id = database.games.len();
                if let Some(game) = database.games.get_mut(&last_game_id) {
                    game.status = Some(name.to_string());
                };
            };
            counter
        }
        Field::Store(items) => {
            if let Some(items) = items {
                let last_game_id = database.games.len();
                if let Some(game) = database.games.get_mut(&last_game_id) {
                    for item in items {
                        // Tries to grap the Steam one
                        // if a steam link is given in store.
                        // if is_empty && item.contains("steampowered") {
                        if item.contains("steampowered") && steam_cover {
                            let item = item;
                            let app_id = get_app_id(item);
                            if let Some(app_id) = app_id {
                                game.cover = Some(format!(
                                    "https://cdn.akamai.steamstatic.com/steam/apps/{}/header.jpg",
                                    app_id
                                ));
                            }
                        }
                        match &mut game.stores {
                            Some(stores) => stores.push(item.to_string()),
                            None => {
                                game.stores = Some(vec![item.to_string()]);
                            }
                        }
                    }
                };
            };
            counter
        }
        Field::Genres(items) => {
            if let Some(items) = items {
                let last_game_id = database.games.len();
                if let Some(game) = database.games.get_mut(&last_game_id) {
                    for item in &items {
                        match &mut game.genres {
                            Some(genres) => genres.push(item.to_string()),
                            None => {
                                game.genres = Some(vec![item.to_string()]);
                            }
                        }
                    }
                };
                for item in &items {
                    database
                        .genres
                        .entry(item.to_string())
                        .and_modify(|e| e.games.push(last_game_id))
                        .or_insert(Item {
                            name: item.to_string(),
                            games: vec![last_game_id],
                        });
                }
            };
            counter
        }
        Field::Tags(items) => {
            if let Some(items) = items {
                let last_game_id = database.games.len();
                if let Some(game) = database.games.get_mut(&last_game_id) {
                    for item in &items {
                        match &mut game.tags {
                            Some(tags) => tags.push(item.to_string()),
                            None => {
                                game.tags = Some(vec![item.to_string()]);
                            }
                        }
                    }
                };
                for item in &items {
                    database
                        .tags
                        .entry(item.to_string())
                        .and_modify(|e| e.games.push(last_game_id))
                        .or_insert(Item {
                            name: item.to_string(),
                            games: vec![last_game_id],
                        });
                }
            };
            counter
        }
        Field::Year(year) => {
            if let Some(year) = year {
                let last_game_id = database.games.len();
                if let Some(game) = database.games.get_mut(&last_game_id) {
                    game.year = Some(year.to_string());
                };
                database
                    .years
                    .entry(year.to_string())
                    .and_modify(|e| e.games.push(last_game_id))
                    .or_insert(Item {
                        name: year.to_string(),
                        games: vec![last_game_id],
                    });
            }
            counter
        }
        Field::Added(date) => {
            if let Some(date) = date {
                let last_game_id = database.games.len();
                if let Some(game) = database.games.get_mut(&last_game_id) {
                    game.added = Some(
                        NaiveDate::parse_from_str(&date, "%F").expect("fail to convert to date"),
                    );
                };
            }
            counter
        }
        Field::Updated(date) => {
            if let Some(date) = date {
                let last_game_id = database.games.len();
                if let Some(game) = database.games.get_mut(&last_game_id) {
                    game.updated = Some(
                        NaiveDate::parse_from_str(&date, "%F").expect("fail to convert to date"),
                    );
                };
            } else {
                let last_game_id = database.games.len();
                if let Some(game) = database.games.get_mut(&last_game_id) {
                    game.updated = game.added.clone()
                };
            }
            counter
        }
        Field::Unknown(left, right) => {
            if let Some(left) = left {
                if let Some(right) = right {
                    eprintln!("Skipping unknown field: {}: {}", left, right);
                } else {
                    eprintln!("Skipping unknown field: {}", left);
                };
            } else {
                eprintln!("Skipping unknown field");
            }
            counter
        }
    }
}

#[cfg(test)]
mod test_game_dispatch {
    use super::*;
    use crate::collections::DataBase;
    use crate::models::Field;
    #[test]
    fn dispatch_game() {
        let mut db = DataBase::default();
        let fd = Field::Game(Some("test"));
        game_dispatch(fd, &mut db, true, true);
        assert_eq!(db.games.len(), 1);
        assert_eq!(db.games.get(&1).unwrap().name, "test".to_string());
    }
    #[test]
    fn dispatch_cover() {
        let mut db = DataBase::default();
        let fd = Field::Game(Some("test"));
        let co = Field::Cover(Some("cover"));
        game_dispatch(fd, &mut db, true, true);
        game_dispatch(co, &mut db, true, true);
        assert_eq!(db.games.len(), 1);
        assert_eq!(
            db.games.get(&1).unwrap().cover.as_ref().unwrap(),
            &"https://playonbsd.com/legacy/shopping_guide/pics/originals/cover".to_string()
        );
    }
    #[test]
    fn dispatch_engine() {
        let mut db = DataBase::default();
        let fd1 = Field::Game(Some("test1"));
        let fd2 = Field::Engine(Some("test2"));
        game_dispatch(fd1, &mut db, true, true);
        game_dispatch(fd2, &mut db, true, true);
        assert_eq!(db.games.len(), 1);
        assert_eq!(
            db.games.get(&1).unwrap().engine.as_ref().unwrap(),
            &"test2".to_string()
        );
        assert_eq!(db.engines.len(), 1);
        assert_eq!(
            db.engines.get("test2").unwrap(),
            &Item {
                name: "test2".to_string(),
                games: vec![1 as usize]
            }
        );
    }
    #[test]
    fn dispatch_setup() {
        let mut db = DataBase::default();
        let fd1 = Field::Game(Some("test1"));
        let fd2 = Field::Setup(Some("test2"));
        game_dispatch(fd1, &mut db, true, true);
        game_dispatch(fd2, &mut db, true, true);
        assert_eq!(db.games.len(), 1);
        assert_eq!(
            db.games.get(&1).unwrap().setup.as_ref().unwrap(),
            &"test2".to_string()
        );
    }
    #[test]
    fn dispatch_runtime() {
        let mut db = DataBase::default();
        let fd1 = Field::Game(Some("test1"));
        let fd2 = Field::Runtime(Some("test2"));
        game_dispatch(fd1, &mut db, true, true);
        game_dispatch(fd2, &mut db, true, true);
        assert_eq!(db.games.len(), 1);
        assert_eq!(
            db.games.get(&1).unwrap().runtime.as_ref().unwrap(),
            &"test2".to_string()
        );
        assert_eq!(db.runtimes.len(), 1);
        assert_eq!(
            db.runtimes.get("test2").unwrap(),
            &Item {
                name: "test2".to_string(),
                games: vec![1 as usize]
            }
        );
    }
}
