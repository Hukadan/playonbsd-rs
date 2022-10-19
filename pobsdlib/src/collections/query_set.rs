use crate::traits::BasicItem;

#[derive(Serialize, Clone, Default, Debug)]
pub struct QuerySet<T> {
    pub count: usize,
    pub items: Vec<T>,
}

impl<T: BasicItem + PartialOrd> QuerySet<T> {
    pub fn new(mut items: Vec<T>) -> Self {
        items.sort_by(|a, b| a.partial_cmp(&b).unwrap());
        Self {
            count: items.len(),
            items,
        }
    }
    /// Returns a refrence the item corresponding to the id if it exists, None otherwise.
    pub fn get_by_id(&self, id: usize) -> Option<&T> {
        match self.items.get(id - 1) {
            Some(item) => Some(item),
            None => None,
        }
    }
    /// Returns a reference the item corresponding to the name if it exists, None otherwise.
    pub fn get_by_name(&self, name: &str) -> Option<&T> {
        // assumre there is only one element with a given name
        match self.items.iter().find(|&item| item.get_name() == name) {
            Some(item) => Some(item),
            None => None,
        }
    }
}
