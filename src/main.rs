use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "supapasskeys", about = "Supapasskeys extension for Supabase", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(about = "Start local Supapasskeys instance and connect to Supabase", long_about = None)]
    Start,
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Start => {
            println!("Start command");
        }
    }
}
