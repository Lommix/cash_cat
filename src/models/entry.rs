use std::time::Duration;

#[derive(Debug)]
pub struct TimeEntry {
    pub id: Option<i64>,
    pub customer_id: i64,
    pub ticket_id: i64,
    pub minutes: u64,
    pub created_at: i64,
}

impl TimeEntry {
    pub fn new(customer_id: i64, ticket_id: i64, minutes: u64) -> Self {
        let now = chrono::offset::Local::now().naive_local().timestamp();

        Self {
            id: None,
            customer_id,
            ticket_id,
            minutes,
            created_at: now,
        }
    }
}

impl std::fmt::Display for TimeEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let date = chrono::NaiveDateTime::from_timestamp_opt(self.created_at, 0)
            .unwrap()
            .date();
        write!(
            f,
            "ID[{}] >> {:?}  ticket:{:?}  time:{:?}",
            self.id.unwrap(), date, self.ticket_id, self.minutes
        )
    }
}

impl super::Insertable for TimeEntry {
    fn insert(&self, con: &rusqlite::Connection) -> anyhow::Result<Self> {
        let mut stmt = con.prepare(
            "INSERT INTO entries (customer_id, ticket_id, minutes, created_at) VALUES (?, ?, ?, ?)",
        )?;

        stmt.execute((
            self.customer_id,
            self.ticket_id,
            self.minutes,
            self.created_at,
        ))?;

        Ok(TimeEntry {
            id: Some(con.last_insert_rowid()),
            customer_id: self.customer_id,
            ticket_id: self.ticket_id,
            minutes: self.minutes,
            created_at: self.created_at,
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
