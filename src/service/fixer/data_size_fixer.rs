use std::error::Error;
use std::fs;
use std::path::Path;

use fit_rust::Fit;

pub struct DataSizeFixer {}

impl DataSizeFixer {
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn fix(&self, filename: &Path, expected_size: u8) -> Result<(), Box<dyn Error>> {
        println!("Fixing a data size...");

        let file = fs::read(filename)?;
        let mut fit = Fit::read(file)?;
        let mut header = fit.header;
        header.header_size = expected_size;
        fit.header = header;
        fit.write(filename)?;

        println!("A data size has been fixed.");

        Ok(())
    }
}

impl Default for DataSizeFixer {
    fn default() -> Self {
        Self {}
    }
}
