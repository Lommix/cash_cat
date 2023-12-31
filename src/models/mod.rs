use anyhow::Result;
pub mod customer;
pub mod entry;
pub mod ticket;

pub mod prelude {
    pub use super::customer::*;
    pub use super::entry::*;
    pub use super::ticket::*;
    pub use super::*;
}

pub trait Insertable
where
    Self: Sized,
{
    fn insert(&self, con: &rusqlite::Connection) -> Result<Self>;
}

pub trait FindableById
where
    Self: Sized,
{
    fn find_by_id(con: &rusqlite::Connection, id: i64) -> Result<Self>;
}

pub trait FindableByName
where
    Self: Sized,
{
    fn find_by_name(con: &rusqlite::Connection, name: &str) -> Result<Self>;
}

pub trait Deleteable
where
    Self: Sized,
{
    fn delete(con: &rusqlite::Connection, id: i64) -> Result<()>;
}

pub struct Store;

impl Store {
    pub fn up(con: &rusqlite::Connection) -> anyhow::Result<()> {
        con.execute(
            "CREATE TABLE IF NOT EXISTS customers (
                id INTEGER PRIMARY KEY,
                name TEXT);",
            (),
        )?;
        con.execute(
            "CREATE TABLE IF NOT EXISTS tickets(
                id INTEGER PRIMARY KEY,
                customer_id INTEGER,
                description TEXT,
                name TEXT);",
            (),
        )?;
        con.execute(
            "CREATE TABLE IF NOT EXISTS entries(
                id INTEGER PRIMARY KEY,
                customer_id INTEGER,
                ticket_id INTEGER,
                minutes INTEGER,
                created_at INTEGER);",
            (),
        )?;
        Ok(())
    }

    pub fn down(con: &rusqlite::Connection) -> anyhow::Result<()> {
        con.execute("DROP TABLE IF EXISTS customers;", ())?;
        con.execute("DROP TABLE IF EXISTS entries;", ())?;
        con.execute("DROP TABLE IF EXISTS tickets;", ())?;
        Ok(())
    }

    pub fn copy_database(dst: std::path::PathBuf) -> anyhow::Result<()> {
        let base = directories::BaseDirs::new().expect("unable to get home dir");
        let db_path = base.home_dir().join(".cash_cat/cash_cat.db");

        if !db_path.exists(){
            anyhow::bail!("Database not found. Have you even created one yet?")
        }

        std::fs::copy(db_path, dst)?;

        Ok(())
    }

    pub fn open_connection() -> rusqlite::Connection {
        let base = directories::BaseDirs::new().expect("unable to get home dir");
        let app_dir = base.home_dir().join(".cash_cat");

        if !app_dir.exists() {
            std::fs::create_dir(&app_dir)
                .expect(format!("Could not create directory: {:?}", app_dir).as_str());
        }

        rusqlite::Connection::open(&app_dir.join("cash_cat.db").to_str().unwrap())
            .expect("Could not open database.")
    }
}
