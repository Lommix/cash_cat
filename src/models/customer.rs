// ---------------------------------------
// Customer
#[derive(Debug)]
pub struct Customer {
    pub id: Option<i64>,
    pub name: String,
}

impl Customer {
    pub fn new(name: &str) -> Self {
        Self {
            id: None,
            name: name.to_string(),
        }
    }
}

impl super::FindableById for Customer {
    fn find_by_id(con: &rusqlite::Connection, id: i64) -> anyhow::Result<Self> {
        let mut stmt = con.prepare("Select * from customers where id = ?")?;
        let customer = stmt.query_row([id], |row| {
            Ok(Customer {
                id: Some(row.get(0)?),
                name: row.get(1)?,
            })
        });
        customer.map_err(anyhow::Error::msg)
    }
}

impl super::FindableByName for Customer {
    fn find_by_name(con: &rusqlite::Connection, name: &str) -> anyhow::Result<Self> {
        let mut stmt = con.prepare("Select * from customers where name = ?")?;
        let customer = stmt.query_row([name], |row| {
            Ok(Customer {
                id: Some(row.get(0)?),
                name: row.get(1)?,
            })
        });
        customer.map_err(anyhow::Error::msg)
    }
}

impl super::Insertable for Customer {
    fn insert(&self, con: &rusqlite::Connection) -> anyhow::Result<Self> {
        let mut stmt = con.prepare("INSERT INTO customers (name) VALUES (?)")?;
        stmt.execute([self.name.as_str()])?;
        Ok(Customer {
            id: Some(con.last_insert_rowid()),
            name: self.name.clone(),
        })
    }
}

impl super::Deleteable for Customer {
    fn delete(con: &rusqlite::Connection, id: i64) -> anyhow::Result<()> {
        let mut stmt = con.prepare("Delete from customers where id = ?")?;
        stmt.execute([id])?;
        Ok(())
    }
}
