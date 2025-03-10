use crate::model::{AppError, SleepScore};
use serde_json::json;

pub trait OutputFormatter {
    fn print_sleep_scores(&self, scores: &[SleepScore], format: &str) -> Result<(), AppError>;
}

pub struct ConsoleOutputFormatter;

impl ConsoleOutputFormatter {
    pub fn new() -> Self {
        Self {}
    }
}

impl OutputFormatter for ConsoleOutputFormatter {
    fn print_sleep_scores(&self, scores: &[SleepScore], format: &str) -> Result<(), AppError> {
        match format {
            "text" => {
                for score in scores {
                    println!("\"{}\",{}", score.date, score.score);
                }
                Ok(())
            }
            "json" => {
                let json_scores: Vec<_> = scores
                    .iter()
                    .map(|s| json!({"date": s.date, "score": s.score}))
                    .collect();
                
                let json_string = serde_json::to_string(&json_scores)
                    .map_err(|e| AppError::Io(format!("Failed to serialize to JSON: {}", e)))?;
                
                println!("{}", json_string);
                Ok(())
            }
            _ => Err(AppError::InvalidInput(format!("Unsupported output format: {}", format))),
        }
    }
}

#[cfg(test)]
mod format_tests {
    use super::*;
    
    #[test]
    fn test_console_formatter_text_format() {
        let formatter = ConsoleOutputFormatter::new();
        let scores = vec![
            SleepScore {
                date: "2023-01-01".to_string(),
                score: 85,
            },
        ];
        
        // This test just verifies no errors - we can't easily capture the output
        // from println in this test, so we just check it doesn't error.
        let result = formatter.print_sleep_scores(&scores, "text");
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_console_formatter_json_format() {
        let formatter = ConsoleOutputFormatter::new();
        let scores = vec![
            SleepScore {
                date: "2023-01-01".to_string(),
                score: 85,
            },
        ];
        
        let result = formatter.print_sleep_scores(&scores, "json");
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_console_formatter_invalid_format() {
        let formatter = ConsoleOutputFormatter::new();
        let scores = vec![
            SleepScore {
                date: "2023-01-01".to_string(),
                score: 85,
            },
        ];
        
        let result = formatter.print_sleep_scores(&scores, "invalid");
        assert!(result.is_err());
        match result {
            Err(AppError::InvalidInput(msg)) => {
                assert!(msg.contains("Unsupported output format"));
            },
            _ => panic!("Expected InvalidInput error"),
        }
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use std::cell::RefCell;

    pub struct MockOutputFormatter {
        pub output: RefCell<Vec<String>>,
    }

    impl MockOutputFormatter {
        pub fn new() -> Self {
            Self {
                output: RefCell::new(Vec::new()),
            }
        }

        pub fn get_output(&self) -> Vec<String> {
            self.output.borrow().clone()
        }
    }

    impl OutputFormatter for MockOutputFormatter {
        fn print_sleep_scores(&self, scores: &[SleepScore], format: &str) -> Result<(), AppError> {
            match format {
                "text" => {
                    for score in scores {
                        self.output
                            .borrow_mut()
                            .push(format!("\"{}\",{}", score.date, score.score));
                    }
                    Ok(())
                }
                "json" => {
                    let json_scores: Vec<_> = scores
                        .iter()
                        .map(|s| json!({"date": s.date, "score": s.score}))
                        .collect();
                    
                    let json_string = serde_json::to_string(&json_scores)
                        .map_err(|e| AppError::Io(format!("Failed to serialize to JSON: {}", e)))?;
                    
                    self.output.borrow_mut().push(json_string);
                    Ok(())
                }
                _ => Err(AppError::InvalidInput(format!("Unsupported output format: {}", format))),
            }
        }
    }
}

#[cfg(not(test))]
pub mod tests {
    use super::*;
    use std::cell::RefCell;

    pub struct MockOutputFormatter {
        pub output: RefCell<Vec<String>>,
    }

    impl MockOutputFormatter {
        pub fn new() -> Self {
            Self {
                output: RefCell::new(Vec::new()),
            }
        }

        pub fn get_output(&self) -> Vec<String> {
            self.output.borrow().clone()
        }
    }

    impl OutputFormatter for MockOutputFormatter {
        fn print_sleep_scores(&self, scores: &[SleepScore], format: &str) -> Result<(), AppError> {
            match format {
                "text" => {
                    for score in scores {
                        self.output
                            .borrow_mut()
                            .push(format!("\"{}\",{}", score.date, score.score));
                    }
                    Ok(())
                }
                "json" => {
                    let json_scores: Vec<_> = scores
                        .iter()
                        .map(|s| json!({"date": s.date, "score": s.score}))
                        .collect();
                    
                    let json_string = serde_json::to_string(&json_scores)
                        .map_err(|e| AppError::Io(format!("Failed to serialize to JSON: {}", e)))?;
                    
                    self.output.borrow_mut().push(json_string);
                    Ok(())
                }
                _ => Err(AppError::InvalidInput(format!("Unsupported output format: {}", format))),
            }
        }
    }
}