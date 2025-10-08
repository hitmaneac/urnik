use std::time::Duration;

use super::{Reader, ReaderError, ReaderType};

pub struct Rdm6300;

impl Reader for Rdm6300 {
    fn init(&mut self) -> Result<(), ReaderError> {
        Err(ReaderError::Initialization(
            "RDM6300 reader is not implemented".into(),
        ))
    }

    fn read_uid(&mut self, _timeout: Duration) -> Result<Vec<u8>, ReaderError> {
        Err(ReaderError::Device(
            "RDM6300 reader is not available in this build".into(),
        ))
    }

    fn get_reader_type(&self) -> ReaderType {
        ReaderType::RFID
    }
}
