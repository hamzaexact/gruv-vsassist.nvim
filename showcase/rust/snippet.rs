use std::collections::HashMap;
use std::error::Error;

pub struct Config {
    pub name: String,
    pub timeout: u32,
}

impl Config {
    pub fn new(name: String, timeout: u32) -> Self {
        Config { name, timeout }
    }

    pub fn validate(&self) -> Result<(), Box<dyn Error>> {
        if self.name.is_empty() {
            return Err("Name cannot be empty".into());
        }
        if self.timeout == 0 {
            return Err("Timeout must be greater than 0".into());
        }
        Ok(())
    }
}

pub enum Status {
    Active,
    Inactive,
    Pending(u32),
}

pub trait Handler {
    fn handle(&self, config: &Config) -> Result<(), Box<dyn Error>>;
}

pub struct DefaultHandler;

impl Handler for DefaultHandler {
    fn handle(&self, config: &Config) -> Result<(), Box<dyn Error>> {
        println!("Handling: {}", config.name);
        config.validate()
    }
}

pub fn process_items<T: Handler>(
    handler: &T,
    configs: Vec<Config>,
) -> HashMap<String, Status> {
    let mut results = HashMap::new();

    for config in configs {
        if let Ok(_) = handler.handle(&config) {
            results.insert(config.name.clone(), Status::Active);
        } else {
            results.insert(config.name.clone(), Status::Inactive);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_validation() {
        let config = Config::new("test".to_string(), 100);
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_invalid_config() {
        let config = Config::new("".to_string(), 0);
        assert!(config.validate().is_err());
    }
}

fn main() {
    let mut configs = vec![
        Config::new("primary".to_string(), 30),
        Config::new("secondary".to_string(), 60),
    ];

    let handler = DefaultHandler;
    let results = process_items(&handler, configs);

    match results.get("primary") {
        Some(Status::Active) => println!("Primary is active"),
        Some(Status::Inactive) => println!("Primary is inactive"),
        Some(Status::Pending(id)) => println!("Primary pending: {}", id),
        None => println!("Primary not found"),
    }
}
