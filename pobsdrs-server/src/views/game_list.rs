use crate::models::HtmlTemplate;
use crate::wrappers::{Page, Paginator};
use askama::Template;
use axum::response::IntoResponse;
use pobsdlib::{Game, QueryResult};

#[derive(Template)]
#[template(path = "game_list.html")]
struct GameListTemplate {
    games: Vec<Game>,
    query_str: String,
    paginator: Page,
}

pub fn game_list_view(
    game_query: QueryResult<Game>,
    page: Option<String>,
    query_str: String,
) -> impl IntoResponse {
    let page = match page {
        Some(page) => page.parse::<usize>().unwrap(),
        None => 1,
    };
    let page = Paginator::new(game_query.count, 15).page(page);
    let template: GameListTemplate = match page {
        Some(page) => GameListTemplate {
            games: game_query.items[page.first_element..=page.last_element].to_vec(),
            query_str,
            paginator: page,
        },
        None => {
            let page = Page {
                first_element: 0,
                last_element: game_query.count,
                current_page: 1,
                last_page: 1,
            };
            GameListTemplate {
                games: game_query.items,
                query_str: "".to_string(),
                paginator: page,
            }
        }
    };
    HtmlTemplate(template)
}
