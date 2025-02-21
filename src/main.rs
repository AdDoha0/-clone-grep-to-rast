use std::env;

use my_project::{Config, run};
// cargo run -- test poem.txt
fn main() {
    let args: Vec<String> = env::args().collect();

    // let config = Config::build(&args).unwrap_or_else(|err| {
    //     println!("Problem parsing arguments: {err}");
    //     process::exit(1);
    // });

    let config = match Config::build(&args) {
        Ok(config) => config,
        Err(err) => {
            eprintln!("Problem parsing arguments: {}", err);
            std::process::exit(1);
        }
    };

    if let Err(e) = run(config) {
        eprintln!("Error reading file: {}", e);
        std::process::exit(1);
    }
}