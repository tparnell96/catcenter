use clap::{Parser, Subcommand};
use log::error;
use std::process;

mod api;
mod auth;
mod config;
mod utils;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    /// Reset and set up configuration
    #[arg(long)]
    setup: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Manage device inventory
    Inventory {
        /// Get all devices
        #[arg(short, long)]
        all: bool,
    },
    // TUI functionality has been removed for now
    // Interactive,
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let cli = Cli::parse();

    if cli.setup {
        if let Err(e) = config::reset_config() {
            error!("Failed to reset config: {}", e);
            process::exit(1);
        }
        println!("Configuration reset successfully.");
        // Exit after resetting config
        return;
    }

    // Load configuration
    let config = match config::load_config() {
        Ok(cfg) => cfg,
        Err(e) => {
            error!("Failed to load config: {}", e);
            process::exit(1);
        }
    };

    // Authenticate and get token
    let token = match auth::authenticate(&config).await {
        Ok(t) => t,
        Err(e) => {
            error!("Authentication failed: {}", e);
            process::exit(1);
        }
    };

    if let Some(command) = cli.command {
        match command {
            Commands::Inventory { all } => {
                if all {
                    match api::get_all_devices(&config, &token).await {
                        Ok(devices) => {
                            utils::print_devices(devices);
                        }
                        Err(e) => {
                            error!("Failed to get devices: {}", e);
                            process::exit(1);
                        }
                    }
                } else {
                    println!("No action specified for Inventory command.");
                }
            }
            // No TUI command here
        }
    } else {
        println!("No command provided. Use --help for more information.");
    }
}
