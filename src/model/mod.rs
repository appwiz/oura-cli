use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct CliConfig {
    pub oura_token: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SleepData {
    pub data: Vec<SleepEntry>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SleepEntry {
    pub day: String,
    pub score: u32,
}

#[derive(Serialize, Debug, Clone)]
pub struct SleepScore {
    pub date: String,
    pub score: u32,
}

#[derive(Debug)]
pub enum AppError {
    Config(String),
    Api(String),
    InvalidInput(String),
    Io(String),
}

impl From<reqwest::Error> for AppError {
    fn from(error: reqwest::Error) -> Self {
        AppError::Api(error.to_string())
    }
}

impl From<serde_json::Error> for AppError {
    fn from(error: serde_json::Error) -> Self {
        AppError::Api(format!("JSON parsing error: {}", error))
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::Config(msg) => write!(f, "Configuration error: {}", msg),
            AppError::Api(msg) => write!(f, "API error: {}", msg),
            AppError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            AppError::Io(msg) => write!(f, "I/O error: {}", msg),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_app_error_display() {
        let config_error = AppError::Config("missing token".to_string());
        let api_error = AppError::Api("network failure".to_string());
        let input_error = AppError::InvalidInput("invalid date".to_string());
        let io_error = AppError::Io("file not found".to_string());
        
        assert_eq!(
            format!("{}", config_error),
            "Configuration error: missing token"
        );
        assert_eq!(
            format!("{}", api_error),
            "API error: network failure"
        );
        assert_eq!(
            format!("{}", input_error),
            "Invalid input: invalid date"
        );
        assert_eq!(
            format!("{}", io_error),
            "I/O error: file not found"
        );
    }
    
    #[test]
    fn test_app_error_from_json_error() {
        // Create a serde_json error
        let json_error = serde_json::from_str::<SleepData>("invalid json").unwrap_err();
        let app_error = AppError::from(json_error);
        
        match app_error {
            AppError::Api(msg) => {
                assert!(msg.contains("JSON parsing error"));
            },
            _ => panic!("Expected Api error variant"),
        }
    }
    
    #[test]
    fn test_sleep_score_serialization() {
        let score = SleepScore {
            date: "2023-01-01".to_string(),
            score: 85,
        };
        
        let json = serde_json::to_string(&score).unwrap();
        assert!(json.contains("\"date\":\"2023-01-01\""));
        assert!(json.contains("\"score\":85"));
    }
    
    #[test]
    fn test_sleep_data_deserialization() {
        let json = r#"{"data":[{"day":"2023-01-01","score":85}]}"#;
        let data: SleepData = serde_json::from_str(json).unwrap();
        
        assert_eq!(data.data.len(), 1);
        assert_eq!(data.data[0].day, "2023-01-01");
        assert_eq!(data.data[0].score, 85);
    }
}