use std::{fs::File, path::Path};

use anyhow::Result;
use crossterm::terminal;
use fitparser::profile::MesgNum;
use pager::Pager;
use tabled::{
    settings::{peaker::Priority, Settings, Style, Width},
    Table, Tabled,
};

#[derive(Clone, Debug, PartialEq, Eq, Tabled)]
struct Record {
    kind: MesgNum,
    num_fields: usize,
}

pub struct Viewer {}

impl Viewer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn display(&self, filename: &Path) -> Result<()> {
        let mut file = File::open(filename)?;
        let data = fitparser::from_reader(&mut file)?;
        let records = data.iter().map(|d| Record {
            kind: d.kind(),
            num_fields: d.fields().len(),
        });

        let terminal_width = terminal::size()?.0 as usize;
        let table_settings = Settings::default()
            .with(Width::wrap(terminal_width).priority(Priority::max(true)))
            .with(Width::increase(terminal_width));
        let mut table = Table::new(records);
        table.with(Style::modern()).with(table_settings);

        Pager::new().setup();
        println!("{table}");

        Ok(())
    }
}

impl Default for Viewer {
    fn default() -> Self {
        Self {}
    }
}
