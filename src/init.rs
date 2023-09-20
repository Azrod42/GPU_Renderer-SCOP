use crate::utils::exit;
use std::env;

//READ ARGS AND RETURN FIRST ARG IS .obj
pub fn parse_argv() -> String {
    let argv: Vec<String> = env::args().skip(1).collect();

    if argv.len() != 1 {
        exit("Usage : cargo run ./path-to-obj", 1);
    }

    let file_provide = argv.first().unwrap();

    let mut argv: Vec<&str> = argv.first().unwrap().split(".").collect();
    match argv.pop().unwrap() {
        "obj" => {}
        _ => exit("Worng file format please use .obj", 1),
    }
    file_provide.to_string()
}
