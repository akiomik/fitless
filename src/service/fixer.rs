mod data_size_fixer;
mod invalid_crc_fixer;
mod parse_error_fixer;

use std::fs::File;
use std::path::Path;

use anyhow::Result;
use fitparser;
use fitparser::de::DecodeOption;
use fitparser::ErrorKind;

use invalid_crc_fixer::InvalidCrcFixer;
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

        // TODO: Fix the file size first to detect CRC errors correctly
        let mut opts = vec![
            DecodeOption::SkipHeaderCrcValidation,
            DecodeOption::SkipDataCrcValidation,
        ]
        .iter()
        .map(|o| *o)
        .collect();

        // Detect infinite loop
        let mut is_invalid_crc_fixed = false;
        let mut is_parse_error_fixed = false;

        loop {
            match fitparser::de::from_reader_with_options(&mut file, &opts) {
                Ok(_) => {
                    println!("{:?} is valid. Nothing to do.", filename);
                    break;
                }
                Err(e) => {
                    match *e {
                        ErrorKind::InvalidCrc((_data, _fit, actual, expected)) => {
                            if is_invalid_crc_fixed {
                                println!("A crc has been fixed, but still invalid. Expected: {expected}, Actual: {actual}");
                                break;
                            }

                            println!("A crc is invalid. Expected: {expected}, Actual: {actual}");

                            let invalid_crc_fixer = InvalidCrcFixer::new();
                            invalid_crc_fixer.fix(filename, expected)?;

                            is_invalid_crc_fixed = true;
                        }
                        ErrorKind::ParseError(pos, _) => {
                            if is_parse_error_fixed {
                                println!(
                                    "A parse error has been fixed, but something wrong. Pos: {pos}"
                                );
                                break;
                            }

                            println!("A file format is invalid. Pos: {pos}");

                            let parse_error_fixer = ParseErrorFixer::new();
                            parse_error_fixer.fix(filename)?;

                            // Enable crc validation after fixing parse error
                            opts.clear();

                            is_parse_error_fixed = true;
                        }
                        _ => {
                            // TODO
                            println!("{e}");
                            break;
                        }
                    }
                }
            }
        }

        // TODO: Add missing acitivity

        println!("All errors have been fixed.");

        Ok(())
    }
}

impl Default for Fixer {
    fn default() -> Self {
        Self {}
    }
}
