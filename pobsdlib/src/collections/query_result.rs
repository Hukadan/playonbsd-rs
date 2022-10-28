#[derive(Serialize, Clone, Default, Debug)]
/// The QueryResult struct, as its name implies,
/// represents the result of a database query.
/// It has two fields: a count for the number
/// of items returned by the query and items
/// which contains the ordered items returned
/// by the query.
pub struct QueryResult<T> {
    /// number of items returned by the query
    pub count: usize,
    /// items returned by the query
    pub items: Vec<T>,
}

impl<T: PartialOrd> QueryResult<T> {
    /// Sort the items returned by the query
    /// For games, the ordering is done based
    /// on the id of the games.
    /// For the other items, the ordering is
    /// done based on the name of the items.
    pub fn new(mut items: Vec<T>) -> Self {
        items.sort_by(|a, b| a.partial_cmp(b).unwrap());
        Self {
            count: items.len(),
            items,
        }
    }
}
