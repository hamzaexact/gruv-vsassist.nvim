use std::collections::HashMap;
use std::sync::Arc;

pub mod config;
pub mod handler;
pub mod error;

pub use config::Config;
pub use handler::Handler;
pub use error::Error;

pub trait Service {
    fn start(&mut self) -> Result<(), Error>;
    fn stop(&mut self) -> Result<(), Error>;
}
