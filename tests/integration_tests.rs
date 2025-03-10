use oura_cli::{
    api::tests::MockOuraClient,
    cli::{App, Commands},
    config::tests::MockConfigManager,
    model::SleepEntry,
    output::tests::MockOutputFormatter,
};

#[test]
fn test_configure_command() {
    // Setup
    let config_manager = MockConfigManager::new("");
    let client_factory = Box::new(|_token: &str| MockOuraClient::new(vec![]));
    let output_formatter = MockOutputFormatter::new();

    let app = App::new(config_manager, client_factory, output_formatter);

    // Execute
    let command = Commands::Configure {
        oura_token: "test_token".to_string(),
    };
    let result = app.run(command);

    // Verify
    assert!(result.is_ok());
    assert!(*app.config_manager.save_called.borrow());
    assert_eq!(app.config_manager.config.borrow().oura_token, "test_token");
}

#[test]
fn test_show_command() {
    // Setup
    let config_manager = MockConfigManager::new("test_token");
    let client_factory = Box::new(|_token: &str| MockOuraClient::new(vec![]));
    let output_formatter = MockOutputFormatter::new();

    let app = App::new(config_manager, client_factory, output_formatter);

    // Execute
    let command = Commands::Show {};
    let result = app.run(command);

    // Verify
    assert!(result.is_ok());
    assert!(*app.config_manager.load_called.borrow());
}

#[test]
fn test_latest_command() {
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
    let command = Commands::Latest {};
    let result = app.run(command);

    // Verify
    assert!(result.is_ok());
    assert!(*app.config_manager.load_called.borrow());
    
    let output = app.output_formatter.get_output();
    assert_eq!(output.len(), 1);
    assert_eq!(output[0], "\"2023-01-01\",85");
}

#[test]
fn test_score_command() {
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
    let client_factory = Box::new(move |_token: &str| {
        MockOuraClient::new(test_entries.clone())
    });
    let output_formatter = MockOutputFormatter::new();

    let app = App::new(config_manager, client_factory, output_formatter);

    // Execute
    let command = Commands::Score {
        start_date: "2023-01-01".to_string(),
        end_date: "2023-01-02".to_string(),
        output_format: "text".to_string(),
    };
    let result = app.run(command);

    // Verify
    assert!(result.is_ok());
    assert!(*app.config_manager.load_called.borrow());
    
    let output = app.output_formatter.get_output();
    assert_eq!(output.len(), 2);
    assert_eq!(output[0], "\"2023-01-01\",85");
    assert_eq!(output[1], "\"2023-01-02\",90");
}

#[test]
fn test_score_command_with_json_output() {
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
    let command = Commands::Score {
        start_date: "2023-01-01".to_string(),
        end_date: "2023-01-01".to_string(),
        output_format: "json".to_string(),
    };
    let result = app.run(command);

    // Verify
    assert!(result.is_ok());
    
    let output = app.output_formatter.get_output();
    assert_eq!(output.len(), 1);
    assert!(output[0].contains("\"date\":\"2023-01-01\""));
    assert!(output[0].contains("\"score\":85"));
}

#[test]
fn test_score_command_with_invalid_date_format() {
    // Setup
    let config_manager = MockConfigManager::new("test_token");
    let client_factory = Box::new(|_token: &str| MockOuraClient::new(vec![]));
    let output_formatter = MockOutputFormatter::new();

    let app = App::new(config_manager, client_factory, output_formatter);

    // Execute
    let command = Commands::Score {
        start_date: "01/01/2023".to_string(), // Invalid format (MM/DD/YYYY)
        end_date: "2023-01-02".to_string(),
        output_format: "text".to_string(),
    };
    let result = app.run(command);

    // Verify
    assert!(result.is_err());
}

#[test]
fn test_score_command_with_invalid_date_range() {
    // Setup
    let config_manager = MockConfigManager::new("test_token");
    let client_factory = Box::new(|_token: &str| MockOuraClient::new(vec![]));
    let output_formatter = MockOutputFormatter::new();

    let app = App::new(config_manager, client_factory, output_formatter);

    // Execute
    let command = Commands::Score {
        start_date: "2023-01-02".to_string(), // Start date after end date
        end_date: "2023-01-01".to_string(),
        output_format: "text".to_string(),
    };
    let result = app.run(command);

    // Verify
    assert!(result.is_err());
}