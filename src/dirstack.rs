// Module to hold the vector of Strings that encodes the directory stack.
use std::str;

pub struct DirStack<'a> {
    dirstack: Vec<&'a str>
}

impl<'a> DirStack<'a> {
    // new: pass in a str of space separated directories as in environment variable $DIRSTACK
    // and create a DirStack structure.
    pub fn new(dirstack_str: &'a str) -> DirStack<'a> {
        DirStack { dirstack: str::split_whitespace(dirstack_str).collect() }
    }

    pub fn contains(&self, directory: &str) -> bool {
        self.dirstack.contains(&directory)
    }

    pub fn remove(&mut self, directory: &str) -> () {
        self.dirstack.retain(|&dir| dir != directory);
    }

    pub fn to_string(&self) -> String {
        self.dirstack.join(" ")
    }
}

