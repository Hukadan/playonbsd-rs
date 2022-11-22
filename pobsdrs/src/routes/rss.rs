use askama::Template;
use axum::extract::Extension;
use axum::http::{header, HeaderMap};
use axum::response::IntoResponse;
use chrono::{prelude::*, Duration};
use pobsdlib::{DataBase, Game};
use rss::{ChannelBuilder, Item};
use std::sync::Arc;

#[derive(Template)]
#[template(path = "rss.html")]
struct RSSTemplate {
    game: Option<Game>,
}

pub async fn rss(Extension(db): Extension<Arc<DataBase>>) -> impl IntoResponse {
    let now = Local::now().naive_local().date();
    let game_query = db.get_all_games();
    let mut games: Vec<Game> = game_query
        .items
        .into_iter()
        .filter(|a| now - a.updated.unwrap() < Duration::days(90))
        .collect();
    games.sort_by(|a, b| {
        let date_a = a.updated.unwrap();
        let date_b = b.updated.unwrap();
        date_b.partial_cmp(&date_a).unwrap()
    });
    let mut items: Vec<Item> = Vec::new();
    for game in games {
        let mut item = Item::default();
        if game.added.unwrap() == game.updated.unwrap() {
            item.set_title(format!("The game {} has been added.", &game.name))
        } else {
            item.set_title(format!("The game {} has been updated.", &game.name))
        }
        item.set_pub_date(format!("{}", game.updated.unwrap().format("%F")));
        let template = RSSTemplate { game: Some(game) };
        if let Ok(content) = template.render() {
            item.set_content(content);
        }
        items.push(item);
    }
    let channel = ChannelBuilder::default()
        .title("PlayOnBSD updates")
        .link("https://playonbsd.com")
        .description("Game database updates")
        .items(items)
        .build();
    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        "application/rss+xml;charset=UTF-8".parse().unwrap(),
    );
    (headers, channel.to_string())
}
