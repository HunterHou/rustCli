use std::fmt::Debug;
use std::time::Instant;
use std::time;

#[derive(Debug)]
pub struct MFile {
    pub name: String,
    pub ext: String,
    pub size: u64,

    pub ctimes: String,
    pub mtimes: String,
    ctime: Instant,
    mtime: Instant,
}

impl MFile {
    pub fn new() -> MFile {
        let now = Instant::now();
        MFile {
            name: String::from(""),
            ext: String::from(""),
            size: 0,
            ctimes: String::from(""),
            mtimes: String::from(""),
            ctime: now,
            mtime: now,
        }
    }
    pub fn getCTime(&self)->String{
    }
}
