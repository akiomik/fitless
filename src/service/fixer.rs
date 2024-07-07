use std::{error::Error, path::PathBuf};

pub struct Fixer;

impl Fixer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn fix(&self, filename: &PathBuf) -> Result<(), Box<dyn Error>> {
        println!("Fix {:?}", filename);

        Ok(())
    }
}

impl Default for Fixer {
    fn default() -> Self {
        Self {}
    }
}
