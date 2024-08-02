use clap::{Parser, Subcommand};
use confy;
use reqwest::blocking::Client;
use reqwest::Error;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::process;
use chrono::Local;
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}
#[derive(Subcommand)]
enum Commands {
    Configure {
        #[arg(short, long)]
        oura_token: String,
    },
    Show {
    },
    Latest {
    },
    Score {
        #[arg(short, long)]
        start_date: String,
        #[arg(short, long)]
        end_date: String,
        #[arg(short, long, default_value = "text")]
        output_format: String, // text or json
    }
}
#[derive(Default, Serialize, Deserialize)]
struct CliConfig {
    oura_token: String,
}

fn print_sleep_score_as_csv(date: &str, score: &str) {
    println!("\"{}\",{}", date, score);
}

fn main() {
    let args = Cli::parse();
    let mut config: CliConfig = confy::load("oura-cli", None).unwrap();

    if let Some(command) = args.command {
        match command {
            Commands::Configure { oura_token } => {
                config.oura_token = oura_token;
                confy::store("oura-cli", None, &config).unwrap();
                println!("Oura token has been configured.");
            }
            Commands::Show {} => {
                println!("Oura token: {}", config.oura_token);
            }
            Commands::Latest {} => {
                let token = config.oura_token.as_str();

                if token.is_empty() {
                    eprintln!("Error: Oura token is missing in the configuration.");
                    process::exit(1);
                }

                let today = Local::now().format("%Y-%m-%d").to_string();

                match get_sleep_score(&today, &today, token) {
                    Ok(scores) => {
                        for score in scores {
                            print_sleep_score_as_csv(score["date"].as_str().unwrap(), score["score"].to_string().as_str());
                        }
                    }
                    Err(e) => eprintln!("Error fetching sleep score: {}", e),
                }
            }
            Commands::Score { start_date, end_date, output_format } => {
                let token = config.oura_token.as_str();

                if token.is_empty() {
                    eprintln!("Error: Oura token is missing in the configuration.");
                    process::exit(1);
                }

                match get_sleep_score(&start_date, &end_date, token) {
                    Ok(scores) => {
                        if output_format == "text" {
                            for score in scores {
                                print_sleep_score_as_csv(score["date"].as_str().unwrap(), score["score"].to_string().as_str());
                            }
                        } else {
                            let json_scores = serde_json::to_string(&scores).expect("Failed to serialize scores to JSON");
                            println!("{}", json_scores);
                        }
                    }
                    Err(e) => eprintln!("Error fetching sleep score: {}", e),
                }
            }
        }
        return;
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
) -> Result<Vec<serde_json::Value>, Error> {
    let url = format!(
        "https://api.ouraring.com/v2/usercollection/daily_sleep?start_date={}&end_date={}",
        start_date, end_date
    );

    let client = Client::new();
    let response = client.get(&url).bearer_auth(token).send()?;

    let response_text = response.text()?;

    let sleep_data: SleepData = serde_json::from_str(&response_text).unwrap();

    let mut sleep_scores: Vec<serde_json::Value> = sleep_data
        .data
        .into_iter()
        .map(|entry| json!({ "date": entry.day, "score": entry.score }))
        .collect();

    sleep_scores.sort_by(|a, b| a["date"].as_str().cmp(&b["date"].as_str()));

    Ok(sleep_scores)
}
