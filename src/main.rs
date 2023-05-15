use anyhow::Result;
use rusqlite::{params, Connection};
use std::fs;
use std::path::PathBuf;
use std::time::Instant;

use clap::Parser;

#[derive(Parser)]
struct Config {
    #[clap(index = 1)]
    db_file: PathBuf,

    #[clap(long, default_value = "delete")]
    journal_mode: String,

    #[clap(long, default_value = "full")]
    synchronous: String,

    #[clap(long, default_value = "1000")]
    batch_size: usize,

    #[clap(long, default_value = "1000")]
    batch_count: usize,

    #[clap(long, default_value = "100")]
    row_size: usize,

    #[clap(long, default_value = "")]
    table_options: String,
}

fn main() -> Result<()> {
    let config = Config::parse();

    let journal_mode = config.journal_mode;
    let synchronous = config.synchronous;
    let batch_size = config.batch_size;
    let batch_count = config.batch_count;
    let row_size = config.row_size;
    let db_file = config.db_file;
    let table_options = config.table_options;

    fs::remove_file(&db_file).ok(); // Remove db file, if it already exists.

    let mut conn = Connection::open(&db_file)?;

    // Set journal settings.
    conn.query_row(
        format!("PRAGMA journal_mode = {}", journal_mode).as_str(),
        [],
        |_| Ok(()),
    )?;
    conn.execute(format!("PRAGMA synchronous = {}", synchronous).as_str(), [])?;

    // Initialize schema.
    conn.execute("DROP TABLE IF EXISTS t", [])?;
    let schema = "CREATE TABLE t (id, name TEXT) ".to_owned() + &table_options;
    conn.execute(&schema, [])?;

    let name = "x".repeat(row_size);
    let mut current_id = 0;
    let start = Instant::now();
    for _ in 0..batch_count {
        insert_batch(&mut conn, batch_size, &name, &mut current_id)?;
    }
    let elapsed = start.elapsed();

    // Checkpoint if using WAL.
    conn.query_row("PRAGMA wal_checkpoint(TRUNCATE)", [], |_| Ok(()))?;

    // Check file size.
    let file_size = fs::metadata(&db_file)?.len();

    println!("Inserts:   {} rows", batch_size * batch_count);
    println!("Elapsed:   {:0.03}s", elapsed.as_secs_f64());
    println!(
        "Rate:      {:0.03} insert/sec",
        (batch_size * batch_count) as f64 / elapsed.as_secs_f64()
    );
    println!("File size: {} bytes", file_size);

    Ok(())
}

fn insert_batch(
    conn: &mut Connection,
    batch_size: usize,
    name: &str,
    current_id: &mut i64,
) -> Result<()> {
    let tx = conn.transaction()?;
    for _ in 0..batch_size {
        tx.execute(
            "INSERT INTO t (id, name) VALUES (?1, ?2)",
            params![current_id.to_string(), name],
        )?;
        *current_id += 1;
    }
    tx.commit()?;
    Ok(())
}
