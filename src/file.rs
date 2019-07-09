use std::io;
use std::fs::File;
use std::io::{BufReader, Read};

pub trait SaveFile {
    fn content(&self) -> String;
    fn file(&self) -> String;
}

pub fn read_file(path: &str) -> io::Result<String> {
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut content = String::new();
    buf_reader.read_to_string(&mut content)?;
    Ok(content)
}