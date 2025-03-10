use clap::{Parser, Subcommand};
use chrono::Local;

use crate::api::OuraClient;
use crate::config::ConfigManager;
use crate::model::{AppError, CliConfig};
use crate::output::OutputFormatter;
use crate::service::SleepScoreService;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    Configure {
        #[arg(short, long)]
        oura_token: String,
    },
    Show {},
    Latest {},
    Score {
        #[arg(short, long)]
        start_date: String,
        #[arg(short, long)]
        end_date: String,
        #[arg(short, long, default_value = "text")]
        output_format: String, // text or json
    },
}

pub struct App<C: OuraClient, M: ConfigManager, O: OutputFormatter> {
    pub config_manager: M,
    client_factory: Box<dyn Fn(&str) -> C>,
    pub output_formatter: O,
}

impl<C: OuraClient, M: ConfigManager, O: OutputFormatter> App<C, M, O> {
    pub fn new(
        config_manager: M,
        client_factory: Box<dyn Fn(&str) -> C>,
        output_formatter: O,
    ) -> Self {
        Self {
            config_manager,
            client_factory,
            output_formatter,
        }
    }

    pub fn run(&self, command: Commands) -> Result<(), AppError> {
        match command {
            Commands::Configure { oura_token } => self.handle_configure(oura_token),
            Commands::Show {} => self.handle_show(),
            Commands::Latest {} => self.handle_latest(),
            Commands::Score {
                start_date,
                end_date,
                output_format,
            } => self.handle_score(&start_date, &end_date, &output_format),
        }
    }

    fn handle_configure(&self, oura_token: String) -> Result<(), AppError> {
        let config = CliConfig {
            oura_token,
        };
        
        self.config_manager.save_config(&config)?;
        println!("Oura token has been configured.");
        
        Ok(())
    }

    fn handle_show(&self) -> Result<(), AppError> {
        let config = self.config_manager.load_config()?;
        println!("Oura token: {}", config.oura_token);
        
        Ok(())
    }

    fn handle_latest(&self) -> Result<(), AppError> {
        let today = Local::now().format("%Y-%m-%d").to_string();
        self.handle_score(&today, &today, "text")
    }

    fn handle_score(&self, start_date: &str, end_date: &str, output_format: &str) -> Result<(), AppError> {
        let config = self.config_manager.load_config()?;
        
        let client = (self.client_factory)(&config.oura_token);
        let service = SleepScoreService::new(client);
        
        let scores = service.get_sleep_scores(start_date, end_date)?;
        
        self.output_formatter.print_sleep_scores(&scores, output_format)?;
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::tests::MockOuraClient;
    use crate::config::tests::MockConfigManager;
    use crate::model::SleepEntry;
    use crate::output::tests::MockOutputFormatter;
    use std::rc::Rc;

    #[test]
    fn test_handle_configure() {
        // Setup
        let config_manager = MockConfigManager::new("");
        let client_factory = Box::new(|_token: &str| MockOuraClient::new(vec![]));
        let output_formatter = MockOutputFormatter::new();

        let app = App::new(config_manager, client_factory, output_formatter);

        // Execute
        let result = app.handle_configure("test_token".to_string());

        // Verify
        assert!(result.is_ok());
        assert!(*app.config_manager.save_called.borrow());
        assert_eq!(app.config_manager.config.borrow().oura_token, "test_token");
    }

    #[test]
    fn test_handle_show() {
        // Setup
        let config_manager = MockConfigManager::new("test_token");
        let client_factory = Box::new(|_token: &str| MockOuraClient::new(vec![]));
        let output_formatter = MockOutputFormatter::new();

        let app = App::new(config_manager, client_factory, output_formatter);

        // Execute
        let result = app.handle_show();

        // Verify
        assert!(result.is_ok());
        assert!(*app.config_manager.load_called.borrow());
    }

    #[test]
    fn test_handle_score() {
        // Setup
        let test_entries = vec![
            SleepEntry {
                day: "2023-01-01".to_string(),
                score: 85,
            },
            SleepEntry {
                day: "2023-01-02".to_string(),
                score: 90,
            },
        ];

        let config_manager = MockConfigManager::new("test_token");
        
        // Create a shared mock client
        let client = Rc::new(MockOuraClient::new(test_entries));
        let client_clone = client.clone();
        
        let client_factory = Box::new(move |_token: &str| {
            MockOuraClient::new(
                client_clone.sleep_data.borrow().data.clone()
            )
        });
        
        let output_formatter = MockOutputFormatter::new();

        let app = App::new(config_manager, client_factory, output_formatter);

        // Execute
        let result = app.handle_score("2023-01-01", "2023-01-02", "text");

        // Verify
        assert!(result.is_ok());
        assert!(*app.config_manager.load_called.borrow());
        
        let output = app.output_formatter.get_output();
        assert_eq!(output.len(), 2);
        assert_eq!(output[0], "\"2023-01-01\",85");
        assert_eq!(output[1], "\"2023-01-02\",90");
    }

    #[test]
    fn test_handle_score_with_json_output() {
        // Setup
        let test_entries = vec![
            SleepEntry {
                day: "2023-01-01".to_string(),
                score: 85,
            },
        ];

        let config_manager = MockConfigManager::new("test_token");
        
        let client_factory = Box::new(move |_token: &str| {
            MockOuraClient::new(test_entries.clone())
        });
        
        let output_formatter = MockOutputFormatter::new();

        let app = App::new(config_manager, client_factory, output_formatter);

        // Execute
        let result = app.handle_score("2023-01-01", "2023-01-01", "json");

        // Verify
        assert!(result.is_ok());
        
        let output = app.output_formatter.get_output();
        assert_eq!(output.len(), 1);
        assert!(output[0].contains("\"date\":\"2023-01-01\""));
        assert!(output[0].contains("\"score\":85"));
    }
}