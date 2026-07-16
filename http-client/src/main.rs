mod request;

use std::env::args;
use request::request;

static USAGE: &str = "\x1b[4;32mUsage:\x1b[0m http [URI]";

fn main() {
    
    let args: Vec<String> = args().skip(1).collect();

    if args.is_empty() {
        println!("{}", USAGE);
    }

    let url = args.get(0).unwrap();

    request(url);
}
