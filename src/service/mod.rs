use crate::api::OuraClient;
use crate::model::{AppError, SleepScore};
use chrono::NaiveDate;

pub struct SleepScoreService<C: OuraClient> {
    client: C,
}

impl<C: OuraClient> SleepScoreService<C> {
    pub fn new(client: C) -> Self {
        Self { client }
    }

    pub fn get_sleep_scores(&self, start_date: &str, end_date: &str) -> Result<Vec<SleepScore>, AppError> {
        // Validate date format
        self.validate_date_format(start_date)?;
        self.validate_date_format(end_date)?;
        
        // Compare dates to ensure start_date <= end_date
        self.validate_date_range(start_date, end_date)?;

        let sleep_data = self.client.get_sleep_data(start_date, end_date)?;

        let mut sleep_scores: Vec<SleepScore> = sleep_data
            .data
            .into_iter()
            .map(|entry| SleepScore {
                date: entry.day,
                score: entry.score,
            })
            .collect();

        // Sort by date
        sleep_scores.sort_by(|a, b| a.date.cmp(&b.date));

        Ok(sleep_scores)
    }

    fn validate_date_format(&self, date: &str) -> Result<(), AppError> {
        // Use chrono to parse and validate the date format
        NaiveDate::parse_from_str(date, "%Y-%m-%d")
            .map_err(|_| AppError::InvalidInput(format!("Invalid date format: {}. Expected format: YYYY-MM-DD", date)))?;
        Ok(())
    }

    fn validate_date_range(&self, start_date: &str, end_date: &str) -> Result<(), AppError> {
        let start = NaiveDate::parse_from_str(start_date, "%Y-%m-%d")
            .map_err(|_| AppError::InvalidInput(format!("Invalid start date: {}", start_date)))?;
        
        let end = NaiveDate::parse_from_str(end_date, "%Y-%m-%d")
            .map_err(|_| AppError::InvalidInput(format!("Invalid end date: {}", end_date)))?;
        
        if start > end {
            return Err(AppError::InvalidInput(format!(
                "Start date ({}) must be before or equal to end date ({})",
                start_date, end_date
            )));
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::tests::MockOuraClient;
    use crate::model::SleepEntry;

    #[test]
    fn test_get_sleep_scores_sorts_by_date() {
        // Setup mock data with intentionally out-of-order dates
        let entries = vec![
            SleepEntry {
                day: "2023-01-02".to_string(),
                score: 90,
            },
            SleepEntry {
                day: "2023-01-01".to_string(),
                score: 85,
            },
        ];

        let client = MockOuraClient::new(entries);
        let service = SleepScoreService::new(client);

        // Call the service
        let result = service.get_sleep_scores("2023-01-01", "2023-01-02").unwrap();

        // Verify the results are sorted by date
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].date, "2023-01-01");
        assert_eq!(result[0].score, 85);
        assert_eq!(result[1].date, "2023-01-02");
        assert_eq!(result[1].score, 90);
    }

    #[test]
    fn test_validate_date_format_valid() {
        let client = MockOuraClient::new(vec![]);
        let service = SleepScoreService::new(client);

        let result = service.validate_date_format("2023-01-01");
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_date_format_invalid() {
        let client = MockOuraClient::new(vec![]);
        let service = SleepScoreService::new(client);

        let result = service.validate_date_format("01/01/2023");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_date_range_valid() {
        let client = MockOuraClient::new(vec![]);
        let service = SleepScoreService::new(client);

        let result = service.validate_date_range("2023-01-01", "2023-01-02");
        assert!(result.is_ok());

        // Same day is also valid
        let result = service.validate_date_range("2023-01-01", "2023-01-01");
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_date_range_invalid() {
        let client = MockOuraClient::new(vec![]);
        let service = SleepScoreService::new(client);

        let result = service.validate_date_range("2023-01-02", "2023-01-01");
        assert!(result.is_err());
    }
}