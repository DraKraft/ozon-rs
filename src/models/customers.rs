use rusqlite::{Connection, Result};

#[derive(Debug, Clone)]
pub struct Customer {
    pub uid: String,
    pub name: String,
    pub phone: String,
    pub address: String,
    pub address_additional: Option<String>,
    pub active: bool,
}

impl Customer {
    pub fn get_all(conn: &Connection) -> Result<Vec<Customer>> {
        let mut stmt = conn.prepare(
            "SELECT uid, name, phone, address, address_additional, active FROM clients_client",
        )?;
        let customer_iter = stmt.query_map([], |row| {
            Ok(Customer {
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

    pub fn get_limited(conn: &Connection, limit: usize) -> Result<Vec<Customer>> {
        let mut stmt = conn.prepare(
            "SELECT uid, name, phone, address, address_additional, active FROM clients_client LIMIT ?",
        )?;
        let customers = stmt.query_map([limit], |row| {
            Ok(Customer {
                uid: row.get(0)?,
                name: row.get(1)?,
                phone: row.get(2)?,
                address: row.get(3)?,
                address_additional: row.get(4).ok(),
                active: row.get(5)?,
            })
        })?
        .collect::<Result<Vec<Customer>, _>>()?;

        Ok(customers)
    }
}