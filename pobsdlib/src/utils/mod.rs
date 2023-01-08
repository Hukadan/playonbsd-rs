pub use crate::utils::game_dispatch::game_dispatch;
pub use crate::utils::get_appid::get_app_id;
pub use crate::utils::read_lines::read_lines;
pub use crate::utils::split_line::split_line;

#[macro_use]
mod game_dispatch_macros;
pub mod database_builder;
pub mod game_dispatch;
pub mod get_appid;
pub mod read_lines;
pub mod split_line;
