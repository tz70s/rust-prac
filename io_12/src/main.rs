extern crate io_12;

use std::process;

fn main() {
    let c = io_12::parse_args().unwrap_or_else(|err| {
        println!("Problem parsing arguments : {}", err);
        process::exit(1);
    });
    c.print_config();
    run(c);
}

use io_12::Config;
use std::io::prelude::*;
use std::fs::File;

fn run(config: Config) {
    // open file
    let mut f = File::open(config.content).expect("unable to open the file");
    let mut content = String::new();

    f.read_to_string(&mut content).expect(
        "failed to read string",
    );
    println!("text: {}", content);
}
