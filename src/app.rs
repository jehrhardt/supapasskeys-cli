use clap::{Parser, Subcommand};

use crate::commands;

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

pub async fn start() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Start => {
            commands::start::run().await;
        }
    }
}
