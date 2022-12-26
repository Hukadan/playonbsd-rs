use crate::collections::DataBase;
use crate::models::{Field, Game, Item};
use crate::utils::database_builder::Cursor;
use crate::utils::get_app_id;
use chrono::NaiveDate;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub fn game_dispatch<'a>(
    field: Field,
    database: &mut DataBase,
    // expand the cover to complete url
    expand_cover: bool,
    // fetch steam cover if possible
    steam_cover: bool,
    cursor: &'a mut Cursor,
) {
    match field {
        Field::Game(name) => {
            if let Some(name) = name {
                cursor.counter += 1;
                let mut game = Game::default();
                let mut hasher = DefaultHasher::new();
                name.hash(&mut hasher);
                cursor.uuid = hasher.finish();
                game.name = name.to_string();
                game.id = cursor.counter;
                game.uuid = cursor.uuid;
                database.games.insert(cursor.uuid, game);
            } else {
                eprintln!(
                    "Error trying to insert game with id: {}.",
                    database.games.len() + 1
                );
            };
        }
        Field::Cover(name) => {
            if let Some(name) = name {
                if let Some(game) = database.games.get_mut(&cursor.uuid) {
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
        }
        Field::Engine(name) => {
            if let Some(name) = name {
                if let Some(game) = database.games.get_mut(&cursor.uuid) {
                    game.engine = Some(name.to_string());
                };
                database
                    .engines
                    .entry(name.to_string())
                    .and_modify(|e| e.games.push(cursor.uuid))
                    .or_insert(Item {
                        name: name.to_string(),
                        games: vec![cursor.uuid],
                    });
            };
        }
        Field::Setup(name) => {
            if let Some(name) = name {
                if let Some(game) = database.games.get_mut(&cursor.uuid) {
                    game.setup = Some(name.to_string());
                };
            };
        }
        Field::Runtime(name) => {
            if let Some(name) = name {
                if let Some(game) = database.games.get_mut(&cursor.uuid) {
                    game.runtime = Some(name.to_string());
                };
                database
                    .runtimes
                    .entry(name.to_string())
                    .and_modify(|e| e.games.push(cursor.uuid))
                    .or_insert(Item {
                        name: name.to_string(),
                        games: vec![cursor.uuid],
                    });
            };
        }
        Field::Hints(name) => {
            if let Some(name) = name {
                if let Some(game) = database.games.get_mut(&cursor.uuid) {
                    game.hints = Some(name.to_string());
                };
            }
        }
        Field::Dev(name) => {
            if let Some(name) = name {
                if let Some(game) = database.games.get_mut(&cursor.uuid) {
                    game.dev = Some(name.to_string());
                };
                database
                    .devs
                    .entry(name.to_string())
                    .and_modify(|e| e.games.push(cursor.uuid))
                    .or_insert(Item {
                        name: name.to_string(),
                        games: vec![cursor.uuid],
                    });
            };
        }
        Field::Publi(name) => {
            if let Some(name) = name {
                if let Some(game) = database.games.get_mut(&cursor.uuid) {
                    game.publi = Some(name.to_string());
                };
                database
                    .publis
                    .entry(name.to_string())
                    .and_modify(|e| e.games.push(cursor.uuid))
                    .or_insert(Item {
                        name: name.to_string(),
                        games: vec![cursor.uuid],
                    });
            };
        }
        Field::Version(name) => {
            if let Some(name) = name {
                if let Some(game) = database.games.get_mut(&cursor.uuid) {
                    game.version = Some(name.to_string());
                };
            };
        }
        Field::Status(name) => {
            if let Some(name) = name {
                if let Some(game) = database.games.get_mut(&cursor.uuid) {
                    game.status = Some(name.to_string());
                };
            };
        }
        Field::Store(items) => {
            if let Some(items) = items {
                if let Some(game) = database.games.get_mut(&cursor.uuid) {
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
        }
        Field::Genres(items) => {
            if let Some(items) = items {
                if let Some(game) = database.games.get_mut(&cursor.uuid) {
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
                        .and_modify(|e| e.games.push(cursor.uuid))
                        .or_insert(Item {
                            name: item.to_string(),
                            games: vec![cursor.uuid],
                        });
                }
            };
        }
        Field::Tags(items) => {
            if let Some(items) = items {
                if let Some(game) = database.games.get_mut(&cursor.uuid) {
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
                        .and_modify(|e| e.games.push(cursor.uuid))
                        .or_insert(Item {
                            name: item.to_string(),
                            games: vec![cursor.uuid],
                        });
                }
            };
        }
        Field::Year(year) => {
            if let Some(year) = year {
                if let Some(game) = database.games.get_mut(&cursor.uuid) {
                    game.year = Some(year.to_string());
                };
                database
                    .years
                    .entry(year.to_string())
                    .and_modify(|e| e.games.push(cursor.uuid))
                    .or_insert(Item {
                        name: year.to_string(),
                        games: vec![cursor.uuid],
                    });
            }
        }
        Field::Added(date) => {
            if let Some(date) = date {
                if let Some(game) = database.games.get_mut(&cursor.uuid) {
                    game.added = Some(
                        NaiveDate::parse_from_str(date, "%F").expect("fail to convert to date"),
                    );
                };
            }
        }
        Field::Updated(date) => {
            if let Some(date) = date {
                if let Some(game) = database.games.get_mut(&cursor.uuid) {
                    game.updated = Some(
                        NaiveDate::parse_from_str(date, "%F").expect("fail to convert to date"),
                    );
                };
            } else if let Some(game) = database.games.get_mut(&cursor.uuid) {
                game.updated = game.added;
            }
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
        let mut cursor = Cursor::new();
        let mut db = DataBase::default();
        let fd = Field::Game(Some("test"));
        game_dispatch(fd, &mut db, true, true, &mut cursor);
        assert_eq!(db.games.len(), 1);
        assert_eq!(db.games.get(&cursor.uuid).unwrap().name, "test".to_string());
    }
    #[test]
    fn dispatch_cover() {
        let mut cursor = Cursor::new();
        let mut db = DataBase::default();
        let fd = Field::Game(Some("test"));
        let co = Field::Cover(Some("cover"));
        game_dispatch(fd, &mut db, true, true, &mut cursor);
        game_dispatch(co, &mut db, true, true, &mut cursor);
        assert_eq!(db.games.len(), 1);
        assert_eq!(
            db.games.get(&cursor.uuid).unwrap().cover.as_ref().unwrap(),
            &"https://playonbsd.com/legacy/shopping_guide/pics/originals/cover".to_string()
        );
    }
    #[test]
    fn dispatch_engine() {
        let mut cursor = Cursor::new();
        let mut db = DataBase::default();
        let fd1 = Field::Game(Some("test1"));
        let fd2 = Field::Engine(Some("test2"));
        game_dispatch(fd1, &mut db, true, true, &mut cursor);
        game_dispatch(fd2, &mut db, true, true, &mut cursor);
        assert_eq!(db.games.len(), 1);
        assert_eq!(
            db.games.get(&cursor.uuid).unwrap().engine.as_ref().unwrap(),
            &"test2".to_string()
        );
        assert_eq!(db.engines.len(), 1);
        assert_eq!(
            db.engines.get("test2").unwrap(),
            &Item {
                name: "test2".to_string(),
                games: vec![cursor.uuid]
            }
        );
    }
    #[test]
    fn dispatch_setup() {
        let mut cursor = Cursor::new();
        let mut db = DataBase::default();
        let fd1 = Field::Game(Some("test1"));
        let fd2 = Field::Setup(Some("test2"));
        game_dispatch(fd1, &mut db, true, true, &mut cursor);
        game_dispatch(fd2, &mut db, true, true, &mut cursor);
        assert_eq!(db.games.len(), 1);
        assert_eq!(
            db.games.get(&cursor.uuid).unwrap().setup.as_ref().unwrap(),
            &"test2".to_string()
        );
    }
    #[test]
    fn dispatch_runtime() {
        let mut cursor = Cursor::new();
        let mut db = DataBase::default();
        let fd1 = Field::Game(Some("test1"));
        let fd2 = Field::Runtime(Some("test2"));
        game_dispatch(fd1, &mut db, true, true, &mut cursor);
        game_dispatch(fd2, &mut db, true, true, &mut cursor);
        assert_eq!(db.games.len(), 1);
        assert_eq!(
            db.games
                .get(&cursor.uuid)
                .unwrap()
                .runtime
                .as_ref()
                .unwrap(),
            &"test2".to_string()
        );
        assert_eq!(db.runtimes.len(), 1);
        assert_eq!(
            db.runtimes.get("test2").unwrap(),
            &Item {
                name: "test2".to_string(),
                games: vec![cursor.uuid]
            }
        );
    }
}
