extern crate io_12;

use std::process;
fn main() {
    let c = io_12::parse_args().unwrap_or_else(|err| {
        println!("Problem parsing arguments : {}", err);
        process::exit(1);
    });
    c.print_config();
}
