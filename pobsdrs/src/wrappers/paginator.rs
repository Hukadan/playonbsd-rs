pub struct Page {
    pub first_element: usize,
    pub last_element: usize,
    pub current_page: usize,
    pub last_page: usize,
}

pub struct Paginator {
    item_number: usize,
    item_per_page: usize,
}

impl Paginator {
    pub fn new(item_number: usize, item_per_page: usize) -> Self {
        Self {
            item_number,
            item_per_page,
        }
    }
    pub fn page(&self, page_number: usize) -> Option<Page> {
        let last_page = if self.item_number % self.item_per_page == 0 {
            self.item_number / self.item_per_page
        } else {
            self.item_number / self.item_per_page + 1
        };
        if page_number > last_page {
            None
        } else if page_number == last_page {
            let first_element = self.item_per_page * (page_number - 1);
            
            let last_element = if self.item_number % self.item_per_page == 0 {
                first_element + self.item_per_page - 1
            } else {
                first_element + (self.item_number % self.item_per_page - 1)
            };
            Some(Page {
                first_element,
                last_element,
                current_page: page_number,
                last_page,
            })
        } else {
            let first_element = self.item_per_page * (page_number - 1);
            let last_element = first_element + self.item_per_page;
            Some(Page {
                first_element,
                last_element,
                current_page: page_number,
                last_page,
            })
        }
    }
}
