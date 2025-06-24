use clap::{Parser, Subcommand, ValueEnum};
use dotenvy::from_filename;
use std::env;

mod migration;

#[derive(Parser)]
#[command(name = "migrate")]
#[command(about = "A simple PostgreSQL migration tool", long_about = None)]
struct Cli {
    /// Environment to use (production, test, development)
    #[arg(long, value_enum, default_value_t = Env::Development)]
    env: Env,

    #[clap(subcommand)]
    command: Commands,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Env {
    Production,
    Test,
    Development,
}

#[derive(Subcommand)]
enum Commands {
    /// List pending migrations
    List,

    /// Apply pending migrations
    Up,

    /// Roll back migrations
    Down {
        /// Number of steps to roll back (default 1, -1 = all)
        #[arg(short, long)]
        step: Option<i32>,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // Load corresponding .env file
    let env_filename = match cli.env {
        Env::Production => ".env.production",
        Env::Test => ".env.test",
        Env::Development => ".env",
    };
    from_filename(env_filename).ok();

    
    // Get schema from env or default to "public"
    let schema = env::var("MIGRATION_SCHEMA").unwrap_or_else(|_| "public".to_string());

    let db_url = format!(
        "{}?options=-c%20search_path={}",
        env::var("MIGRATION_DATABASE_URL").expect("MIGRATION_DATABASE_URL must be set"),
        schema
    );

    match cli.command {
        Commands::List => {
            println!("Available migrations:");

            let mut conn = postgres::Client::connect(&db_url, postgres::NoTls)?;

            migration::list(&mut conn, &schema)?;

            conn.close()?;
        },
        Commands::Up => {
            let mut conn = postgres::Client::connect(&db_url, postgres::NoTls)?;

            migration::up(&mut conn, &schema)?;

            conn.close()?;
            println!("Migrations applied successfully.");
        },
        Commands::Down { step } => {
            let mut conn = postgres::Client::connect(&db_url, postgres::NoTls)?;
            
            migration::down(&mut conn, step, &schema)?;

            conn.close()?;

            println!("Migrations rolled back successfully.");
        },
    }

    Ok(())
}
