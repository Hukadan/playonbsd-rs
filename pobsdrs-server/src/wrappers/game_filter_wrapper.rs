use pobsdlib::GameFilter;
use std::collections::HashMap;

pub struct GameFilterWrapper<'a> {
    pub game_filter: GameFilter<'a>,
    // true if there is something in the game filter
    pub filter_on: bool,
    pub query_str: String,
}

impl<'a> GameFilterWrapper<'a> {
    pub fn new(query: &'a HashMap<String, String>) -> Self {
        let mut game_filter = GameFilter::new();
        let mut query_str: Vec<String> = Vec::new();
        if let Some(name) = query.get("name") {
            game_filter.name_contains(name);
            query_str.push(format!("name={}", name));
        }
        if let Some(engine) = query.get("engine") {
            game_filter.engine_contains(engine);
            query_str.push(format!("engine={}", engine));
        }
        if let Some(runtime) = query.get("runtime") {
            game_filter.runtime_contains(runtime);
            query_str.push(format!("runtime={}", runtime));
        }
        if let Some(genre) = query.get("genre") {
            game_filter.genre_contains(genre);
            query_str.push(format!("genre={}", genre));
        }
        if let Some(tag) = query.get("tag") {
            game_filter.tag_contains(tag);
            query_str.push(format!("tag={}", tag));
        }
        if let Some(year) = query.get("year") {
            game_filter.year_contains(year);
            query_str.push(format!("year={}", year));
        }
        if let Some(dev) = query.get("dev") {
            game_filter.dev_contains(dev);
            query_str.push(format!("dev={}", dev));
        }
        if let Some(publi) = query.get("publi") {
            game_filter.publi_contains(publi);
            query_str.push(format!("publi={}", publi));
        }
        let filter_on = !query_str.is_empty();
        let query_str = query_str.join("&");
        Self {
            game_filter,
            query_str,
            filter_on,
        }
    }
}
