// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;
mod models;
mod database;

use models::customers::Customer as RustCustomer;
use rusqlite::Connection;

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let app = AppWindow::new()?;
    
    // Fetching and passing customers
    let conn = Connection::open("OZON_DB.sqlite")?;
    let customers: Vec<RustCustomer> = RustCustomer::get_limited(&conn, 50)?;
    println!("Fetched clients: {:?}", customers);

    app.global::<CustomersPage>().set_customers(
        customers.iter().map(|c| CustomersPage::Customer {
            uid: c.uid.clone().into(),
            name: c.name.clone().into(),
            phone: c.phone.clone().into(),
            address: c.address.clone().into(),
            address_additional: c.address_additional.clone().unwrap_or_default().into(),
            active: c.active,
        }).collect::<Vec<_>>(),
    );


    // Handle the menu_clicked callback
    app.on_menu_clicked(|page_name| {
        println!("Menu clicked: {}", page_name);
    });

    app.run()?;

    Ok(())
}

