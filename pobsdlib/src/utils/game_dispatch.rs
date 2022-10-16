use crate::collections::DataBase;
use crate::models::{Field, Game};

pub fn game_dispatch(field: Field, database: &mut DataBase) {
    match field {
        Field::Game(name) => {
            let mut game = Game::default();
            let game_id = database.games.len() + 1;
            game.name = name.to_string();
            game.id = game_id;
            database.games.insert(game_id, game);
        }
        Field::Cover(name) => {
            let last_game_id = database.games.len();
            if let Some(game) = database.games.get_mut(&last_game_id) {
                if name.len() > 0 {
                    game.cover = format!("https://playonbsd.com/legacy/shopping_guide/pics/originals/{}", name.to_string());
                }
            };
        }
        Field::Engine(name) => {
            let last_game_id = database.games.len();
            if let Some(game) = database.games.get_mut(&last_game_id) {
                game.engine = name.to_string();
            };
            database
                .engines
                .entry(name.to_string())
                .and_modify(|e| e.push(last_game_id))
                .or_insert(vec![last_game_id]);
        }
        Field::Setup(name) => {
            let last_game_id = database.games.len();
            if let Some(game) = database.games.get_mut(&last_game_id) {
                game.setup = name.to_string();
            };
        }
        Field::Runtime(name) => {
            let last_game_id = database.games.len();
            if let Some(game) = database.games.get_mut(&last_game_id) {
                game.runtime = name.to_string();
            };
            database
                .runtimes
                .entry(name.to_string())
                .and_modify(|e| e.push(last_game_id))
                .or_insert(vec![last_game_id]);
        }
        Field::Hints(name) => {
            let last_game_id = database.games.len();
            if let Some(game) = database.games.get_mut(&last_game_id) {
                game.hints = name.to_string();
            };
        }
        Field::Dev(name) => {
            let last_game_id = database.games.len();
            if let Some(game) = database.games.get_mut(&last_game_id) {
                game.publi = name.to_string();
            };
            database
                .devs
                .entry(name.to_string())
                .and_modify(|e| e.push(last_game_id))
                .or_insert(vec![last_game_id]);
        }
        Field::Publi(name) => {
            let last_game_id = database.games.len();
            if let Some(game) = database.games.get_mut(&last_game_id) {
                game.dev = name.to_string();
            };
            database
                .publis
                .entry(name.to_string())
                .and_modify(|e| e.push(last_game_id))
                .or_insert(vec![last_game_id]);
        }
        Field::Version(name) => {
            let last_game_id = database.games.len();
            if let Some(game) = database.games.get_mut(&last_game_id) {
                game.version = name.to_string();
            };
        }
        Field::Status(name) => {
            let last_game_id = database.games.len();
            if let Some(game) = database.games.get_mut(&last_game_id) {
                game.status = name.to_string();
            };
        }
        Field::Store(items) => {
            let last_game_id = database.games.len();
            if let Some(game) = database.games.get_mut(&last_game_id) {
                let is_empty = game.cover.is_empty();
                for item in items {
                    if is_empty && item.contains("steampowered") {
                        let item = item.clone();
                        let app_id = item
                            .split("app/")
                            .collect::<Vec<&str>>()[1]
                            .split("/")
                            .collect::<Vec<&str>>()[0];
                        game.cover = format!("https://cdn.akamai.steamstatic.com/steam/apps/{}/header.jpg", app_id);
                    }
                    game.store.push(item.to_string());
                }
            };
        }
        Field::Genres(items) => {
            let last_game_id = database.games.len();
            if let Some(game) = database.games.get_mut(&last_game_id) {
                for item in &items {
                    if item.len() > 0 {
                        game.genres.push(item.to_string());
                    }
                }
            };
            for item in &items {
                database
                    .genres
                    .entry(item.to_string())
                    .and_modify(|e| e.push(last_game_id))
                    .or_insert(vec![last_game_id]);
            }
        }
        Field::Tags(items) => {
            let last_game_id = database.games.len();
            if let Some(game) = database.games.get_mut(&last_game_id) {
                for item in &items {
                    game.tags.push(item.to_string());
                }
            };
            for item in &items {
                database
                    .tags
                    .entry(item.to_string())
                    .and_modify(|e| e.push(last_game_id))
                    .or_insert(vec![last_game_id]);
            }
        }
        Field::Year(year) => {
            let last_game_id = database.games.len();
            if let Some(game) = database.games.get_mut(&last_game_id) {
                game.year = year.to_string();
            };
            database
                .years
                .entry(year.to_string())
                .and_modify(|e| e.push(last_game_id))
                .or_insert(vec![last_game_id]);
        }
        Field::Unknown(field) => {
            eprintln!("Skipping unknown field: {}", field);
        }
    };
}

#[cfg(test)]
mod test_game_dispatch {
    use super::*;
    use crate::collections::DataBase;
    use crate::models::Field;
    #[test]
    fn dispatch_game() {
        let mut db = DataBase::default();
        let fd = Field::Game("test");
        game_dispatch(fd, &mut db);
        assert_eq!(db.games.len(), 1);
        assert_eq!(db.games.get(&1).unwrap().name, "test".to_string());
    }
    #[test]
    fn dispatch_cover() {
        let mut db = DataBase::default();
        let fd = Field::Game("test");
        let co = Field::Cover("cover");
        game_dispatch(fd, &mut db);
        game_dispatch(co, &mut db);
        assert_eq!(db.games.len(), 1);
        assert_eq!(db.games.get(&1).unwrap().cover, "cover".to_string());
    }
    #[test]
    fn dispatch_engine() {
        let mut db = DataBase::default();
        let fd1 = Field::Game("test1");
        let fd2 = Field::Engine("test2");
        game_dispatch(fd1, &mut db);
        game_dispatch(fd2, &mut db);
        assert_eq!(db.games.len(), 1);
        assert_eq!(db.games.get(&1).unwrap().engine, "test2".to_string());
        assert_eq!(db.engines.len(), 1);
        assert_eq!(db.engines.get("test2").unwrap(), &vec![1 as usize]);
    }
    #[test]
    fn dispatch_setup() {
        let mut db = DataBase::default();
        let fd1 = Field::Game("test1");
        let fd2 = Field::Setup("test2");
        game_dispatch(fd1, &mut db);
        game_dispatch(fd2, &mut db);
        assert_eq!(db.games.len(), 1);
        assert_eq!(db.games.get(&1).unwrap().setup, "test2".to_string());
    }
    #[test]
    fn dispatch_runtime() {
        let mut db = DataBase::default();
        let fd1 = Field::Game("test1");
        let fd2 = Field::Runtime("test2");
        game_dispatch(fd1, &mut db);
        game_dispatch(fd2, &mut db);
        assert_eq!(db.games.len(), 1);
        assert_eq!(db.games.get(&1).unwrap().runtime, "test2".to_string());
        assert_eq!(db.runtimes.len(), 1);
        assert_eq!(db.runtimes.get("test2").unwrap(), &vec![1 as usize]);
    }
}
