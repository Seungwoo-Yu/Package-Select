use std::error::Error;

#[derive(Debug)]
pub enum CommandError {
    None,
    String(String),
    Others(Box<dyn Error>)
}
