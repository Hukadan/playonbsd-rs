use pobsdlib::{Game, QueryResult};
use rocket::serde::Serialize;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Paginator<'a> {
    query_result: QueryResult<&'a Game>,
    // number of the last page
    last_page: usize,
    current_page: usize,
}

impl<'a> Paginator<'a> {
    pub fn new(query_result: QueryResult<&'a Game>) -> Self {
        let count = query_result.count;
        Self {
            query_result,
            last_page: 0,
            current_page: 0,
        }
    }
    pub fn get_page(&self, pagination: usize, page: usize) -> Paginator {
        let modulo = self.query_result.count % pagination;
        // return an empty QueryResult
        if self.query_result.count == 0 {
            let items: Vec<&Game> = Vec::new();
            let qr = QueryResult::new(items);
            return Self {
                query_result: qr,
                last_page: 1,
                current_page: 0,
            };
        }
        let mut page_number: usize = self.query_result.count / pagination;
        // remaining items need a new page
        if modulo != 0 {
            page_number = page_number + 1;
        }
        let mut page = page;
        // if we go to far, go to the last page
        if page > page_number {
            page = page_number;
        }
        // if the page 0 is asked, go to the first page
        if page == 0 {
            page = 1;
        }
        let first_element = pagination * (page - 1);
        let last_element: usize;
        if self.query_result.count < pagination * page {
            last_element = self.query_result.count - 1;
        } else {
            last_element = pagination * page - 1;
        }
        // The previous checks ensure that the indices are not
        // out of bounds.
        let items = &self.query_result.items[first_element..=last_element];
        let items = items.to_vec();
        let qr = QueryResult::new(items);
        Paginator {
            query_result: qr,
            last_page: page_number,
            current_page: page,
        }
    }
}
