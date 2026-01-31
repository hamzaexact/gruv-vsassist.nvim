// This is a comprehensive Rust example showcasing enhanced theme colors
use std::collections::HashMap;
use std::io::{self, Read, Write};

/// A trait for database operations
pub trait Database {
    fn insert(&mut self, key: String, value: String);
    fn retrieve(&self, key: &str) -> Option<&String>;
}

/// A simple in-memory database implementation
#[derive(Debug, Clone)]
pub struct MemoryDatabase {
    store: HashMap<String, String>,
}

impl MemoryDatabase {
    /// Creates a new empty database
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }

    /// Loads data from a file
    pub fn load_from_file(path: &str) -> io::Result<Self> {
        let mut file = std::fs::File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let mut db = Self::new();
        for line in contents.lines() {
            if let Some((key, value)) = line.split_once(':') {
                db.store.insert(key.to_string(), value.to_string());
            }
        }
        Ok(db)
    }

    /// Saves data to a file
    pub fn save_to_file(&self, path: &str) -> io::Result<()> {
        let mut file = std::fs::File::create(path)?;
        for (key, value) in &self.store {
            writeln!(file, "{}:{}", key, value)?;
        }
        Ok(())
    }
}

impl Database for MemoryDatabase {
    fn insert(&mut self, key: String, value: String) {
        self.store.insert(key, value);
    }

    fn retrieve(&self, key: &str) -> Option<&String> {
        self.store.get(key)
    }
}

/// Configuration for the server
#[derive(Debug)]
pub struct ServerConfig {
    pub host: &'static str,
    pub port: u16,
    pub timeout_ms: u64,
    pub max_connections: usize,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1",
            port: 8080,
            timeout_ms: 5000,
            max_connections: 100,
        }
    }
}

/// Enum for database operations
pub enum DbOperation {
    Insert { key: String, value: String },
    Retrieve { key: String },
    Delete { key: String },
    Update { key: String, value: String },
}

impl DbOperation {
    /// Execute the operation on the database
    pub fn execute(&self, db: &mut MemoryDatabase) -> Result<String, String> {
        match self {
            DbOperation::Insert { key, value } => {
                db.insert(key.clone(), value.clone());
                Ok(format!("Inserted: {} = {}", key, value))
            }
            DbOperation::Retrieve { key } => match db.retrieve(key) {
                Some(value) => Ok(format!("Retrieved: {} = {}", key, value)),
                None => Err(format!("Key not found: {}", key)),
            },
            DbOperation::Delete { key } => {
                if db.store.contains_key(key) {
                    db.store.remove(key);
                    Ok(format!("Deleted: {}", key))
                } else {
                    Err(format!("Key not found: {}", key))
                }
            }
            DbOperation::Update { key, value } => {
                if db.store.contains_key(key) {
                    db.store.insert(key.clone(), value.clone());
                    Ok(format!("Updated: {} = {}", key, value))
                } else {
                    Err(format!("Key not found: {}", key))
                }
            }
        }
    }
}

/// Processing function with lifetime annotations
pub fn process_data<'a>(input: &'a str, config: &ServerConfig) -> Result<&'a str, String> {
    const MAX_SIZE: usize = 1024 * 1024;

    if input.len() > MAX_SIZE {
        return Err(format!("Input exceeds max size: {}", MAX_SIZE));
    }

    if config.timeout_ms == 0 {
        return Err("Invalid timeout".to_string());
    }

    Ok(input)
}

/// Macros for debugging
#[macro_export]
macro_rules! debug_print {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        println!($($arg)*);
    };
}

#[macro_export]
macro_rules! assert_db_insert {
    ($db:expr, $key:expr, $value:expr) => {
        $db.insert($key.to_string(), $value.to_string());
        assert!($db.retrieve($key).is_some());
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_database() {
        let mut db = MemoryDatabase::new();
        let key = "username".to_string();
        let value = "admin".to_string();

        db.insert(key.clone(), value.clone());

        match db.retrieve(&key) {
            Some(retrieved_value) => {
                assert_eq!(retrieved_value, &value);
                println!("Test passed: {} = {}", key, retrieved_value);
            }
            None => panic!("Key not found"),
        }
    }

    #[test]
    fn test_server_config() {
        let config = ServerConfig::default();
        assert_eq!(config.port, 8080);
        assert_eq!(config.max_connections, 100);
        debug_print!("Config: {:?}", config);
    }

    #[test]
    fn test_db_operations() {
        let mut db = MemoryDatabase::new();

        let insert_op = DbOperation::Insert {
            key: "test_key".to_string(),
            value: "test_value".to_string(),
        };

        match insert_op.execute(&mut db) {
            Ok(msg) => println!("Operation result: {}", msg),
            Err(e) => panic!("Operation failed: {}", e),
        }
    }
}

fn main() {
    println!("🦀 Rust Theme Showcase");
    println!("======================\n");

    // Numbers with different formats
    let decimal: i32 = 42;
    let hex: u32 = 0xFF_FF_FF_FF;
    let octal: u8 = 0o77;
    let binary: u16 = 0b1111_0000;
    let float: f64 = 3.14159;

    println!("Decimal: {}", decimal);
    println!("Hex: 0x{:X}", hex);
    println!("Float: {:.5}", float);

    // String examples
    let name = "Rust";
    let greeting = format!("Welcome to {}", name);
    let raw_string = r#"Raw string with "quotes" and \backslashes"#;

    println!("{}", greeting);
    println!("{}", raw_string);

    // Database example
    let mut database = MemoryDatabase::new();
    database.insert("language".to_string(), "Rust".to_string());
    database.insert("year".to_string(), "2010".to_string());

    if let Some(value) = database.retrieve("language") {
        println!("Language: {}", value);
    }

    // Configuration
    let config = ServerConfig::default();
    println!("Server running at {}:{}", config.host, config.port);

    // Closures with lifetimes
    let data = vec![1, 2, 3, 4, 5];
    let sum: u32 = data.iter().fold(0, |acc, &x| acc + x);
    println!("Sum: {}", sum);

    // Match statement
    let result = process_data("Hello, Rust!", &config);
    match result {
        Ok(data) => println!("Processed: {}", data),
        Err(e) => eprintln!("Error: {}", e),
    }

    debug_print!("Debug info: {:?}", config);
}
