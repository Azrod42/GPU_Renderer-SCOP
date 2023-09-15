extern crate obj_parser;

fn main() {
    let number = obj_parser::file::readfile(3, 942);

    println!("Hello, world! number is {number}");
}
