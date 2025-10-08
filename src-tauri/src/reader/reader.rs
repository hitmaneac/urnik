use std::error::Error;
use std::fmt::{Display, Formatter};
use std::time::Duration;

#[derive(Debug)]
pub enum ReaderError {
    Initialization(String),
    Timeout,
    Device(String),
    Protocol(String),
}

impl Display for ReaderError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ReaderError::Initialization(msg) => write!(f, "Reader initialization failed: {}", msg),
            ReaderError::Timeout => write!(f, "Timed out while waiting for a card"),
            ReaderError::Device(msg) => write!(f, "Reader device error: {}", msg),
            ReaderError::Protocol(msg) => write!(f, "Reader protocol error: {}", msg),
        }
    }
}

impl Error for ReaderError {}

pub trait Reader: Send {
    /// Initializes the reader device.
    fn init(&mut self) -> Result<(), ReaderError>;

    /// Returns the unique identifier (UID) of the current tag.
    fn read_uid(&mut self, timeout: Duration) -> Result<Vec<u8>, ReaderError>;

    /// Gets the type of the reader.
    fn get_reader_type(&self) -> ReaderType;
}

#[derive(Debug, Clone, Copy)]
pub enum ReaderType {
    NFC,
    RFID,
}
