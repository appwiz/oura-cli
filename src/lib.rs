// Remove unused import
use clap::{Parser, Subcommand};
use reqwest::blocking::Client;
use reqwest::Error;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::env;
use std::io::{self, Write};
use std::process;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug, PartialEq)]
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

#[derive(Default, Serialize, Deserialize, Debug, PartialEq)]
pub struct CliConfig {
    pub oura_token: String,
}

pub const CONFIG_APP_NAME: &'static str = "oura-cli";

#[derive(Deserialize, Debug, PartialEq)]
pub struct SleepData {
    pub data: Vec<SleepEntry>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct SleepEntry {
    pub day: String,
    pub score: u32,
}

pub fn print_sleep_score_as_json<W: Write>(scores: &Vec<Value>, writer: &mut W) -> io::Result<()> {
    let json_scores = serde_json::to_string(&scores).expect("Failed to serialize scores to JSON");
    writeln!(writer, "{}", json_scores)
}

pub fn print_sleep_score_as_csv<W: Write>(date: &str, score: &str, writer: &mut W) -> io::Result<()> {
    writeln!(writer, "\"{}\",{}", date, score)
}

pub fn get_sleep_score(
    cli_config: CliConfig,
    start_date: &str,
    end_date: &str,
) -> Result<Vec<serde_json::Value>, Error> {
    let base_url = env::var("OURA_API_URL").unwrap_or_else(|_| "https://api.ouraring.com".to_string());
    
    let url = format!(
        "{}/v2/usercollection/daily_sleep?start_date={}&end_date={}",
        base_url, start_date, end_date
    );

    let token = cli_config.oura_token.as_str();

    if token.is_empty() {
        eprintln!("Error: Oura token is missing in the configuration.");
        process::exit(1);
    }

    let client = Client::new();
    let response = client.get(&url).bearer_auth(token).send()?;

    let response_text = response.text()?;

    // Parse the JSON response
    let sleep_data: SleepData = match serde_json::from_str(&response_text) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error parsing response: {}", e);
            // Return an empty data structure
            SleepData { data: vec![] }
        }
    };

    let mut sleep_scores: Vec<serde_json::Value> = sleep_data
        .data
        .into_iter()
        .map(|entry| json!({ "date": entry.day, "score": entry.score }))
        .collect();

    sleep_scores.sort_by(|a, b| a["date"].as_str().cmp(&b["date"].as_str()));

    Ok(sleep_scores)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_cli_config_default() {
        let config = CliConfig::default();
        assert_eq!(config.oura_token, "");
    }
    
    #[test]
    fn test_sleep_data_deserialization() {
        let json_data = r#"
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
        
        let sleep_data: SleepData = serde_json::from_str(json_data).unwrap();
        
        assert_eq!(sleep_data.data.len(), 2);
        assert_eq!(sleep_data.data[0].day, "2023-01-01");
        assert_eq!(sleep_data.data[0].score, 85);
        assert_eq!(sleep_data.data[1].day, "2023-01-02");
        assert_eq!(sleep_data.data[1].score, 75);
    }
    
    #[test]
    fn test_print_sleep_score_as_csv() {
        let date = "2023-01-01";
        let score = "85";
        
        let mut output = Vec::new();
        print_sleep_score_as_csv(date, score, &mut output).unwrap();
        
        let output_str = String::from_utf8(output).unwrap();
        assert_eq!(output_str, "\"2023-01-01\",85\n");
    }
    
    #[test]
    fn test_print_sleep_score_as_json() {
        let scores = vec![
            json!({"date": "2023-01-01", "score": 85}),
            json!({"date": "2023-01-02", "score": 75}),
        ];
        
        let mut output = Vec::new();
        print_sleep_score_as_json(&scores, &mut output).unwrap();
        
        let output_str = String::from_utf8(output).unwrap();
        let expected = r#"[{"date":"2023-01-01","score":85},{"date":"2023-01-02","score":75}]
"#;
        assert_eq!(output_str, expected);
    }
}
