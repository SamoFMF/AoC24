use std::{fs, io};
use std::fs::{File, OpenOptions};
use std::io::{ErrorKind, Write};
use std::path::Path;

#[allow(dead_code)]
pub fn read_file(file_name: &str) -> Result<String, io::Error> {
    fs::read_to_string(file_name)
}

#[allow(dead_code)]
pub fn read_input(dir: &str, day: u8) -> Result<String, io::Error> {
    let file_name = format!("{}/input{:02}.txt", dir, day);
    read_file(&file_name)
}

#[allow(dead_code)]
pub fn write_to_file(file_name: &str, content: &str) -> Result<(), io::Error> {
    let path = Path::new(file_name);
    if path.exists() {
        return Err(io::Error::new(
            ErrorKind::AlreadyExists,
            "File already exists.",
        ));
    }

    let mut output = File::create(path)?;
    write!(output, "{}", content)
}

#[allow(dead_code)]
pub fn append(file_name: &str, content: &str) -> Result<(), io::Error> {
    let mut file = OpenOptions::new().append(true).open(file_name)?;
    writeln!(file, "{}", content)
}
