use crate::traits::BasicItem;

#[derive(Serialize, Clone, Default, Debug, PartialEq)]
pub struct Item {
    pub id: usize,
    pub name: String,
}

impl BasicItem for Item {
    /// Is equivalent to Game::Default().
    fn new() -> Self {
        Self::default()
    }
    /// Returns the name of the item
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_id(&self) -> usize {
        self.id
    }
    /// Sets the id of the game.
    fn set_id(&mut self, id: usize) {
        self.id = id;
    }
}
