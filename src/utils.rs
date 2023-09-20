pub fn exit(reason: &str, code: i32) {
    println!("Error: {reason}");
    std::process::exit(code);
}
