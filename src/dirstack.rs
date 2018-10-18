// Module to hold the vector of Strings that encodes the directory stack.
use std::str;

pub struct DirStack {
    dirstack: Vec<&str>
}

impl DirStack {
    // new: pass is a string of space separated directories as in environment variable $DIRSTACK
    // and create a DirStack structure.
    pub fn new(dirstack_str: String) -> DirStack {
        DirStack { dirstack: str::split_whitespace(&dirstack_str).collect() }
    }
}

