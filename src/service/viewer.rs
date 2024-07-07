use std::{fs::File, path::Path};

use anyhow::Result;

pub struct Viewer {}

impl Viewer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn display(&self, filename: &Path) -> Result<String> {
        let mut file = File::open(filename)?;
        let data = fitparser::from_reader(&mut file)?;
        Ok(format!("{:?}", data))
    }
}

impl Default for Viewer {
    fn default() -> Self {
        Self {}
    }
}
