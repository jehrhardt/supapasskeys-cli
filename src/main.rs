use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(about = "Supapasskeys extension for Supabase", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(about = "Start local Supapasskeys instance and connect to Supabase", long_about = None)]
    Start,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Start => {
            println!("Start command");
        }
    }
}
