use crate::traits::BasicItem;

pub trait GameItem: BasicItem {
    fn get_tags(&self) -> Option<&Vec<String>>;
    fn get_genres(&self) -> Option<&Vec<String>>;
}
