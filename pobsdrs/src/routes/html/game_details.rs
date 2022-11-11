use pobsdlib::{DataBase, Game};
use rocket::State;
use rocket_dyn_templates::{context, Template};

#[get("/<game_id>")]
pub fn gamedetails<'a>(db: &'a State<DataBase>, game_id: usize) -> Template {
    if let Some(game) = db.get_game_by_id(game_id) {
        Template::render("game_details", context! { game: &game })
    } else {
        Template::render("game_details", context! { game: None::<Game> })
    }
}
