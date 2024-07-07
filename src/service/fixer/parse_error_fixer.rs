use std::error::Error;
use std::fs;
use std::path::Path;

use fit_rust::Fit;

use super::data_size_fixer::DataSizeFixer;

pub struct ParseErrorFixer {}

impl ParseErrorFixer {
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn fix(&self, filename: &Path) -> Result<(), Box<dyn Error>> {
        println!("Fixing a parse error...");

        // Check file size
        let file = fs::read(filename)?;
        let file_size = file.len();
        let fit = Fit::read(file)?;

        let actual_data_size = fit.header.data_size as usize;
        let expected_data_size = file_size - fit.header.header_size as usize;
        if actual_data_size != expected_data_size {
            println!(
                "A data size is broken. Expected: {}, Actual: {}",
                expected_data_size, actual_data_size
            );

            // TODO: Check expected_data_size must be within u8 range
            let data_size_fixer = DataSizeFixer::new();
            data_size_fixer.fix(filename, expected_data_size as u8)?;
        }

        println!("A parse error has been fixed.");

        Ok(())
    }
}

impl Default for ParseErrorFixer {
    fn default() -> Self {
        Self {}
    }
}
