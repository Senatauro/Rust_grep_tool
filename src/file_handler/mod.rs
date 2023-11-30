use std::{fs, io::Error, path::Path};

pub fn read_file_content(file_path: &str) -> Result<String, Error> {
    let _p = Path::new(file_path);
    _p.try_exists()?;
    fs::read_to_string(_p)
}