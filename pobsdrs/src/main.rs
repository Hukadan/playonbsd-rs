pub mod models;
pub mod routes;
pub mod wrappers;

use axum::{extract::Extension, routing::get, Router};
use reqwest;

use std::sync::Arc;

use crate::routes::{game_list, game_details};
use pobsdlib::collections::DataBase;

#[tokio::main]
async fn main() {
    let shared_db: Arc<DataBase>;
    if let Ok(req) = reqwest::get("https://raw.githubusercontent.com/playonbsd/OpenBSD-Games-Database/main/openbsd-games.db").await {
        if let Ok(content) = req.text().await {
            let db = DataBase::new_from_string(content);
            shared_db = Arc::new(db);
        } else {
            panic!("Could no fetch the database from GitHub");
        }
    } else {
        panic!("Could no fetch the database from GitHub");
    }
    
    let app = Router::new()
        .route("/", get(game_list::game_list).post(game_list::game_list_search))
        .route("/:game_id", get(game_details::game_details))
        .layer(Extension(shared_db));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
