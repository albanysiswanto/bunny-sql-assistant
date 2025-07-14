mod ai;
mod config;
mod db;

use clap::{Args, Parser, Subcommand};
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

#[derive(Parser)]
#[command(name = "bunny", about = "🐰 Bunny SQL Assistant")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Config(ConfigArgs),

    Query {
        #[arg(help = "Prompt natural language (ID/EN)")]
        prompt: String,
    },
}

#[derive(Args)]
struct ConfigArgs {
    #[arg(long, help = "Tipe database: sqlite atau postgres")]
    db_type: String,

    #[arg(help = "Database URL (contoh: postgres://user:pass@localhost/db)")]
    url: String,
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let cli = Cli::parse();

    match &cli.command {
        Commands::Config(args) => {
            config::save_db_config(&args.db_type, &args.url);
            println!("✅ Database configuration saved!");
        }

        Commands::Query { prompt } => {
            let spinner = ProgressBar::new_spinner();
            spinner.set_message("🤖 Turning the prompt into SQL...");
            spinner.enable_steady_tick(Duration::from_millis(100));
            spinner.set_style(
                ProgressStyle::with_template("{spinner} {msg}")
                    .unwrap()
                    .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
            );

            let sql_result = ai::generate_sql(prompt).await;
            spinner.finish_and_clear();

            match sql_result {
                Ok(sql) => {
                    println!("\n📜 SQL generated:\n");

                    let border = "─".repeat(50);
                    println!("{}", border.cyan());
                    println!("{}", sql.trim().cyan());
                    println!("{}\n", border.cyan());

                    match config::load_db_config() {
                        Ok(conf) => {
                            // println!("DEBUG: DB Type = {}", conf.db_type);
                            // println!("DEBUG: DB URL  = {}", conf.url);

                            if let Err(e) =
                                db::execute_sql_and_print(&sql, &conf.url, &conf.db_type).await
                            {
                                eprintln!("❌ Failed to run the query: {}", e);
                            }
                        }
                        Err(e) => {
                            eprintln!("❌ Failed to read the database configuration: {}", e);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("❌ Failed to generate SQL: {}", e);
                }
            }
        }
    }
}
