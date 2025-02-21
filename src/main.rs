use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // args[0] это имя програмы
    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {query}");
    println!("In file {file_path}");
}
