use std::fs::File;
use std::io::{self, BufRead};

use crate::db::db_error::DbError;
use crate::db::table::Table;
use crate::util::file;
use crate::db::line::Line;
use super::v1::line_to_str::line_to_str;
use super::v1::reader;

const TBL_EXT: &str = ".dt";
const TBL_VERSION: &str = "#v1.0#";

pub struct TableManagerV1 {
    tbl_path: String
}

impl TableManagerV1 {
    pub fn new(base_path: &str, tbl: &str) -> TableManagerV1 {
        let with_ext = tbl.to_owned() + TBL_EXT;

        let fullpath = std::path::Path::new(&base_path).join(with_ext); 
        let fullpath = fullpath.to_str().unwrap_or("");
        if fullpath == "" {
            panic!("The path to the table is empty");
        }

        let m = TableManagerV1 {
            tbl_path: String::from(fullpath)
        };

        m
    }

    pub fn delete_all(&self) -> Result<(), io::Error> {
        file::remove_file(&self.tbl_path)?;

        Ok(())
    }

    pub fn insert(&self, line: &Line) -> Result<(), DbError> {
        self.create_table()?;

        match self.read() {
           Ok(v) => {
            let mut lines = TableManagerV1::convert_to_str(&v);
            lines.push(line_to_str(line));

            file::write(&self.tbl_path, TBL_VERSION, &lines)?;
           },
           Err(msg) => {
               return Err(DbError::Custom(msg));
           }
        }        

        Ok(())
    }

    pub fn read(&self) -> Result<Vec<Line>, String> {
        let raw = file::read(&self.tbl_path).unwrap_or(Vec::new());
        let r = reader::read(&raw)?;

        Ok(r)
    }

    fn convert_to_str(lines: &Vec<Line>) -> Vec<String> {
        let mut str_lines: Vec<String> = Vec::new();
        for line in lines {
            str_lines.push(line_to_str(&line));
        }

        str_lines
    }

    fn create_table(&self) -> Result<(), std::io::Error> {
        let path = std::path::Path::new(&self.tbl_path);
        if !path.exists() {
            file::insert(&self.tbl_path, TBL_VERSION)?;
        }

        Ok(())
    }
}

#[test]
fn test_delete() {
    let tbl = "test_delete";
    let m = TableManagerV1::new("/tmp/", tbl);
    _insert(&m);
    _insert(&m);
    _insert(&m);
    assert_eq!(_count_lines(&m.tbl_path), 4);

    let m = TableManagerV1::new("/tmp/", tbl);
    assert_eq!(_count_lines(&m.tbl_path), 4);
    m.delete_all().unwrap();
    assert_eq!(_count_lines(&m.tbl_path), 0);

    _insert(&m);
    _insert(&m);
    _insert(&m);
    assert_eq!(_count_lines(&m.tbl_path), 4);

    let m = TableManagerV1::new("/tmp/", tbl);
    _insert(&m);
    _insert(&m);
    assert_eq!(_count_lines(&m.tbl_path), 6);

    m.delete_all().unwrap();
    assert_eq!(_count_lines(&m.tbl_path), 0);
}


#[test]
fn test_insert() {
    let m = TableManagerV1::new("/tmp/", "test_insert");
    m.delete_all().unwrap();

    _insert(&m);
    _insert(&m);
    _insert(&m);
    _insert(&m);

    assert_eq!(_count_lines(&m.tbl_path), 5);
}

#[test]
fn test_read() {
   let m = TableManagerV1::new("/tmp", "test_read");

   _insert(&m);
   _insert(&m);
   _insert(&m);

   let lines = m.read().unwrap();
   assert_eq!(lines.len(), 3);

   m.delete_all().unwrap();
}

fn _insert(m: &TableManagerV1) {
    let fields = vec![ crate::db::line::Field::new("Col1", "123") ];
    let line = Line::new(fields);

    assert_eq!(m.insert(&line).unwrap(), ());
}

fn _count_lines(path: &str) -> usize {
    let file = File::open(path);

    let count = match file {
        Ok(file) => io::BufReader::new(file).lines().count(),
        Err(_) => 0
    };

    count
}
