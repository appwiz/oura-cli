use mockito::Server;
use oura_cli::{CliConfig, get_sleep_score};
use std::env;

#[test]
fn test_get_sleep_score_with_mock() {
    // Start a mock server
    let mut server = Server::new();
    
    // Define the mock response
    let mock_response = r#"
    {
        "data": [
            {
                "day": "2023-01-01",
                "score": 85
            },
            {
                "day": "2023-01-02",
                "score": 75
            }
        ]
    }
    "#;

    // Set up the mock endpoint
    let _m = server.mock("GET", "/v2/usercollection/daily_sleep?start_date=2023-01-01&end_date=2023-01-02")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(mock_response)
        .create();
    
    // Override the API URL with our mock server
    env::set_var("OURA_API_URL", &server.url());
    
    let config = CliConfig {
        oura_token: "test_token".to_string(),
    };
    
    let result = get_sleep_score(config, "2023-01-01", "2023-01-02");
    
    assert!(result.is_ok());
    let scores = result.unwrap();
    assert_eq!(scores.len(), 2);
    
    assert_eq!(scores[0]["date"].as_str().unwrap(), "2023-01-01");
    assert_eq!(scores[0]["score"].as_u64().unwrap(), 85);
    assert_eq!(scores[1]["date"].as_str().unwrap(), "2023-01-02");
    assert_eq!(scores[1]["score"].as_u64().unwrap(), 75);
}
