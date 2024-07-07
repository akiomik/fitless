use fit::Fit;
use std::path::PathBuf;

fn main() {
    let filepath = PathBuf::from("files/broken.fit");
    let f = Fit::new(&filepath);
    for m in f {
        println!("Read a message of type {:?}", m);
    }
}
