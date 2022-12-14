use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let current_dir = env::current_dir().expect("Directory expected");
    let dest_path = Path::new(&current_dir).join("core.pyi");
    let data = fs::read(&dest_path).expect("File not found!");
    let data_string = "".to_string();
}
