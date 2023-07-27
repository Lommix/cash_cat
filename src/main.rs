#![allow(unused)]
use clap::Parser;
use models::prelude::*;

mod models;

#[derive(Parser)]
#[command(name = "cash_cat")]
#[command(author = "Lorenz Mielke")]
#[command(version = "1.0")]
#[command(about = "A simple cli time tracker", long_about = None)]
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
    duration: i64,
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
        Cli::Track(args) => {
            let customer = Customer::find_by_name(&con, &args.customer).unwrap_or({
                Customer::insert(
                    &Customer {
                        id: None,
                        name: args.customer,
                    },
                    &con,
                )
                .expect("Customer not insertable")
            });

            let ticket = Ticket::find_by_name(&con, &args.ticket).unwrap_or({
                Ticket::insert(
                    &Ticket {
                        id: None,
                        customer_id: customer.id.unwrap(),
                        description: String::default(),
                        name: args.ticket,
                    },
                    &con,
                )
                .expect("Ticket not insertable")
            });

            let track = TimeEntry::new(customer.id.unwrap(), ticket.id.unwrap(), args.duration);
            track.insert(&con).expect("TimeEntry not insertable");
        }
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
