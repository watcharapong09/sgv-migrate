use postgres::Client;
use std::fs;
use std::path::Path;
use regex::Regex;

pub fn list(conn: &mut Client) -> Result<(), Box<dyn std::error::Error>> {
    ensure_migrations_table(conn)?;

    // 1. Load applied versions from the database
    let applied_versions: std::collections::HashSet<String> = conn
        .query("SELECT name FROM migrations ORDER BY name", &[])?
        .into_iter()
        .map(|row| row.get::<_, String>(0))
        .collect();

    println!("== Pending migrations ==");

    // 2. Scan migration files
    let mut pending = 0;
    for entry in fs::read_dir("migrations")? {
        let path = entry?.path();
        if path.extension().and_then(|s| s.to_str()) != Some("sql") {
            continue;
        }

        let filename = path.file_stem().unwrap().to_str().unwrap();
        let (version, _name) = filename.split_once('_').ok_or("Invalid filename format")?;

        // 3. If not applied yet, print it
        if !applied_versions.contains(version) {
            println!("{}", path.display());
            pending += 1;
        }
    }

    if pending == 0 {
        println!("✅ No pending migrations.");
    }

    Ok(())
}

pub fn up(conn: &mut Client) -> Result<(), Box<dyn std::error::Error>> {
    ensure_migrations_table(conn)?;

    let mut files: Vec<_> = fs::read_dir("migrations")?
        .filter_map(Result::ok)
        .collect();
    files.sort_by_key(|e| e.path());

    let mut applied_any = false;

    for entry in files {
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("sql") {
            continue;
        }

        let filename = path.file_name().unwrap().to_str().unwrap(); // e.g., 0001_create_users.sql
        let migration = parse_sql_file(&path)?; // Parse up/down SQL

        let already_applied = conn.query_opt(
            "SELECT 1 FROM migrations WHERE name = $1",
            &[&filename],
        )?;

        if already_applied.is_none() {
            println!("🔼 Applying: {}", filename);
            conn.batch_execute(&migration.up_sql)?;
            conn.execute(
                "INSERT INTO migrations (name) VALUES ($1)",
                &[&filename],
            )?;
            applied_any = true;
        }
    }

    if !applied_any {
        println!("✅ No pending migrations to apply.");
    }

    Ok(())
}

/// Roll back `step` number of migrations:
/// - `None` => 1 step (default)
/// - `Some(-1)` => all
/// - `Some(n)` => n steps
pub fn down(conn: &mut Client, step: Option<i32>) -> Result<(), Box<dyn std::error::Error>> {
    ensure_migrations_table(conn)?;

    let applied: Vec<(String,)> = conn
        .query("SELECT name FROM migrations ORDER BY applied_at DESC", &[])?
        .into_iter()
        .map(|row| (row.get(0),))
        .collect();

    let total_to_rollback = match step {
        Some(n) if n == -1 => applied.len(),
        Some(n) if n > 0 => n as usize,
        _ => 1, // default
    };

    let to_rollback = &applied[..total_to_rollback.min(applied.len())];

    if to_rollback.is_empty() {
        println!("⚠️  No migrations to revert.");
        return Ok(());
    }

    for (name,) in to_rollback {
        let path = Path::new("migrations").join(name);
        if !path.exists() {
            println!("❌ Migration file not found: {}", path.display());
            continue;
        }

        let migration = parse_sql_file(&path)?;
        println!("🔽 Reverting: {}", name);
        conn.batch_execute(&migration.down_sql)?;
        conn.execute("DELETE FROM migrations WHERE name = $1", &[&name])?;
    }

    Ok(())
}

fn ensure_migrations_table(conn: &mut Client) -> Result<(), Box<dyn std::error::Error>> {
    conn.batch_execute(
        "CREATE TABLE IF NOT EXISTS migrations (
            name TEXT PRIMARY KEY,
            applied_at TIMESTAMPTZ DEFAULT now()
        );"
    )?;
    Ok(())
}

struct Migration {
    up_sql: String,
    down_sql: String,
}

fn parse_sql_file(path: &Path) -> Result<Migration, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(path)?;
    let re = Regex::new(r"(?s)-- up\s*(.*?)-- down\s*(.*)")?;
    let caps = re.captures(&content).ok_or("Invalid format")?;

    Ok(Migration {
        up_sql: caps[1].trim().to_string(),
        down_sql: caps[2].trim().to_string(),
    })
}
