use clap::{Parser, Subcommand};
use dotenvy::dotenv;
use std::env;

mod migration;

#[derive(Parser)]
#[command(name = "migrate")]
#[command(about = "A simple PostgreSQL migration tool", long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
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
    dotenv().ok();

    let cli = Cli::parse();

    let db_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env or environment");

    match cli.command {
        Commands::List => {
            println!("Available migrations:");

            let mut conn = postgres::Client::connect(&db_url, postgres::NoTls)?;

            migration::list(&mut conn)?;

            conn.close()?;
        },
        Commands::Up => {
            let mut conn = postgres::Client::connect(&db_url, postgres::NoTls)?;

            migration::up(&mut conn)?;

            conn.close()?;
            println!("Migrations applied successfully.");
        },
        Commands::Down { step } => {
            let mut conn = postgres::Client::connect(&db_url, postgres::NoTls)?;
            
            migration::down(&mut conn, step)?;

            conn.close()?;

            println!("Migrations rolled back successfully.");
        },
    }

    Ok(())
}
