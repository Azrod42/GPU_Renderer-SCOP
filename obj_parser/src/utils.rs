use std::process;

pub fn check_rgb(rgb: &(f32, f32, f32)) -> Result<bool, String> {
    if rgb.0 < 0.0 || rgb.0 > 1.0 || rgb.1 < 0.0 || rgb.1 > 1.0 || rgb.2 < 0.0 || rgb.2 > 1.0 {
        return Err(String::from("invalid rgb on mtl file"));
    }
    Ok(true)
}

pub fn exit(reason: &str, id: i32) {
    println!("Error: {reason}");
    process::exit(id);
}
