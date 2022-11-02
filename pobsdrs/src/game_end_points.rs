use rocket::State;

use pobsdlib::collections::DataBase;
use pobsdlib::models::GameFilter;

#[get("/games")]
pub(crate) fn game_all(db: &State<DataBase>) -> String {
    serde_json::to_string_pretty(&db.get_all_games()).unwrap()
}

#[get("/games/<id>")]
pub(crate) fn game_id(db: &State<DataBase>, id: usize) -> String {
    serde_json::to_string_pretty(&db.get_game_by_id(id).expect("Should not fail")).unwrap()
}

#[get("/games/andsearch?<name>&<engine>&<runtime>&<genre>&<tag>&<year>&<dev>&<publi>")]
pub(crate) fn game_search_and(
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
    let mut filter = GameFilter::new();
    if let Some(name) = name {
        filter.name_contains(name);
    }
    if let Some(engine) = engine {
        filter.engine_contains(engine);
    }
    if let Some(runtime) = runtime {
        filter.runtime_contains(runtime);
    }
    if let Some(genre) = genre {
        filter.genre_contains(genre);
    }
    if let Some(tag) = tag {
        filter.tag_contains(tag);
    }
    if let Some(year) = year {
        filter.year_contains(year);
    }
    if let Some(dev) = dev {
        filter.dev_contains(dev);
    }
    if let Some(publi) = publi {
        filter.publi_contains(publi);
    }
    return serde_json::to_string_pretty(
        &db.game_contains_and(filter)
    )
    .unwrap();
}

#[get("/games/orsearch?<name>&<engine>&<runtime>&<genre>&<tag>&<year>&<dev>&<publi>")]
pub(crate) fn game_search_or(
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
    let mut filter = GameFilter::new();
    if let Some(name) = name {
        filter.name_contains(name);
    }
    if let Some(engine) = engine {
        filter.engine_contains(engine);
    }
    if let Some(runtime) = runtime {
        filter.runtime_contains(runtime);
    }
    if let Some(genre) = genre {
        filter.genre_contains(genre);
    }
    if let Some(tag) = tag {
        filter.tag_contains(tag);
    }
    if let Some(year) = year {
        filter.year_contains(year);
    }
    if let Some(dev) = dev {
        filter.dev_contains(dev);
    }
    if let Some(publi) = publi {
        filter.publi_contains(publi);
    }
    return serde_json::to_string_pretty(
        &db.game_contains_or(filter),
    )
    .unwrap();
}
