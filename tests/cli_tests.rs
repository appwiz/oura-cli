use clap::Parser;
use oura_cli::{Cli, Commands};

#[test]
fn test_cli_parse_configure_command() {
    let args = vec!["oura-cli", "configure", "--oura-token", "test_token"];
    let cli = Cli::parse_from(args);
    
    match cli.command {
        Some(Commands::Configure { oura_token }) => {
            assert_eq!(oura_token, "test_token");
        }
        _ => panic!("Expected Configure command"),
    }
}

#[test]
fn test_cli_parse_show_command() {
    let args = vec!["oura-cli", "show"];
    let cli = Cli::parse_from(args);
    
    match cli.command {
        Some(Commands::Show {}) => {}
        _ => panic!("Expected Show command"),
    }
}

#[test]
fn test_cli_parse_latest_command() {
    let args = vec!["oura-cli", "latest"];
    let cli = Cli::parse_from(args);
    
    match cli.command {
        Some(Commands::Latest {}) => {}
        _ => panic!("Expected Latest command"),
    }
}

#[test]
fn test_cli_parse_score_command() {
    let args = vec![
        "oura-cli", "score", 
        "--start-date", "2023-01-01", 
        "--end-date", "2023-01-02",
        "--output-format", "json"
    ];
    let cli = Cli::parse_from(args);
    
    match cli.command {
        Some(Commands::Score { start_date, end_date, output_format }) => {
            assert_eq!(start_date, "2023-01-01");
            assert_eq!(end_date, "2023-01-02");
            assert_eq!(output_format, "json");
        }
        _ => panic!("Expected Score command"),
    }
}

#[test]
fn test_cli_parse_score_command_default_format() {
    let args = vec![
        "oura-cli", "score", 
        "--start-date", "2023-01-01", 
        "--end-date", "2023-01-02"
    ];
    let cli = Cli::parse_from(args);
    
    match cli.command {
        Some(Commands::Score { start_date, end_date, output_format }) => {
            assert_eq!(start_date, "2023-01-01");
            assert_eq!(end_date, "2023-01-02");
            assert_eq!(output_format, "text");
        }
        _ => panic!("Expected Score command"),
    }
}
