use reqwest;

use scraper::{Html, Selector};


#[derive(Debug, serde::Serialize)]
pub struct Links {
    pub link: String,
    pub title: String,
}

pub async fn generate_links() -> Vec<Links> {

    let index = reqwest::get(
        "http://localhost:8000/"
    ).await.unwrap().text().await.unwrap();
    let account = reqwest::get(
        "http://localhost:8000/account/users"
    ).await.unwrap().text().await.unwrap();

    let index_document = Html::parse_document(&index);
    let account_document = Html::parse_document(&account);
    let index_selector = Selector::parse("a.sitemap").unwrap();
    let account_selector = Selector::parse("a.account").unwrap();

    let mut return_data = Vec::new();

    for i in index_document.select(&index_selector) {
        return_data.push(Links {
            link: i.value().attr("href").unwrap_or("").to_string(),
            title: i.value().attr("title").unwrap_or("").to_string(),
        });
    };
    for i in account_document.select(&account_selector) {
        return_data.push(Links {
            link: i.value().attr("href").unwrap_or("").to_string(),
            title: i.value().attr("title").unwrap_or("").to_string(),
        });
    }
    return_data
}
