mod data_size_fixer;
mod parse_error_fixer;

use std::fs::File;
use std::path::Path;

use anyhow::Result;
use fitparser;
use fitparser::de::DecodeOption;
use fitparser::ErrorKind;

use parse_error_fixer::ParseErrorFixer;

pub struct Fixer;

impl Fixer {
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn fix(&self, filename: &Path) -> Result<()> {
        println!("Tring to fix {:?}...", filename);

        let mut file = File::open(filename)?;

        let opts = [
            DecodeOption::SkipHeaderCrcValidation,
            DecodeOption::SkipDataCrcValidation,
        ]
        .iter()
        .map(|o| *o)
        .collect();

        match fitparser::de::from_reader_with_options(&mut file, &opts) {
            Ok(_) => println!("{:?} is valid. Nothing to do.", filename),
            Err(e) => match *e {
                // TODO
                // ErrorKind::InvalidCrc((_vec, _fit, _expected, _actual)) => {
                //     println!("Fixing CRC...");
                // }
                ErrorKind::ParseError(_, _) => {
                    let parse_error_fixer = ParseErrorFixer::new();
                    parse_error_fixer.fix(filename)?
                }
                _ => {
                    // TODO
                    println!("{e}")
                }
            },
        }

        println!("All errors have been fixed.");

        Ok(())
    }
}

impl Default for Fixer {
    fn default() -> Self {
        Self {}
    }
}
