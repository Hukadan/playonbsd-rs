use crate::views::game_list::game_list_view;
use crate::wrappers::GameFilterWrapper;
use axum::extract::{Extension, Form, Query};
use axum::response::IntoResponse;
use pobsdlib::{DataBase, Game, QueryResult};
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Deserialize, Debug)]
pub struct Search {
    pattern: String,
}

pub async fn game_list(
    Extension(db): Extension<Arc<DataBase>>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let game_filter_wrapper = GameFilterWrapper::new(&params);
    let game_query: QueryResult<Game>;
    if game_filter_wrapper.filter_on {
        game_query = db.game_contains_or(game_filter_wrapper.game_filter);
    } else {
        game_query = db.get_all_games();
    }
    let page = params.get("page");
    game_list_view(game_query, page.cloned(), game_filter_wrapper.query_str)
}

pub async fn game_list_search(
    Extension(db): Extension<Arc<DataBase>>,
    Form(search): Form<Search>,
) -> impl IntoResponse {
    let pattern = search.pattern;
    let mut params: HashMap<String, String> = HashMap::new();
    if !pattern.is_empty() {
        params.insert("name".to_string(), pattern.clone());
        params.insert("engine".to_string(), pattern.clone());
        params.insert("runtime".to_string(), pattern.clone());
        params.insert("genre".to_string(), pattern.clone());
        params.insert("tag".to_string(), pattern.clone());
        params.insert("year".to_string(), pattern.clone());
        params.insert("dev".to_string(), pattern.clone());
        params.insert("publi".to_string(), pattern.clone());
    }
    let game_filter_wrapper = GameFilterWrapper::new(&params);
    let game_query: QueryResult<Game>;
    if game_filter_wrapper.filter_on {
        game_query = db.game_contains_or(game_filter_wrapper.game_filter);
    } else {
        game_query = db.get_all_games();
    }
    game_list_view(game_query, None, game_filter_wrapper.query_str)
}
