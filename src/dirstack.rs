use std::str;

pub struct DirStack {
    dirstack: Vec<String>
}

impl DirStack {
    pub fn new(dirstack_str: String) -> DirStack {
        DirStack { dirstack: str::split_whitespace(&dirstack_str).collect() }
    }
}

