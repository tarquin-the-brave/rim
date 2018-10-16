use std::env;
use std::fs;

mod dirstack;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let file = &args[1];
    let file_type = fs::metadata(&file)?.file_type();
    if file_type.is_dir() {
        fs::remove_dir_all(&file)
        // delete from $DIRSTACK if needed.
    } else {
        fs::remove_file(&file)
    }
}

