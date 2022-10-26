extern crate pobsdlib;
use pobsdlib::collections::DataBase;

#[test]
fn test_get_all_game() {
    let db = DataBase::new("tests/data/test-games.db");
    let qs = db.get_all_games();
    // we get them all
    assert_eq!(qs.count, 8);
    // we get the right ones and in the right order
    assert_eq!(
        qs.items.get(0).unwrap().name,
        "AaaaaAAaaaAAAaaAAAAaAAAAA!!! for the Awesome"
    );
    assert_eq!(qs.items.get(1).unwrap().name, "The Adventures of Shuggy");
    assert_eq!(qs.items.get(2).unwrap().name, "Aedemphia");
    assert_eq!(qs.items.get(3).unwrap().name, "Aeternum");
    assert_eq!(qs.items.get(4).unwrap().name, "Airships: Conquer the Skies");
    assert_eq!(qs.items.get(5).unwrap().name, "Akane the Kunoichi");
    assert_eq!(qs.items.get(6).unwrap().name, "Always Sometimes Monsters");
    assert_eq!(qs.items.get(7).unwrap().name, "Amazing Princess Sarah");
}
#[test]
fn test_game_get_by_id() {
    let db_game = DataBase::new("tests/data/test-games.db");
    match db_game.get_game_by_id(2) {
        Some(game) => {
            assert_eq!(game.name, "The Adventures of Shuggy".to_string());
            assert_eq!(
                game.store.as_ref().unwrap(),
                &vec![
                    "https://store.steampowered.com/app/211440/Adventures_of_Shuggy/".to_string(),
                    "https://www.gog.com/game/the_adventures_of_shuggy".to_string()
                ]
            );
        }
        None => panic!("Game not found"),
    }
}
#[test]
fn test_game_get_by_name() {
    let db_game = DataBase::new("tests/data/test-games.db");
    let qs = db_game.get_game_by_name("Akane the Kunoichi");
    // check we have the right number
    assert_eq!(qs.count, 1);
    // check we have the good one
    assert_eq!(qs.items.get(0).unwrap().name, "Akane the Kunoichi");
    assert_eq!(qs.items.get(0).unwrap().id, 6);
    assert_eq!(
        qs.items.get(0).unwrap().engine.as_ref().unwrap(),
        &"XNA".to_string()
    );
}
#[test]
fn test_game_get_by_engine() {
    let db_game = DataBase::new("tests/data/test-games.db");
    let qs = db_game.get_game_by_engine("FNA");
    // check we have the right number
    assert_eq!(qs.count, 2);
    // check we have the good ones
    assert_eq!(qs.items.get(0).unwrap().name, "The Adventures of Shuggy");
    assert_eq!(qs.items.get(0).unwrap().id, 2);
    assert_eq!(qs.items.get(1).unwrap().name, "Aeternum");
    assert_eq!(qs.items.get(1).unwrap().id, 4);
}
#[test]
fn test_game_get_by_runtime() {
    let db_game = DataBase::new("tests/data/test-games.db");
    let qs = db_game.get_game_by_runtime("easyrpg");
    // check we have the right number
    assert_eq!(qs.count, 1);
    // check we have the good ones
    assert_eq!(qs.items.get(0).unwrap().name, "Aedemphia");
    assert_eq!(qs.items.get(0).unwrap().id, 3);
}
#[test]
fn test_game_get_by_genre() {
    let db_game = DataBase::new("tests/data/test-games.db");
    let games = db_game.get_game_by_genre("RPG");
    assert_eq!(games.count, 2);
    assert_eq!(games.items.get(0).unwrap().name, "Aedemphia".to_string());
    assert_eq!(games.items.get(1).unwrap().name, "Always Sometimes Monsters".to_string());
}
#[test]
fn test_game_get_by_tag() {
    let db_game = DataBase::new("tests/data/test-games.db");
    let games = db_game.get_game_by_tag("indie");
    assert_eq!(games.count, 2);
    assert_eq!(games.items[0].name, "The Adventures of Shuggy".to_string());
    assert_eq!(games.items[1].name, "Aeternum".to_string());
}
#[test]
fn test_game_get_by_year() {
    let db_game = DataBase::new("tests/data/test-games.db");
    let games = db_game.get_game_by_year("2017");
    // check we have the right number
    assert_eq!(games.count, 1);
    // check we have the right one
    assert_eq!(games.items.get(0).unwrap().name, "Aeternum");
    assert_eq!(games.items.get(0).unwrap().id, 4);
}
