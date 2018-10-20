use std::env;
use std::fs;

mod dirstack;

fn main() -> std::io::Result<()> {
    let dirstack_env = env::var("DIRSTACK").expect("Could not get environment variable DIRSTACK");
    let mut dir_stack = dirstack::DirStack::new(dirstack_env.as_str());
    let args: Vec<String> = env::args().collect();

    let file = &args[1];
    let file_type = fs::metadata(&file)?.file_type();

    if file_type.is_dir() {
        if dir_stack.contains(&file) {
            dir_stack.remove(&file);
            env::set_var("DIRSTACK", dir_stack.to_string());
        }
        fs::remove_dir_all(&file)
    } else {
        fs::remove_file(&file)
    }
}

