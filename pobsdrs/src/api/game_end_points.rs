use rocket::State;
use rocket::serde::json::Json;

use pobsdlib::collections::DataBase;
use pobsdlib::models::GameFilter;

use crate::api::wrappers::{GameWrapper, QueryResultWrapper};
use crate::Config;

#[get("/games")]
pub(crate) fn game_all<'a>(db: &'a State<DataBase>, config: &'a State<Config>) -> Json<QueryResultWrapper> {
    Json(QueryResultWrapper::new(db.get_all_games(), &config.domain))
}

#[get("/games/<id>")]
pub(crate) fn game_id<'a>(db: &'a State<DataBase>, config: &'a State<Config>, id: usize) -> Json<GameWrapper<'a>> {
    Json(GameWrapper::new(&db.get_game_by_id(id).expect("Should not fail"), &config.domain))
}

#[get("/games/search?<name>&<engine>&<runtime>&<genre>&<tag>&<year>&<dev>&<publi>")]
pub(crate) fn game_search<'a>(
    db: &'a State<DataBase>,
    config: &'a State<Config>,
    name: Option<&'a str>,
    engine: Option<&'a str>,
    runtime: Option<&'a str>,
    genre: Option<&'a str>,
    tag: Option<&'a str>,
    year: Option<&'a str>,
    dev: Option<&'a str>,
    publi: Option<&'a str>,
) -> Json<QueryResultWrapper<'a>> {
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
    Json(QueryResultWrapper::new(db.game_contains_or(filter), &config.domain))
}
