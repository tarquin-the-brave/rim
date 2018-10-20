use std::env;
use std::fs;

mod dirstack;

fn main() {
    // Get the directory stack for the session.
    let dirstack_env = env::var("DIRSTACK").expect("Could not get environment variable DIRSTACK");
    let mut dir_stack = dirstack::DirStack::new(dirstack_env.as_str());

    // Take every argument as a list of files to delete.
    let args: Vec<String> = env::args().collect();
    let mut files = args.clone();
    files.remove(0);

    for file in files.iter() {
        let file_type = fs::metadata(&file).expect("could not get file type").file_type();

        if file_type.is_dir() {
            if dir_stack.contains(&file) {
                dir_stack.remove(&file);
                env::set_var("DIRSTACK", dir_stack.to_string());
            }
            fs::remove_dir_all(&file);
        } else {
            fs::remove_file(&file);
        }
    }
}

