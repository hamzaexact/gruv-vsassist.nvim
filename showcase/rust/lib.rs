use std::collections::HashMap;
use std::sync::Arc;

pub mod config;
pub mod error;
pub mod handler;

pub use config::Config;
pub use error::Error;
pub use handler::Handler;

pub trait Service {
    fn start(&mut self) -> Result<(), Error>;
    fn stop(&mut self) -> Result<(), Error>;
}
