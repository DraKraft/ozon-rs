use clap::{Parser, Subcommand};
use rusqlite::Connection;

use std::error::Error;
use crate::controllers::customers_controller;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    cmd: Commands
}

use crate::models::customers::RCustomer;

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    Articles,
    Customers
}

pub fn process_args(args: &Args) -> Result<(), Box<dyn Error>> {
    // Connect to the SQLite DB
    let conn = Connection::open("OZON_DB.sqlite")?;

    match args.cmd {
        Commands::Customers => customers_controller::get_customers_limit(&conn, 10),
        Commands::Articles => {
            print!("Articles ...");
            Ok(())
        }
    }
}
