use std::process;
use clap::Parser;
use reqwest::blocking::Client;
use reqwest::Error;
use serde::{Deserialize, Serialize};
use confy;
#[derive(Parser)]
struct Cli {
    start_date: String,
    end_date: String,
}

#[derive(Default, Serialize, Deserialize)]
struct CliConfig {
    oura_token: String,
}

fn main() {
    let args = Cli::parse();
    let config: CliConfig = confy::load("oura-cli", None).unwrap();

    let start_date = args.start_date.as_str();
    let end_date = args.end_date.as_str();
    let token = config.oura_token.as_str();

    if token.is_empty() {
        eprintln!("Error: Oura token is missing in the configuration.");
        process::exit(1);
    }

    match get_sleep_score(start_date, end_date, token) {
        Ok(scores) => {
            let json_scores = serde_json::to_string(&scores).expect("Failed to serialize scores to JSON");
            println!("{}", json_scores);

            // for (date, score) in scores {
            //     println!("Date: {}, Sleep score: {}", date, score);
            // }
        }
        Err(e) => eprintln!("Error fetching sleep score: {}", e),
    }
}

#[derive(Deserialize)]
struct SleepData {
    data: Vec<SleepEntry>,
}

#[derive(Deserialize)]
struct SleepEntry {
    day: String,
    score: u32,
}

fn get_sleep_score(
    start_date: &str,
    end_date: &str,
    token: &str,
) -> Result<Vec<(String, u32)>, Error> {
    let url = format!(
        "https://api.ouraring.com/v2/usercollection/daily_sleep?start_date={}&end_date={}",
        start_date, end_date
    );

    let client = Client::new();
    let response = client.get(&url).bearer_auth(token).send()?;

    let response_text = response.text()?;

    let sleep_data: SleepData = serde_json::from_str(&response_text).unwrap();

    let mut sleep_scores: Vec<(String, u32)> = sleep_data
        .data
        .into_iter()
        .map(|entry| (entry.day, entry.score))
        .collect();

    sleep_scores.sort_by(|a, b| a.0.cmp(&b.0));

    Ok(sleep_scores)
}
