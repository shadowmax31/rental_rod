use std::fs::File;
use std::io::{self, BufRead};
use std::vec;

use crate::db::db_error::DbError;
use crate::db::table::Table;
use crate::util::file;
use crate::db::line::Line;

use super::reader;
use super::line_to_str::line_to_str;

const TBL_EXT: &str = ".dt";
const TBL_VERSION: &str = "#v1.0#";

pub struct TableManagerV1 {
    tbl_path: String,
    tbl_name: String
}

impl TableManagerV1 {
    pub fn new(base_path: &str, tbl: &str) -> Result<TableManagerV1, DbError> {
        let with_ext = tbl.to_owned() + TBL_EXT;

        let fullpath = std::path::Path::new(&base_path).join(with_ext); 
        let fullpath = fullpath.to_str().unwrap_or("");
        if fullpath == "" {
            panic!("The path to the table is empty");
        }

        let m = TableManagerV1 {
            tbl_path: String::from(fullpath),
            tbl_name: String::from(tbl)
        };
        m.create()?;

        Ok(m)
    }

    pub fn drop(&self) -> Result<(), io::Error> {
        file::remove_file(&self.tbl_path)?;

        Ok(())
    }

    pub fn create(&self) -> Result<(), std::io::Error> {
        let path = std::path::Path::new(&self.tbl_path);
        if !path.exists() {
            file::insert(&self.tbl_path, TBL_VERSION)?;
        }

        Ok(())
    }

    pub fn write(&self, tbl: &Table) -> Result<(), DbError> {
        let lines = TableManagerV1::convert_to_str(&tbl.lines);
        file::write(&self.tbl_path, TBL_VERSION, &lines)?;

        Ok(())
    }

    pub fn read(&self) -> Result<Table, DbError> {
        let raw = file::read(&self.tbl_path).unwrap_or(Vec::new());

        let mut lines = vec![];
        if raw.len() > 0 {
            lines = reader::read(&raw)?;
        }

        Table::new(&self.tbl_name, lines)
    }

    fn convert_to_str(lines: &Vec<Line>) -> Vec<String> {
        let mut str_lines: Vec<String> = Vec::new();
        for line in lines {
            str_lines.push(line_to_str(&line));
        }

        str_lines
    }
}

#[test]
fn test_delete() {
    let tbl = "test_delete";
    let m = TableManagerV1::new("/tmp/", tbl).unwrap();
    m.drop().unwrap();

    _insert(&m);
    _insert(&m);
    _insert(&m);
    assert_eq!(_count_lines(&m.tbl_path), 4);

    let m = TableManagerV1::new("/tmp/", tbl).unwrap();
    assert_eq!(_count_lines(&m.tbl_path), 4);
    m.drop().unwrap();
    assert_eq!(_count_lines(&m.tbl_path), 0);

    _insert(&m);
    _insert(&m);
    _insert(&m);
    assert_eq!(_count_lines(&m.tbl_path), 4);

    let m = TableManagerV1::new("/tmp/", tbl).unwrap();
    _insert(&m);
    _insert(&m);
    assert_eq!(_count_lines(&m.tbl_path), 6);

    m.drop().unwrap();
    assert_eq!(_count_lines(&m.tbl_path), 0);
}


#[test]
fn test_insert() {
    let m = TableManagerV1::new("/tmp/", "test_insert").unwrap();
    m.drop().unwrap();

    _insert(&m);
    _insert(&m);
    _insert(&m);
    _insert(&m);

    assert_eq!(_count_lines(&m.tbl_path), 5);

    m.drop().unwrap();
}

#[test]
fn test_read() {
   let m = TableManagerV1::new("/tmp", "test_read").unwrap();

   _insert(&m);
   _insert(&m);
   _insert(&m);

   let table = m.read().unwrap();
   assert_eq!(table.lines.len(), 3);

   m.drop().unwrap();
}

fn _insert(m: &TableManagerV1) {
    let mut table = m.read().unwrap();
    let mut line = Line::new();
    line.add("Col1", "123").unwrap();

    table.insert(line);
    
    assert_eq!(m.write(&table).unwrap(), ());
}

fn _count_lines(path: &str) -> usize {
    let file = File::open(path);

    let count = match file {
        Ok(file) => io::BufReader::new(file).lines().count(),
        Err(_) => 0
    };

    count
}
