use clap::Parser;
use oura_cli::{
    api::HttpOuraClient,
    cli::{App, Cli},
    config::FileConfigManager,
    output::ConsoleOutputFormatter,
    CONFIG_APP_NAME, API_BASE_URL,
};

fn main() {
    // Parse command line arguments
    let cli = Cli::parse();
    
    // Create dependencies
    let config_manager = FileConfigManager::new(CONFIG_APP_NAME);
    
    // Factory for creating API clients
    let client_factory = Box::new(|token: &str| {
        HttpOuraClient::new(token, API_BASE_URL)
    });
    
    let output_formatter = ConsoleOutputFormatter::new();
    
    // Create the application with dependencies
    let app = App::new(config_manager, client_factory, output_formatter);
    
    // Run the application with the parsed command
    if let Some(command) = cli.command {
        if let Err(e) = app.run(command) {
            eprintln!("Error: {}", e);
        }
    }
}
