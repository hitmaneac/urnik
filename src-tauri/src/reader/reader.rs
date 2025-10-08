pub trait Reader {
    /// Initializes the reader device.
    fn init(&mut self) -> Result<(), Box<dyn Error>>;

    /// Returns the unique identifier (UID) of the current tag.
    fn read_uid(&mut self, timeout: i32) -> Result<Vec<u8>, Box<dyn Error>>;

    /// Gets the type of the reader.
    fn get_reader_type(&self) -> ReaderType;
}

pub enum ReaderType {
    NFC,
    RFID,
}
