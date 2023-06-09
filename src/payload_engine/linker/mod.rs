use std::fs::File;
use std::io::{BufReader, ErrorKind};

pub struct Linker<'a> {
    entry_point : &'a str
}

impl<'a> Linker<'a> {
    pub fn new(entry_point : &str) -> Linker {

        return Linker {
            entry_point
        }
    }

    pub fn get_buf(&self) -> BufReader<File> {
        let location = "dfsdf";
        let entry = File::open(self.entry_point)
            .expect(format!("Entry point {location} could not be read").as_str());

        //entry.

        return BufReader::new(entry);
    }
}

