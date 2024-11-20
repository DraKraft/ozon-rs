use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    cmd: Commands
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    Articles,
    Customers
}

pub fn process_args(args: &Args) {
    match args.cmd {
        Commands::Articles => print!("Articles ..."),
        Commands::Customers => print!("Customers ...")
    }
}
