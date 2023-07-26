#![allow(unused)]
use clap::Parser;
use models::prelude::*;

mod models;

#[derive(Parser)]
#[command(name = "cash_cat")]
#[command(author = "Lorenz Mielke")]
#[command(version = "1.0")]
#[command(about = "A simple time tracker", long_about = None)]
enum Cli {
    Track(TrackArgs),
    Print(PrintArgs),
    Init,
}

#[derive(clap::Args, Debug)]
struct TrackArgs {
    #[arg(short, long)]
    customer: String,
    #[arg(short, long)]
    ticket: String,
    #[arg(short, long)]
    message: String,
    #[arg(short, long)]
    duration: String,
}

#[derive(clap::Args, Debug)]
struct PrintArgs {
    #[arg(short, long)]
    customer: String,
    #[arg(short, long)]
    month: Option<String>,
}
// -------------------------------------
fn main() {
    let cli = Cli::parse();
    let con = Store::open_connection();
    match cli {
        Cli::Track(args) => {}
        Cli::Print(args) => {
            let c = Customer::find_by_name(&con, &args.customer).unwrap();
            dbg!(c);
        }
        Cli::Init => {
            Store::down(&con);
            Store::up(&con);
        }
    }
}
