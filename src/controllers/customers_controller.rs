use rusqlite::{Connection, Result};
use crate::models::customers::RCustomer;

pub fn get_customers_limit(conn: &Connection, limit: u32) -> Result<(), Box<dyn std::error::Error>> {
    let customers: Vec<RCustomer> = RCustomer::get_limited(&conn, 50)?;

    for customer in customers {
        println!("{}", customer.name);
    }
    println!("Got customers!");
    Ok(())
}
