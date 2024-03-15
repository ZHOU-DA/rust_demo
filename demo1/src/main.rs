use demo1::Config;
use std::env::args;
use std::process;

fn main() {
    let args_vec: Vec<String> = args().collect();

    let config = Config::new(&args_vec).unwrap_or_else(|err: &str| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = demo1::run(config) {
        println!("Problem parsing arguments: {}", e);
        process::exit(1);
    };
}
