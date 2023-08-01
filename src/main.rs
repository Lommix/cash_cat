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
#[command(about = "A simple cli time tracker using sqlite", long_about = None)]
enum Cli {
    Track(TrackArgs),
    Export(ExportArgs),
    List(ListArgs),
    Delete(DeleteArgs),
    Init,
    Backup(BackupArgs),
}

#[derive(clap::Args, Debug)]
struct TrackArgs {
    customer: String,
    ticket: String,
    duration: u64,
}

#[derive(clap::Args, Debug)]
pub struct ExportArgs {
    customer: String,
    months: u64,
    destination: Option<String>,
}

#[derive(clap::Args, Debug)]
pub struct BackupArgs {
    destination: String,
}

#[derive(clap::Args, Debug)]
struct ListArgs {
    customer: String,
    months: u64,
}

#[derive(clap::Args, Debug)]
struct DeleteArgs {
    id: String,
}

// -------------------------------------
#[derive(serde::Serialize, Debug)]
pub struct ExportTemplate {
    pub customer: String,
    pub month: Option<String>,
    pub positions: Vec<Position>,
}

#[derive(serde::Serialize, Debug)]
pub struct Position {
    pub name: String,
    pub description: String,
    pub duration: u64,
}

// -------------------------------------
fn main() {
    let cli = Cli::parse();
    let con = Store::open_connection();
    match cli {
        Cli::Track(args) => track(&con, args),
        Cli::List(args) => list(&con, args),
        Cli::Export(args) => export(&con, args),
        Cli::Init => {
            Store::down(&con);
            Store::up(&con);
        }
        Cli::Delete(args) => delete(&con, args),
        Cli::Backup(args) => {
            Store::copy_database(args.destination.into()).expect("Failed to create backup")
        }
    }
}

fn delete(con: &rusqlite::Connection, args: DeleteArgs) {
    con.execute("DELETE FROM entries WHERE id = ?", [args.id])
        .expect("Failed to delete");
}

fn export(con: &rusqlite::Connection, args: ExportArgs) {
    let customer = Customer::find_by_name(&con, &args.customer).expect("Customer not found");
    let (min, max) = month_range_from_now(args.months);
    let sql = "SELECT t.name, t.description, SUM(e.minutes)
               FROM tickets as t JOIN entries as e ON t.id = e.ticket_id
               WHERE t.customer_id = ? AND e.created_at >= ? AND e.created_at <= ?;";
    let mut stmt = con.prepare(sql).unwrap();
    let entries = stmt
        .query_map([customer.id.unwrap(), min, max], |row| {
            Ok(Position {
                name: row.get(0)?,
                description: row.get(1)?,
                duration: row.get(2)?,
            })
        })
        .unwrap();

    let template = ExportTemplate {
        customer: args.customer,
        month: None,
        positions: entries.map(|t| t.unwrap()).collect(),
    };

    let json = serde_json::to_string(&template).unwrap();
    std::fs::write(args.destination.unwrap_or("./export.json".into()), json)
        .expect("Failed to write");
}

// -------------------------------------
fn list(con: &rusqlite::Connection, args: ListArgs) {
    let (min, max) = month_range_from_now(args.months);
    let customer = Customer::find_by_name(&con, &args.customer).expect("Customer not found");

    let mut stmt = con
        .prepare(
            "SELECT * FROM entries WHERE customer_id = ? AND created_at >= ? AND created_at <= ?",
        )
        .unwrap();
    let tickets = stmt
        .query_map([customer.id.unwrap(), min, max], |row| {
            Ok(TimeEntry {
                id: row.get(0)?,
                customer_id: row.get(1)?,
                ticket_id: row.get(2)?,
                minutes: row.get(3)?,
                created_at: row.get(4)?,
            })
        })
        .unwrap()
        .collect::<Vec<_>>();

    if tickets.is_empty() {
        println!("No entries found");
        return;
    }

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
fn month_range_from_now(offset: u64) -> (i64, i64) {
    let now = chrono::offset::Local::now()
        .naive_local()
        .checked_sub_months(chrono::Months::new(offset as u32))
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
