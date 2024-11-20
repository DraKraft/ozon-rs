use rusqlite::Connection;
use crate::models::articles::RArticle;

slint::include_modules!();

pub fn get_articles(app: &AppWindow,conn: &Connection, limit: u32) {
    let articles: Vec<RArticle> = RArticle::get_all(conn, limit);
    println!("Number of articles: ");

}
