
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;
use std::error::Error;
mod cli;
use clap::{Arg, Command, Parser};
use rusqlite::Connection;

mod models;
use models::customers::RCustomer;
use models::articles::RArticle;
use cli::Args;

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    // Parse CLI arguments using clap
    //
    if env::args().len() <= 1 {
        println!("No arguments provided. Running UI.");
        run_gui();
        ( )
    } else {
        let args = cli::Args::parse(); 
        cli::process_args(&args);
    }
    Ok(())
}

fn run_gui() -> Result<(), Box<dyn Error>> {
    let app = AppWindow::new()?;
    //let customers_page = CustomersPage::get(&app);

    let conn = Connection::open("OZON_DB.sqlite")?;
    let customers: Vec<RCustomer> = RCustomer::get_limited(&conn, 50)?;
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

fn run_cli() {
    println!("Running in CLI mode...");
    // Your CLI logic can go here
}

fn get_articles() {
    println!("Printing first 5 articles");
}
