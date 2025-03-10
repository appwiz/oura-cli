use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use std::fmt::Debug;

use crate::model::{AppError, SleepData};

pub trait OuraClient {
    fn get_sleep_data(&self, start_date: &str, end_date: &str) -> Result<SleepData, AppError>;
}

#[derive(Debug)]
pub struct HttpOuraClient {
    client: Client,
    token: String,
    base_url: String,
}

impl HttpOuraClient {
    pub fn new(token: &str, base_url: &str) -> Self {
        Self {
            client: Client::new(),
            token: token.to_string(),
            base_url: base_url.to_string(),
        }
    }

    fn build_auth_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        let auth_value = format!("Bearer {}", self.token);
        if let Ok(header_value) = HeaderValue::from_str(&auth_value) {
            headers.insert(AUTHORIZATION, header_value);
        }
        headers
    }
}

impl OuraClient for HttpOuraClient {
    fn get_sleep_data(&self, start_date: &str, end_date: &str) -> Result<SleepData, AppError> {
        if self.token.is_empty() {
            return Err(AppError::Config("Oura token is missing in the configuration".into()));
        }

        let url = format!(
            "{}/usercollection/daily_sleep?start_date={}&end_date={}",
            self.base_url, start_date, end_date
        );

        let response = self.client
            .get(&url)
            .headers(self.build_auth_headers())
            .send()?;

        let response_text = response.text()?;
        let sleep_data: SleepData = serde_json::from_str(&response_text)?;

        Ok(sleep_data)
    }
}

// Mocks used for unit tests
#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::model::{SleepData, SleepEntry};
    use std::cell::RefCell;

    pub struct MockOuraClient {
        pub sleep_data: RefCell<SleepData>,
        pub call_count: RefCell<usize>,
    }

    impl MockOuraClient {
        pub fn new(data: Vec<SleepEntry>) -> Self {
            Self {
                sleep_data: RefCell::new(SleepData { data }),
                call_count: RefCell::new(0),
            }
        }
    }

    impl OuraClient for MockOuraClient {
        fn get_sleep_data(&self, _start_date: &str, _end_date: &str) -> Result<SleepData, AppError> {
            *self.call_count.borrow_mut() += 1;
            let data = self.sleep_data.borrow().clone();
            Ok(data)
        }
    }
    
    #[test]
    fn test_http_client_empty_token() {
        let client = HttpOuraClient::new("", "https://test.com");
        let result = client.get_sleep_data("2023-01-01", "2023-01-02");
        assert!(result.is_err());
        if let Err(AppError::Config(msg)) = result {
            assert!(msg.contains("missing"));
        } else {
            panic!("Expected Config error variant");
        }
    }
    
    #[test]
    fn test_build_auth_headers() {
        let client = HttpOuraClient::new("test_token", "https://test.com");
        let headers = client.build_auth_headers();
        
        assert!(headers.contains_key(AUTHORIZATION));
        if let Some(value) = headers.get(AUTHORIZATION) {
            assert_eq!(value.to_str().unwrap(), "Bearer test_token");
        } else {
            panic!("Authorization header not found");
        }
    }
    
    #[test]
    fn test_invalid_header_value_handled() {
        // Create a client with an invalid token (contains invalid header chars)
        let client = HttpOuraClient::new("invalid\ntoken\r", "https://test.com");
        
        // Ensure it doesn't panic when creating headers
        let headers = client.build_auth_headers();
        
        // The header should either not be present or be handled gracefully
        if headers.contains_key(AUTHORIZATION) {
            // If it's present, ensure it's valid
            headers.get(AUTHORIZATION).unwrap().to_str().unwrap_err();
        }
    }
}

// Publicly exposed mock for integration tests
#[cfg(not(test))]
pub mod tests {
    use super::*;
    use crate::model::{SleepData, SleepEntry};
    use std::cell::RefCell;

    pub struct MockOuraClient {
        pub sleep_data: RefCell<SleepData>,
        pub call_count: RefCell<usize>,
    }

    impl MockOuraClient {
        pub fn new(data: Vec<SleepEntry>) -> Self {
            Self {
                sleep_data: RefCell::new(SleepData { data }),
                call_count: RefCell::new(0),
            }
        }
    }

    impl OuraClient for MockOuraClient {
        fn get_sleep_data(&self, _start_date: &str, _end_date: &str) -> Result<SleepData, AppError> {
            *self.call_count.borrow_mut() += 1;
            let data = self.sleep_data.borrow().clone();
            Ok(data)
        }
    }
}