use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Pagination {
    pub current_page: i64,
    pub previous_page: i64,
    pub next_page: i64,
    pub total_pages: i64,
    pub per_page: i64,
    pub total_records: i64,
}

impl Pagination {
    pub fn new(
        current_page: i64,
        per_page: i64,
        total_pages: i64,
        total_records: i64,
    ) -> Pagination {

        let previous_page = if current_page > 1 {
            current_page - 1
        } else {
            1
        };
        let next_page = if current_page < total_pages && total_pages > 0 {
            current_page + 1
        } else if current_page < total_pages {
            total_pages
        } else {
            current_page
        };

        Pagination {
            current_page,
            previous_page,
            next_page,
            total_pages,
            per_page,
            total_records,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Paginate {
    pub p: Pagination,
    pub offset: i64,
}

impl Paginate {
    pub fn new(
        current_page: i64,
        per_page: i64,
        per_page_limit: i64,
        total_records: i64,
    ) -> Paginate {
        let per_page_limit = if per_page_limit > 0 {
            per_page_limit
        } else {
            10
        };
        let per_page = if per_page > per_page_limit {
            per_page_limit
        } else {
            per_page
        };
        let per_page = if per_page > 0 { per_page } else { 10 };

        let total_records = if total_records > 0 { total_records } else { 0 };

        let total_pages = if total_records > 0 {
            (total_records as f64 / per_page as f64).ceil() as i64
        } else {
            0
        };
        let current_page = if current_page < 1 { 1 } else { current_page };
        let current_page = if current_page > total_pages && total_pages > 0 {
            total_pages
        } else {
            current_page
        };

        let limit = per_page;
        let offset = (limit * current_page) - limit;

        let p = Pagination::new(current_page, per_page, total_pages, total_records);

        Paginate { p, offset}
    }
}