#![allow(unused_imports)]
use std::env;
use std::fmt;
use std::io;
use std::io::Write;
use std::str;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub struct Snapshot {
    pub file_name: String,
    buf: String,
}

#[derive(Debug)]
pub struct ExpectErr {
    result: String,
    expectation: String,
}

impl Error for ExpectErr {
    fn description(&self) -> &str {
        "Something bad happened"
    }
}

impl fmt::Display for ExpectErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Oh no, something bad went down")
    }
}

impl Snapshot {
    pub fn new(file_name: String) -> Snapshot {
        let buf = String::new();
        Snapshot { file_name, buf }
    }

    pub fn push(&mut self, text: &[u8]) {
        let t = str::from_utf8(text).unwrap();
        self.buf.push_str(t);
    }

    fn expectation(&self) -> String {
        let mut contents = String::new();
        let path = env::current_dir()
            .unwrap()
            .join("expect")
            .join(&self.file_name);
        let file = File::open(path);
        match file {
            Ok(mut f) => f.read_to_string(&mut contents).unwrap(),
            Err(_f) => 0,
        };
        contents
    }

    pub fn finish(&self) -> std::result::Result<(), ExpectErr> {
        let expectation = self.expectation();
        let result = self.buf.clone();
        if result == expectation {
            Ok(())
        } else {
            Err(ExpectErr {
                result,
                expectation,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_snapshot() {
        let _s = Snapshot::new(String::from("test1"));
    }
    #[test]
    fn write_to_snapshot_incorrect() {
        let mut s = Snapshot::new(String::from("test1"));
        s.push(&"x".as_bytes());
        assert!(s.finish().is_err());
    }
    #[test]
    fn write_to_snapshot_correct() {
        let mut s = Snapshot::new(String::from("test1"));
        s.push(&"hello".as_bytes());
        assert!(s.finish().is_ok());
    }
}
