use rusqlite::{Connection, Result};

#[derive(Debug, Clone)]
pub struct R_Customer {
    pub uid: String,
    pub name: String,
    pub phone: String,
    pub address: String,
    pub address_additional: Option<String>,
    pub active: bool,
}

impl R_Customer {
    pub fn get_all(conn: &Connection) -> Result<Vec<R_Customer>> {
        let mut stmt = conn.prepare(
            "SELECT uid, name, phone, address, address_additional, active FROM clients_client",
        )?;
        let customer_iter = stmt.query_map([], |row| {
            Ok(R_Customer {
                uid: row.get(0)?,
                name: row.get(1)?,
                phone: row.get(2)?,
                address: row.get(3)?,
                address_additional: row.get(4).ok(),
                active: row.get(5)?,
            })
        })?;

        let mut customers = Vec::new();
        for customer in customer_iter {
            customers.push(customer?);
        }
        Ok(customers)
    }

    pub fn get_limited(conn: &Connection, limit: usize) -> Result<Vec<R_Customer>> {
        let mut stmt = conn.prepare("SELECT uid, name, phone, address, address_additional, active FROM clients_client LIMIT ?")?;
        let customers = stmt.query_map([limit], |row| {
            Ok(R_Customer {
                uid: row.get(0)?,
                name: row.get(1)?,
                phone: row.get(2)?,
                address: row.get(3)?,
                address_additional: row.get(4).ok(),
                active: row.get(5)?,
            })
        })?
        .collect::<Result<Vec<R_Customer>, _>>()?;

        Ok(customers)
    }
}
