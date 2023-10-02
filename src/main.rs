use crate::window::init_window;

extern crate gl;
extern crate glfw;
extern crate obj_parser;
mod init;
mod shader;
mod utils;
mod window;

fn main() {
    let file_path = init::parse_argv();
    let obj_content = obj_parser::file::read_file(&file_path);
    let data = obj_parser::parse::pars_obj(&obj_content, &file_path);
    println!("{:#?}\n{:#?}\n", data.0, data.1);
    init_window();
}
