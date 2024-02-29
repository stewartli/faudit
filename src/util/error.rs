use std::{fs, io};

#[allow(unused)]
pub fn cp_temp(src: &str, des: &str) -> Result<(), io::Error> {
    fs::copy(src, des)?;
    println!("✓ file copied");
    Ok(())
}

#[allow(unused)]
enum FAError {
    IOError,
    YAMLError,
    JSONError,
}

// TODO:
// impl From<io::Error> for FAError {}
