use reader::{Reader, ReaderType};

pub struct RDM6300 {}

impl Reader for RDM6300 {
    fn init(&mut self) -> Result<(), Box<dyn Error>> {
        // Initialization logic for RDM6300
        Ok(())
    }

    fn read_uid(&mut self, timeout: i32) -> Result<Vec<u8>, Box<dyn Error>> {
        // Logic to read UID from RDM6300
        Ok(vec![0x00, 0x01, 0x02, 0x03]) // Example UID
    }

    fn get_reader_type(&self) -> ReaderType {
        ReaderType::RFID
    }
}
