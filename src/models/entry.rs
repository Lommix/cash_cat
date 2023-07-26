use std::time::Duration;

#[derive(Debug)]
pub struct TimeEntry {
    pub id: Option<i64>,
    pub customer_id: i64,
    pub ticket_id: i64,
    pub minutes: i64,
}

impl super::Insertable for TimeEntry {
    fn insert(&self, con: &rusqlite::Connection) -> anyhow::Result<Self> {
        let mut stmt =
            con.prepare("INSERT INTO entries (customer_id, ticket_id, minutes) VALUES (?, ?, ?)")?;
        stmt.execute((self.customer_id, self.ticket_id, self.minutes))?;
        Ok(TimeEntry {
            id: Some(con.last_insert_rowid()),
            customer_id: self.customer_id,
            ticket_id: self.ticket_id,
            minutes: self.minutes,
        })
    }
}

impl super::Deleteable for TimeEntry {
    fn delete(con: &rusqlite::Connection, id: i64) -> anyhow::Result<()> {
        let mut stmt = con.prepare("Delete from entries where id = ?")?;
        stmt.execute([id])?;
        Ok(())
    }
}
