use std::fs;

pub fn read_file(path: &String) -> String {
    fs::read_to_string(path).unwrap_or_else(|_| {
        println!("Error: impossilbe to read file");
        std::process::exit(1);
    })
}
