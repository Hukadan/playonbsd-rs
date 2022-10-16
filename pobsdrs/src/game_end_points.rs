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

#[get("/games?<name>")]
pub(crate) fn game_search_by_name(
    db: &State<DataBase>,
    name: Option<String>,
) -> String {
    if let Some(name) = name {
        return serde_json::to_string_pretty(&db.get_game_by_name_contains(&name)).unwrap()
    }
    serde_json::to_string_pretty(&db.get_all_games()).unwrap()
}
