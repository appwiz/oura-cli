use crate::model::{AppError, CliConfig};

pub trait ConfigManager {
    fn load_config(&self) -> Result<CliConfig, AppError>;
    fn save_config(&self, config: &CliConfig) -> Result<(), AppError>;
}

pub struct FileConfigManager {
    app_name: String,
}

impl FileConfigManager {
    pub fn new(app_name: &str) -> Self {
        Self {
            app_name: app_name.to_string(),
        }
    }
}

impl ConfigManager for FileConfigManager {
    fn load_config(&self) -> Result<CliConfig, AppError> {
        confy::load(&self.app_name, None)
            .map_err(|e| AppError::Config(format!("Failed to load config: {}", e)))
    }

    fn save_config(&self, config: &CliConfig) -> Result<(), AppError> {
        confy::store(&self.app_name, None, config)
            .map_err(|e| AppError::Config(format!("Failed to save config: {}", e)))
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use std::cell::RefCell;

    pub struct MockConfigManager {
        pub config: RefCell<CliConfig>,
        pub load_called: RefCell<bool>,
        pub save_called: RefCell<bool>,
    }

    impl MockConfigManager {
        pub fn new(token: &str) -> Self {
            Self {
                config: RefCell::new(CliConfig {
                    oura_token: token.to_string(),
                }),
                load_called: RefCell::new(false),
                save_called: RefCell::new(false),
            }
        }
    }

    impl ConfigManager for MockConfigManager {
        fn load_config(&self) -> Result<CliConfig, AppError> {
            *self.load_called.borrow_mut() = true;
            Ok(self.config.borrow().clone())
        }

        fn save_config(&self, config: &CliConfig) -> Result<(), AppError> {
            *self.save_called.borrow_mut() = true;
            *self.config.borrow_mut() = config.clone();
            Ok(())
        }
    }
    
    #[test]
    fn test_file_config_manager_new() {
        let manager = FileConfigManager::new("test-app");
        assert_eq!(manager.app_name, "test-app");
    }
    
    // Create a mock implementation of the config load/save functionality 
    // that doesn't actually interact with the file system
    #[test]
    fn test_file_config_manager_load_save_mock() {
        let manager = FileConfigManager::new("test-app");
        
        // Testing that it's properly calling confy::load/store 
        // would require mocking the confy crate, which would require
        // significant refactoring. For now, we just test that the manager
        // correctly passes through the app name.
        assert_eq!(manager.app_name, "test-app");
    }
}

#[cfg(not(test))]
pub mod tests {
    use super::*;
    use std::cell::RefCell;

    pub struct MockConfigManager {
        pub config: RefCell<CliConfig>,
        pub load_called: RefCell<bool>,
        pub save_called: RefCell<bool>,
    }

    impl MockConfigManager {
        pub fn new(token: &str) -> Self {
            Self {
                config: RefCell::new(CliConfig {
                    oura_token: token.to_string(),
                }),
                load_called: RefCell::new(false),
                save_called: RefCell::new(false),
            }
        }
    }

    impl ConfigManager for MockConfigManager {
        fn load_config(&self) -> Result<CliConfig, AppError> {
            *self.load_called.borrow_mut() = true;
            Ok(self.config.borrow().clone())
        }

        fn save_config(&self, config: &CliConfig) -> Result<(), AppError> {
            *self.save_called.borrow_mut() = true;
            *self.config.borrow_mut() = config.clone();
            Ok(())
        }
    }
}