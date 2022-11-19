use crate::models::HtmlTemplate;
use crate::wrappers::{GameFilterWrapper, Page, Paginator};
use askama::Template;
use serde::Deserialize;
use axum::extract::{Extension, Form, Query};
use axum::response::IntoResponse;
use pobsdlib::{DataBase, Game, QueryResult};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Template)]
#[template(path = "game_list.html")]
struct GameListTemplate {
    games: Vec<Game>,
    query_str: String,
    paginator: Page,
}

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
    let page = match params.get("page") {
        Some(page) => page.parse::<usize>().unwrap(),
        None => 1,
    };
    let page = Paginator::new(game_query.count, 15).page(page);
    let template: GameListTemplate;
    match page {
        Some(page) => {
            template = GameListTemplate {
                games: game_query.items[page.first_element..=page.last_element].to_vec(),
                query_str: game_filter_wrapper.query_str,
                paginator: page,
            }
        }
        None => {
            let page = Page {
                first_element: 0,
                last_element: game_query.count,
                current_page: 1,
                last_page: 1,
            };
            template = GameListTemplate {
                games: game_query.items,
                query_str: "".to_string(),
                paginator: page,
            }
        }
    }
    HtmlTemplate(template)
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
    let page = match params.get("page") {
        Some(page) => page.parse::<usize>().unwrap(),
        None => 1,
    };
    let page = Paginator::new(game_query.count, 15).page(page);
    let template: GameListTemplate;
    match page {
        Some(page) => {
            template = GameListTemplate {
                games: game_query.items[page.first_element..=page.last_element].to_vec(),
                query_str: game_filter_wrapper.query_str,
                paginator: page,
            }
        }
        None => {
            let page = Page {
                first_element: 0,
                last_element: game_query.count,
                current_page: 1,
                last_page: 1,
            };
            template = GameListTemplate {
                games: game_query.items,
                query_str: "".to_string(),
                paginator: page,
            }
        }
    }
    HtmlTemplate(template)
}
