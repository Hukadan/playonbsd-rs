use rocket::State;

use pobsdlib::collections::DataBase;

#[get("/games")]
pub(crate) fn game_all(db: &State<DataBase>) -> String {
    serde_json::to_string_pretty(&db.get_all_games()).unwrap()
}

#[get("/games/<id>")]
pub(crate) fn game_id(db: &State<DataBase>, id: usize) -> String {
    serde_json::to_string_pretty(&db.get_game_by_id(&id).expect("Should not fail")).unwrap()
}

#[get("/games?<name>&<engine>&<runtime>&<genre>&<tag>&<year>&<dev>&<publi>")]
pub(crate) fn game_search_by_name(
    db: &State<DataBase>,
    name: Option<&str>,
    engine: Option<&str>,
    runtime: Option<&str>,
    genre: Option<&str>,
    tag: Option<&str>,
    year: Option<&str>,
    dev: Option<&str>,
    publi: Option<&str>,
) -> String {
    return serde_json::to_string_pretty(
        &db.game_contains(name, engine, runtime, genre, tag, year, dev, publi),
    )
    .unwrap();
}
