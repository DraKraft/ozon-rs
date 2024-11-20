use rusqlite::{Connection, Result};
use std::sync::{Arc, Mutex};

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        Ok(Self { conn })
    }

    pub fn get_users(&self) -> Result<Vec<String>> {
        let mut stmt = self.conn.prepare("SELECT name FROM users")?;
        let user_iter = stmt.query_map([], |row| Ok(row.get::<_, String>(0)?))?;
        let mut users = Vec::new();
        for user in user_iter {
            users.push(user?);
        }
        Ok(users)
    }
}

// Type alias for shared database access
pub type SharedDatabase = Arc<Mutex<Database>>;

