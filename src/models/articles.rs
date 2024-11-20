use rusqlite::{Connection, Result};

// articles.rs 
//
use crate::models::customers::RCustomer;

pub struct RArticle {
    pub description: String,
    pub customer: String,
    pub service: String,
    pub width: f32,
    pub height: f32,
    pub number: u8,
    pub date: String,
    pub state: bool
}

impl RArticle {
    pub fn get_all(conn: &Connection, limit: u32) -> Result<Vec<RArticle>> {
         let mut ru_query = conn.prepare("SELECT * FROM articles_aricle LIMIT ?")?;
         let article_iter = ru_query.query_map([limit], |row| {
            Ok(RArticle {
                customer: row.get(0)?,
                service: row.get(1)?,
                description: row.get(2)?,
                width: row.get(3)?,
                height: row.get(4)?,
                number: row.get(5)?,
                date: row.get(6)?,
                state: row.get(7)?,
            })
        })?;
        let mut articles = Vec::new();
        for article in article_iter {
            articles.push(article?);
        }
        Ok(articles)

    }
}
