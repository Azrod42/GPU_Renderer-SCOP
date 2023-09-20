extern crate obj_parser;

mod init;
mod utils;

fn main() {
    let file_path = init::parse_argv();
    let obj_content = obj_parser::file::read_file(&file_path);
    let data = obj_parser::parse::pars_obj(&obj_content, &file_path);
    println!("{:#?}\n{:#?}\n", data.0.unwrap(), data.1);
}
