use crate::wrappers::paginator::Paginator;
use pobsdlib::collections::DataBase;
use pobsdlib::models::GameFilter;
use rocket::State;
use rocket_dyn_templates::{context, Template};
use rocket::form::Form;

#[derive(FromForm)]
struct Search<'r> {
    pattern: &'r str,
}

#[post("/", data="<search>")]
pub fn gamesearch<'a>(
    db: &'a State<DataBase>,
    search: From<Search<'_>>,
) -> Template {
    let mut query_str: Vec<String> = Vec::new();
    let page_number = 1;
    let mut filter = GameFilter::new();
    query_str.push(format!("name={}", search.pattern));
    filter.name_contains(search.pattern);
    query_str.push(format!("engine={}", search.pattern));
    filter.engine_contains(search.pattern);
    query_str.push(format!("runtime={}", search.pattern));
    filter.runtime_contains(search.pattern);
    query_str.push(format!("genre={}", search.pattern));
    filter.genre_contains(search.pattern);
    query_str.push(format!("tag={}", search.pattern));
    filter.tag_contains(search.pattern);
    query_str.push(format!("year={}", search.pattern));
    filter.year_contains(search.pattern);
    query_str.push(format!("dev={}", search.pattern));
    filter.dev_contains(search.pattern);
    query_str.push(format!("publi={}", search.pattern));
    filter.publi_contains(search.pattern);
    let games = db.game_contains_or(filter);
    let pgames = Paginator::new(games);
    Template::render(
        "game_list",
        context! { paginator:&pgames.get_page(15,page_number), query_str: query_str.join("&")},
    )
}
