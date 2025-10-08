use libc::c_ulong;
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::os::unix::io::{AsRawFd, RawFd};
use std::sync::Mutex;
use std::time::{Duration, Instant};
use std::{thread, vec::Vec};

use super::{Reader, ReaderError, ReaderType};

const PN532_I2C_ADDRESS: u16 = 0x24;
const ACK_FRAME: [u8; 6] = [0x00, 0x00, 0xFF, 0x00, 0xFF, 0x00];
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(2);
const I2C_SLAVE_IOCTL: c_ulong = 0x0703;

#[derive(Debug)]
pub struct Pn532Reader {
    device: Mutex<std::fs::File>,
    is_initialized: bool,
}

impl Pn532Reader {
    pub fn new(path: impl Into<String>) -> Result<Self, ReaderError> {
        let path_string: String = path.into();
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(&path_string)
            .map_err(|e| ReaderError::Device(e.to_string()))?;

        configure_slave_address(file.as_raw_fd(), PN532_I2C_ADDRESS)?;

        Ok(Self {
            device: Mutex::new(file),
            is_initialized: false,
        })
    }

    pub fn with_default_path() -> Result<Self, ReaderError> {
        let default_path = std::env::var("PN532_I2C_PATH").unwrap_or_else(|_| String::from("/dev/i2c-1"));
        Self::new(default_path)
    }

    fn wakeup(&self) -> Result<(), ReaderError> {
        let mut guard = self
            .device
            .lock()
            .map_err(|_| ReaderError::Device("Failed to lock PN532 device".into()))?;
        let wake_sequence = [0x00u8; 1];
        guard
            .write(&wake_sequence)
            .map_err(|e| ReaderError::Device(e.to_string()))?;
        thread::sleep(Duration::from_millis(20));
        Ok(())
    }

    fn send_command(&self, command: u8, data: &[u8]) -> Result<(), ReaderError> {
        let mut guard = self
            .device
            .lock()
            .map_err(|_| ReaderError::Device("Failed to lock PN532 device".into()))?;

        let len = (1 + data.len()) as u8; // command + payload
        let lcs = (!len).wrapping_add(1);
        let mut frame: Vec<u8> = Vec::with_capacity(8 + data.len());
        frame.push(0x00); // Host to PN532 prefix for I2C
        frame.push(0x00);
        frame.push(0x00);
        frame.push(0xFF);
        frame.push(len);
        frame.push(lcs);
        frame.push(0xD4); // TFI host to PN532
        frame.push(command);
        frame.extend_from_slice(data);

        let mut checksum: u8 = 0xD4;
        checksum = checksum.wrapping_add(command);
        for byte in data {
            checksum = checksum.wrapping_add(*byte);
        }
        let dcs = (!checksum).wrapping_add(1);
        frame.push(dcs);
        frame.push(0x00); // Postamble

        guard
            .write(&frame)
            .map_err(|e| ReaderError::Device(e.to_string()))?;
        Ok(())
    }

    fn read_ack(&self, timeout: Duration) -> Result<(), ReaderError> {
        let mut guard = self
            .device
            .lock()
            .map_err(|_| ReaderError::Device("Failed to lock PN532 device".into()))?;

        let start = Instant::now();
        let mut status = [0u8; 1];
        while start.elapsed() < timeout {
            guard
                .read_exact(&mut status)
                .map_err(|e| ReaderError::Device(e.to_string()))?;
            if status[0] == 0x01 {
                let mut ack = [0u8; 6];
                guard
                    .read_exact(&mut ack)
                    .map_err(|e| ReaderError::Device(e.to_string()))?;
                if ack == ACK_FRAME {
                    return Ok(());
                } else {
                    return Err(ReaderError::Protocol(format!(
                        "Unexpected ACK frame: {:02x?}",
                        ack
                    )));
                }
            }
            thread::sleep(Duration::from_millis(5));
        }
        Err(ReaderError::Timeout)
    }

    fn read_response(&self, expected: u8, timeout: Duration) -> Result<Vec<u8>, ReaderError> {
        let mut guard = self
            .device
            .lock()
            .map_err(|_| ReaderError::Device("Failed to lock PN532 device".into()))?;

        let start = Instant::now();
        let mut status = [0u8; 1];
        while start.elapsed() < timeout {
            guard
                .read_exact(&mut status)
                .map_err(|e| ReaderError::Device(e.to_string()))?;
            if status[0] != 0x01 {
                thread::sleep(Duration::from_millis(5));
                continue;
            }

            let mut header = [0u8; 5];
            guard
                .read_exact(&mut header)
                .map_err(|e| ReaderError::Device(e.to_string()))?;
            if header[0] != 0x00 || header[1] != 0x00 || header[2] != 0xFF {
                return Err(ReaderError::Protocol(format!(
                    "Unexpected response header: {:02x?}",
                    header
                )));
            }
            let len = header[3] as usize;
            let lcs = header[4];
            if len.wrapping_add(lcs as usize) & 0xFF != 0 {
                return Err(ReaderError::Protocol("Invalid length checksum".into()));
            }

            let mut payload = vec![0u8; len + 2];
            guard
                .read_exact(&mut payload)
                .map_err(|e| ReaderError::Device(e.to_string()))?;

            let tfi = payload[0];
            if tfi != 0xD5 {
                return Err(ReaderError::Protocol(format!(
                    "Unexpected frame identifier: {:02x}",
                    tfi
                )));
            }
            if payload[1] != expected + 1 {
                return Err(ReaderError::Protocol(format!(
                    "Unexpected response code: {:02x}",
                    payload[1]
                )));
            }

            let data = &payload[2..len];
            let dcs = payload[len];
            let postamble = payload[len + 1];

            let mut checksum: u8 = 0xD5;
            checksum = checksum.wrapping_add(payload[1]);
            for byte in data {
                checksum = checksum.wrapping_add(*byte);
            }
            if dcs != (!checksum).wrapping_add(1) {
                return Err(ReaderError::Protocol("Invalid data checksum".into()));
            }
            if postamble != 0x00 {
                return Err(ReaderError::Protocol("Missing postamble".into()));
            }

            return Ok(data.to_vec());
        }
        Err(ReaderError::Timeout)
    }

    fn configure_sam(&self) -> Result<(), ReaderError> {
        self.send_command(0x14, &[0x01, 0x14, 0x01])?;
        self.read_ack(DEFAULT_TIMEOUT)?;
        let _ = self.read_response(0x14, DEFAULT_TIMEOUT)?;
        Ok(())
    }

    fn in_list_passive_target(&self, timeout: Duration) -> Result<Option<Vec<u8>>, ReaderError> {
        self.send_command(0x4A, &[0x01, 0x00])?;
        self.read_ack(DEFAULT_TIMEOUT)?;
        let response = self.read_response(0x4A, timeout)?;
        if response.is_empty() {
            return Ok(None);
        }
        let tags_found = response[0];
        if tags_found == 0 {
            return Ok(None);
        }
        if response.len() < 6 {
            return Err(ReaderError::Protocol("Incomplete InListPassiveTarget response".into()));
        }
        let uid_length = response[5] as usize;
        if response.len() < 6 + uid_length {
            return Err(ReaderError::Protocol("Invalid UID length in response".into()));
        }
        Ok(Some(response[6..6 + uid_length].to_vec()))
    }
}

impl Reader for Pn532Reader {
    fn init(&mut self) -> Result<(), ReaderError> {
        if self.is_initialized {
            return Ok(());
        }
        self.wakeup()?;
        self.configure_sam()?;
        self.is_initialized = true;
        Ok(())
    }

    fn read_uid(&mut self, timeout: Duration) -> Result<Vec<u8>, ReaderError> {
        self.init()?;
        let start = Instant::now();
        while start.elapsed() < timeout {
            match self.in_list_passive_target(Duration::from_millis(500))? {
                Some(uid) => return Ok(uid),
                None => thread::sleep(Duration::from_millis(100)),
            }
        }
        Err(ReaderError::Timeout)
    }

    fn get_reader_type(&self) -> ReaderType {
        ReaderType::NFC
    }
}

fn configure_slave_address(fd: RawFd, address: u16) -> Result<(), ReaderError> {
    let result = unsafe { libc::ioctl(fd, I2C_SLAVE_IOCTL, address as c_ulong) };
    if result < 0 {
        return Err(ReaderError::Device(format!(
            "Failed to set I2C slave address: {}",
            std::io::Error::last_os_error()
        )));
    }
    Ok(())
}

