use crate::wrappers::paginator::Paginator;
use crate::Config;
use pobsdlib::collections::DataBase;
use pobsdlib::models::{Game, GameFilter};
use rocket::State;
use rocket_dyn_templates::{context, Template};

#[get("/?<name>&<engine>&<runtime>&<genre>&<tag>&<year>&<dev>&<publi>&<page>")]
pub fn gamelist<'a>(
    db: &'a State<DataBase>,
    name: Option<&'a str>,
    engine: Option<&'a str>,
    runtime: Option<&'a str>,
    genre: Option<&'a str>,
    tag: Option<&'a str>,
    year: Option<&'a str>,
    dev: Option<&'a str>,
    publi: Option<&'a str>,
    page: Option<usize>,
) -> Template {
    let mut query_str: Vec<String> = Vec::new();
    let page_number = match page {
        Some(page_number) => page_number,
        None => 1,
    };
    let mut filter = GameFilter::new();
    if let Some(name) = name {
        query_str.push(format!("name={}", name));
        filter.name_contains(name);
    }
    if let Some(engine) = engine {
        query_str.push(format!("engine={}", engine));
        filter.engine_contains(engine);
    }
    if let Some(runtime) = runtime {
        query_str.push(format!("runtime={}", runtime));
        filter.runtime_contains(runtime);
    }
    if let Some(genre) = genre {
        query_str.push(format!("genre={}", genre));
        filter.genre_contains(genre);
    }
    if let Some(tag) = tag {
        query_str.push(format!("tag={}", tag));
        filter.tag_contains(tag);
    }
    if let Some(year) = year {
        query_str.push(format!("year={}", year));
        filter.year_contains(year);
    }
    if let Some(dev) = dev {
        query_str.push(format!("dev={}", dev));
        filter.dev_contains(dev);
    }
    if let Some(publi) = publi {
        query_str.push(format!("publi={}", publi));
        filter.publi_contains(publi);
    }
    if !query_str.is_empty() {
        let games = db.game_contains_or(filter);
        let pgames = Paginator::new(games);
        Template::render(
            "game_list",
            context! { paginator:&pgames.get_page(15,page_number), query_str: query_str.join("&")},
        )
    } else {
        let games = db.get_all_games();
        let pgames = Paginator::new(games);
        Template::render(
            "game_list",
            context! { paginator:&pgames.get_page(15,page_number), query_str: ""},
        )
    }
}
