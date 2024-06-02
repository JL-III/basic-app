use anyhow::{anyhow, Result};
use chrono::Utc;
use rusqlite::{params, Connection};

use crate::bill::Bill;

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

pub fn get_database_entry(id: i32) -> Result<Bill> {
    let connection = Connection::open("./data.db")?;
    let mut stmt = connection.prepare(
        "SELECT id, date, description, notes, amount, timestamp, allocated, sent, paid FROM bills WHERE id = ?1",
    )?;
    let mut rows = stmt.query(params![id])?;
    if let Some(row) = rows.next()? {
        let bill = Bill {
            id: row.get(0)?,
            date: row.get(1)?,
            description: row.get(2)?,
            notes: row.get(3)?,
            amount: row.get(4)?,
            timestamp: row.get(5)?,
            allocated: row.get::<_, i32>(6)? != 0,
            sent: row.get::<_, i32>(7)? != 0,
            paid: row.get::<_, i32>(8)? != 0,
        };
        Ok(bill)
    } else {
        Err(anyhow!("No bill found with given id"))
    }
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
