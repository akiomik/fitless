use std::fs;
use std::path::Path;

use anyhow::Result;

pub struct DataSizeFixer {}

impl DataSizeFixer {
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn fix(&self, filename: &Path, expected_size: u32) -> Result<()> {
        println!("Fixing a data size...");

        let mut file = fs::read(filename)?;
        for (i, byte) in expected_size.to_le_bytes().iter().enumerate() {
            file[4 + i] = *byte;
        }
        fs::write(filename, file)?;

        println!("A data size has been fixed.");

        Ok(())
    }
}

impl Default for DataSizeFixer {
    fn default() -> Self {
        Self {}
    }
}
