#![allow(unused)]
use chrono::prelude::*;
use clap::Parser;
use models::prelude::*;
use std::io::{self, Write};

mod models;

#[derive(Parser)]
#[command(name = "cash_cat")]
#[command(author = "Lorenz Mielke")]
#[command(version = "1.0")]
#[command(about = "A simple cli time tracker", long_about = None)]
enum Cli {
    Track(TrackArgs),
    Export(ExportArgs),
    List(ListArgs),
    Init,
}

#[derive(clap::Args, Debug)]
struct TrackArgs {
    #[arg(short, long)]
    customer: String,
    #[arg(short, long)]
    ticket: String,
    #[arg(short, long)]
    duration: u64,
}

#[derive(clap::Args, Debug)]
struct ExportArgs {
    #[arg(short, long)]
    customer: String,
    #[arg(short, long)]
    month: Option<String>,
}

#[derive(clap::Args, Debug)]
struct ListArgs {
    #[arg(short, long)]
    customer: String,
    #[arg(short, long)]
    months: u64,
}

// -------------------------------------
fn main() {
    let cli = Cli::parse();
    let con = Store::open_connection();
    match cli {
        Cli::Track(args) => track(&con, args),
        Cli::List(args) => list(&con, args),
        Cli::Export(args) => {
            let c = Customer::find_by_name(&con, &args.customer).unwrap();
            dbg!(c);
        }
        Cli::Init => {
            Store::down(&con);
            Store::up(&con);
        }
    }
}

// -------------------------------------
fn list(con: &rusqlite::Connection, args: ListArgs) {
    let (min, max) = get_month_range(args.months);
    let customer = Customer::find_by_name(&con, &args.customer).expect("Customer not found");

    let mut stmt = con
        .prepare(
            "SELECT * FROM entries WHERE customer_id = ? AND created_at >= ? AND created_at <= ?",
        )
        .unwrap();
    let tickets = stmt
        .query_map([customer.id.unwrap(), min, max], |row| {
            Ok(TimeEntry {
                id: Some(row.get(0)?),
                customer_id: row.get(1)?,
                ticket_id: row.get(2)?,
                minutes: row.get(3)?,
                created_at: row.get(4)?,
            })
        })
        .unwrap();

    for ticket in tickets {
        println!("{}", ticket.unwrap());
    }
}

// -------------------------------------
fn track(con: &rusqlite::Connection, args: TrackArgs) {
    let customer = Customer::find_by_name(&con, &args.customer).unwrap_or_else(|_| {
        Customer::insert(
            &Customer {
                id: None,
                name: args.customer,
            },
            &con,
        )
        .expect("Customer not insertable")
    });

    let ticket = Ticket::find_by_name(&con, &args.ticket).unwrap_or_else(|_| {
        print!("{} Add description >> ", &args.ticket);
        std::io::stdout().flush();
        let mut description = String::new();
        std::io::stdin().read_line(&mut description).unwrap();
        Ticket::insert(
            &Ticket {
                id: None,
                customer_id: customer.id.unwrap(),
                description,
                name: args.ticket,
            },
            &con,
        )
        .expect("Ticket not insertable")
    });

    let track = TimeEntry::new(customer.id.unwrap(), ticket.id.unwrap(), args.duration);
    track.insert(&con).expect("TimeEntry not insertable");
}

// -------------------------------------
fn get_month_range(month_offset: u64) -> (i64, i64) {
    let now = chrono::offset::Local::now()
        .naive_local()
        .checked_sub_months(chrono::Months::new(month_offset as u32))
        .unwrap();

    let month = now.month();
    let year = now.year();

    let month_start = NaiveDate::from_ymd(year, month, 1)
        .and_time(NaiveTime::default())
        .timestamp();

    let month_end = NaiveDate::from_ymd(year, month + 1, 1)
        .and_time(NaiveTime::default())
        .timestamp();

    (month_start, month_end)
}
