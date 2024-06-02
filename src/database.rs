use chrono::Utc;
use rusqlite::{params, Connection, Result};

pub fn initialize_database() -> Result<()> {
    let connection = Connection::open("./data.db")?;
    let query = "
      CREATE TABLE IF NOT EXISTS bills (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        date TEXT NOT NULL,
        description TEXT NOT NULL,
        notes TEXT,
        amount REAL NOT NULL,
        timestamp TEXT NOT NULL,
        allocated INTEGER NOT NULL,
        sent INTEGER NOT NULL,
        paid INTEGER NOT NULL
      );
    ";
    connection.execute(query, ())?;
    println!("successfully initialized database!");
    Ok(())
}

pub fn seed_database() -> Result<()> {
    let connection = Connection::open("./data.db")?;
    add_bill(
        &connection,
        "06/01/2024",
        "rent",
        "",
        690.00,
        false,
        false,
        false,
    )?;
    add_bill(
        &connection,
        "06/01/2024",
        "storage unit",
        "",
        70.00,
        false,
        false,
        false,
    )?;
    println!("successfully seeded database!");
    Ok(())
}

pub fn get_database_data() -> Result<()> {
    print!("fetching database data");
    let connection = Connection::open("./data.db")?;
    let mut stmt = connection.prepare(
        "SELECT id, date, description, notes, amount, timestamp, allocated, sent, paid FROM bills",
    )?;
    let bills_iter = stmt.query_map([], |row| {
        Ok((
            row.get::<_, i32>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?,
            row.get::<_, String>(3)?,
            row.get::<_, f64>(4)?,
            row.get::<_, String>(5)?,
            row.get::<_, i32>(6)? != 0,
            row.get::<_, i32>(7)? != 0,
            row.get::<_, i32>(8)? != 0,
        ))
    })?;
    for bill in bills_iter {
        print!("{:?}", bill?);
    }
    Ok(())
}

fn add_bill(
    conn: &Connection,
    date: &str,
    description: &str,
    notes: &str,
    amount: f64,
    allocated: bool,
    sent: bool,
    paid: bool,
) -> Result<()> {
    let timestamp = Utc::now().to_rfc3339();
    conn.execute(
    "INSERT INTO bills (date, description, notes, amount, timestamp, allocated, sent, paid) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
    params![
      date,
      description,
      notes,
      amount,
      timestamp,
      allocated as i32,
      sent as i32,
      paid as i32,
    ],
  )?;
    print!("successfully added bill!");
    Ok(())
}
