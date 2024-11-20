
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;
mod models;

use models::customers::R_Customer;
use rusqlite::Connection;

slint::include_modules!();


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = AppWindow::new()?;
    let customers_page = CustomersPage::get(&app);

    let conn = Connection::open("OZON_DB.sqlite")?;
    let customers: Vec<R_Customer> = R_Customer::get_limited(&conn, 50)?;
    println!("Fetched clients: {:?}", customers);

    let customer_list = customers
        .into_iter()
        .map(|c| Customer {
            uid: c.uid.into(),
            name: c.name.into(),
            phone: c.phone.into(),
            address: c.address.into(),
            address_additional: c.address_additional.unwrap_or_default().into(),
            active: c.active,
        })
        .collect::<Vec<_>>();

    app.set_customers(slint::ModelRc::new(slint::VecModel::from(customer_list)));


    app.on_menu_clicked(|page_name| {
        println!("Menu clicked: {}", page_name);
    });

    app.run()?;
    Ok(())
}
