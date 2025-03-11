use chrono::Local;
use clap::Parser;
use confy;
use std::io;

use oura_cli::{Cli, CliConfig, Commands, CONFIG_APP_NAME, get_sleep_score, print_sleep_score_as_csv, print_sleep_score_as_json};

fn main() {
    let args = Cli::parse();
    let mut config: CliConfig = confy::load(CONFIG_APP_NAME, None).unwrap();

    if let Some(command) = args.command {
        match command {
            Commands::Configure { oura_token } => {
                config.oura_token = oura_token;
                confy::store(CONFIG_APP_NAME, None, &config).unwrap();
                println!("Oura token has been configured.");
            }
            Commands::Show {} => {
                println!("Oura token: {}", config.oura_token);
            }
            Commands::Latest {} => {
                let today = Local::now().format("%Y-%m-%d").to_string();

                match get_sleep_score(config, &today, &today) {
                    Ok(scores) => {
                        for score in scores {
                            print_sleep_score_as_csv(
                                score["date"].as_str().unwrap(),
                                score["score"].to_string().as_str(),
                                &mut io::stdout(),
                            ).unwrap();
                        }
                    }
                    Err(e) => eprintln!("Error fetching sleep score: {}", e),
                }
            }
            Commands::Score {
                start_date,
                end_date,
                output_format,
            } => match get_sleep_score(config, &start_date, &end_date) {
                Ok(scores) => {
                    if output_format == "text" {
                        for score in scores {
                            print_sleep_score_as_csv(
                                score["date"].as_str().unwrap(),
                                score["score"].to_string().as_str(),
                                &mut io::stdout(),
                            ).unwrap();
                        }
                    } else {
                        print_sleep_score_as_json(&scores, &mut io::stdout()).unwrap();
                    }
                }
                Err(e) => eprintln!("Error fetching sleep score: {}", e),
            },
        }
        return;
    }
}
