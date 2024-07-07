use std::fs;
use std::path::Path;

use anyhow::Result;

pub struct InvalidCrcFixer {}

impl InvalidCrcFixer {
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn fix(&self, filename: &Path, expected: u16) -> Result<()> {
        println!("Fixing an invalid header CRC...");

        let mut file = fs::read(filename)?;
        let file_size = file.len();

        if file[0] == 14 {
            // Fixes header crc
            for (i, byte) in 0_u16.to_le_bytes().iter().enumerate() {
                file[12 + i] = *byte;
            }
        }

        // Fixes file crc
        for (i, byte) in expected.to_le_bytes().iter().enumerate() {
            file[file_size - 2 + i] = *byte;
            // file[14 + i] = *byte;
        }

        fs::write(filename, file)?;

        println!("An invalid header CRC has been fixed.");

        Ok(())
    }
}

impl Default for InvalidCrcFixer {
    fn default() -> Self {
        Self {}
    }
}
