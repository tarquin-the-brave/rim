use std::env;
use std::fs;

mod dirstack;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = &args[1];
    let file_type = fs::metadata(&file)?.file_type();
    if file_type.is_dir() {
        fs::remove_dir_all(&file);
    } else {
        fs::remove_file(&file);
    }
}

