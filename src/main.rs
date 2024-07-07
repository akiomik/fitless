use std::fs;

use fit_file as fit;
use fit_file::fit_file;

/// Called for each record message as it is processed.
fn callback(
    timestamp: u32,
    global_message_num: u16,
    _local_msg_type: u8,
    _message_index: u16,
    fields: Vec<fit_file::FitFieldValue>,
    data: &mut Context,
) {
    if global_message_num == fit::GLOBAL_MSG_NUM_SESSION {
        let msg = fit::FitSessionMsg::new(fields);
        let sport_names = fit::init_sport_name_map();
        let sport_id = msg.sport.unwrap();

        println!("Sport: {}", sport_names.get(&sport_id).unwrap());
    } else if global_message_num == fit::GLOBAL_MSG_NUM_RECORD {
        let msg = fit::FitRecordMsg::new(fields);

        data.num_records_processed += 1;

        println!(
            "Timestamp: {} Latitude: {} Longitude: {}",
            timestamp,
            fit::semicircles_to_degrees(msg.position_lat.unwrap()),
            fit::semicircles_to_degrees(msg.position_long.unwrap())
        );
    } else {
        println!("timestamp: {}", timestamp);
    }
}

struct Context {
    num_records_processed: u16,
}

impl Context {
    pub fn new() -> Self {
        let context = Context {
            num_records_processed: 0,
        };
        context
    }
}

fn main() {
    let file = fs::File::open("files/broken.fit").unwrap();
    let mut reader = std::io::BufReader::new(file);
    let mut context = Context::new();
    fit::read(&mut reader, callback, &mut context).unwrap();
    println!("{} records processed", context.num_records_processed);
}
