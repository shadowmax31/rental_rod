
use std::fs::{OpenOptions, File};
use std::io::{BufReader, prelude::*};
use std::path::Path;
use std::io;

use crate::db::db_error::DbError;

pub fn read(path: &str) -> Result<Vec<String>, DbError>  {
    let file = File::open(path)?;

    let buf = BufReader::new(file);
    Ok(buf.lines().map(|l| l.expect("Cannot parse line")).collect())
}

pub fn insert(path: &str, line: &str) -> Result<(), io::Error> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)?;

    writeln!(file, "{}", line)?;

    Ok(())
}

pub fn write(path: &str, version: &str, lines: &Vec<String>) -> Result<bool, io::Error> {
    let file_existed = remove_file(path)?;
    let file_created = !file_existed;

    insert(path, version)?;
    for line in lines {
        insert(path, line)?;
    }

    Ok(file_created)
}

pub fn remove_file(s_path: &str) -> Result<bool, io::Error> {
    let path = Path::new(s_path);
    let mut file_existed = false;
    if path.exists() {
        file_existed = true;
        std::fs::remove_file(s_path)?;
    }

    Ok(file_existed)
}

