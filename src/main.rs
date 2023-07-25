#![allow(unused)]
use clap::Parser;
use store::Store;

mod store;
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
    match cli {
        Cli::Track(args) => {
            dbg!(args);
        }
        Cli::Print(args) => {
            dbg!(args);
        }
        Cli::Init => {
            Store::up();
        }
    }
}
