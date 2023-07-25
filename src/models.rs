use std::time::Duration;

// ---------------------------------------
pub trait Storeable
where
    Self: Sized,
{
    fn find(con: rusqlite::Connection, id: i64) -> Result<Self, rusqlite::Error>;
    fn insert(&self, con: rusqlite::Connection) -> Result<i64, rusqlite::Error>;
    fn delete(con: rusqlite::Connection, id: i64) -> Result<(), rusqlite::Error>;
    fn all(con: rusqlite::Connection) -> Result<Vec<Self>, rusqlite::Error>;
}
// ---------------------------------------
// Customer
pub struct Customer {
    pub id: i64,
    pub name: String,
}

impl Storeable for Customer {
    fn find(con: rusqlite::Connection, id: i64) -> Result<Self, rusqlite::Error> {
        let mut stmt = con.prepare("Select * from customers where id = ?")?;
        stmt.query_row([id], |row| {
            Ok(Customer {
                id: row.get(0)?,
                name: row.get(1)?,
            })
        })
    }

    fn insert(&self, con: rusqlite::Connection) -> Result<i64, rusqlite::Error> {
        let mut stmt = con.prepare("INSERT INTO customers (name) VALUES (?)")?;
        stmt.execute([self.name.as_str()])?;
        Ok(con.last_insert_rowid())
    }

    fn delete(con: rusqlite::Connection, id: i64) -> Result<(), rusqlite::Error> {
        let mut stmt = con.prepare("Delete from customers where id = ?")?;
        stmt.execute([id])?;
        Ok(())
    }

    fn all(con: rusqlite::Connection) -> Result<Vec<Self>, rusqlite::Error> {
        let mut stmt = con.prepare("Select * from customers")?;
        let mut out = Vec::new();
        stmt.query_map([], |row| {
            Ok(Customer {
                id: row.get(0)?,
                name: row.get(1)?,
            })
        })?;
        Ok(out)
    }
}
// ---------------------------------------

pub struct Ticket {
    pub id: i32,
    pub customer_id: i32,
    pub name: String,
    pub description: String,
}

pub struct TimeEntry {
    pub id: i32,
    pub ticket_id: i32,
    pub duration: Duration,
}
