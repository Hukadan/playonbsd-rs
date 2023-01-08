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
            field_dispatch![newgame name, database, cursor];
        }
        Field::Cover(name) => {
            field_dispatch![cover, name, database, cursor];
        }
        Field::Engine(name) => {
            field_dispatch![engine in engines, name, database, cursor];
        }
        Field::Setup(name) => {
            field_dispatch![setup, name, database, cursor];
        }
        Field::Runtime(name) => {
            field_dispatch![runtime in runtimes, name, database, cursor];
        }
        Field::Hints(name) => {
            field_dispatch![hints, name, database, cursor];
        }
        Field::Dev(name) => {
            field_dispatch![dev in devs, name, database, cursor];
        }
        Field::Publi(name) => {
            field_dispatch![publi in publis, name, database, cursor];
        }
        Field::Version(name) => {
            field_dispatch![version, name, database, cursor];
        }
        Field::Status(name) => {
            field_dispatch![status, name, database, cursor];
        }
        Field::Store(items) => {
            field_dispatch![stores, with_items items, database, cursor];
        }
        Field::Genres(items) => {
            field_dispatch![genres in genres, with_items items, database, cursor];
        }
        Field::Tags(items) => {
            field_dispatch![tags in tags, with_items items, database, cursor];
        }
        Field::Year(year) => {
            field_dispatch![year in years, year, database, cursor];
        }
        Field::Added(date) => {
            field_dispatch![added, is_date date, database, cursor];
        }
        Field::Updated(date) => {
            field_dispatch![updated, is_date date, database, cursor];
        }
        Field::Unknown(left, right) => {
            field_dispatch![unknown left, right];
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
