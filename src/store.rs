pub struct Store;

impl Store {
    pub fn up() {
        let con = Store::open_connection();
        let customer_table =
            "CREATE TABLE IF NOT EXISTS customers (id INTEGER PRIMARY KEY, name TEXT)";
        // con.execute(customer_table).unwrap();
    }

    pub fn down() {
        let con = Store::open_connection();
    }

    pub fn open_connection() -> rusqlite::Connection {
        let home_dir = std::env::home_dir().expect("Could not get home directory.");
        let app_dir = home_dir.join(".cash_cat");

        if !app_dir.exists() {
            std::fs::create_dir(&app_dir)
                .expect(format!("Could not create directory: {:?}", app_dir).as_str());
        }

        rusqlite::Connection::open(&app_dir.join("cash_cat.db").to_str().unwrap())
            .expect("Could not open database.")
    }
}
