use std::error::Error;
use std::fs::File;

use fitparser::de::{from_reader_with_options, DecodeOption};
use fitparser::ErrorKind;

fn main() -> Result<(), Box<dyn Error>> {
    println!(
        "Parsing FIT files using Profile version: {}",
        fitparser::profile::VERSION
    );
    let mut fp = File::open("files/broken-fixed.fit")?;

    let opts = [
        DecodeOption::SkipHeaderCrcValidation,
        DecodeOption::SkipDataCrcValidation,
        DecodeOption::DropUnknownFields,
        DecodeOption::DropUnknownMessages,
    ]
    .iter()
    .map(|o| *o)
    .collect();

    match from_reader_with_options(&mut fp, &opts) {
        Ok(records) => {
            for data in records {
                println!("{:#?}", data);
            }
        }
        Err(e) => match *e {
            ErrorKind::ParseError(_, t) => println!("{}", t.description()),
            _ => println!("error!: {:?}", e),
        },
    };

    Ok(())
}
