#[derive(Debug)]
pub struct Ticket {
    pub id: Option<i64>,
    pub customer_id: i64,
    pub name: String,
    pub description: String,
}

impl super::FindableById for Ticket {
    fn find_by_id(con: &rusqlite::Connection, id: i64) -> anyhow::Result<Self> {
        let mut stmt = con.prepare("Select * from tickets where id = ?")?;
        let ticket = stmt.query_row([id], |row| {
            Ok(Ticket {
                id: Some(row.get(0)?),
                customer_id: row.get(1)?,
                name: row.get(2)?,
                description: row.get(3)?,
            })
        });
        ticket.map_err(anyhow::Error::msg)
    }
}

impl super::FindableByName for Ticket {
    fn find_by_name(con: &rusqlite::Connection, name: &str) -> anyhow::Result<Self> {
        let mut stmt = con.prepare("Select * from tickets where `name` = ?")?;
        let ticket = stmt.query_row([name], |row| {
            Ok(Ticket {
                id: Some(row.get(0)?),
                customer_id: row.get(1)?,
                name: row.get(2)?,
                description: row.get(3)?,
            })
        });
        ticket.map_err(anyhow::Error::msg)
    }
}

impl super::Insertable for Ticket {
    fn insert(&self, con: &rusqlite::Connection) -> anyhow::Result<Self> {
        let mut stmt =
            con.prepare("INSERT INTO tickets (customer_id, name, description) VALUES (?, ?, ?)")?;
        stmt.execute((&self.customer_id, &self.name, &self.description))?;
        Ok(Ticket {
            id: Some(con.last_insert_rowid()),
            customer_id: self.customer_id,
            name: self.name.clone(),
            description: self.description.clone(),
        })
    }
}
