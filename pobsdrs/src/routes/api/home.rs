use crate::Config;
use rocket::serde::json::Json;
use rocket::serde::Serialize;
use rocket::State;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub(crate) struct APIHome {
    games: String,
}
#[get("/")]
pub(crate) fn api_home<'a>(config: &'a State<Config>) -> Json<APIHome> {
    Json(APIHome {
        games: format!("{}/api/games", &config.domain),
    })
}
