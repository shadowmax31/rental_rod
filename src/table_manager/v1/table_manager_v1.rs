use std::fs::File;
use std::io::{self, BufRead};
use std::vec;

use crate::db::db_error::DbError;
use crate::db::field_type::Type;
use crate::db::table::Table;
use crate::table_manager::TableManager;
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

impl TableManager for TableManagerV1 {
    fn drop(&self) -> Result<(), DbError> {
        file::remove_file(&self.tbl_path)?;

        Ok(())
    }

    fn create(&self) -> Result<(), std::io::Error> {
        let path = std::path::Path::new(&self.tbl_path);
        if !path.exists() {
            file::insert(&self.tbl_path, TBL_VERSION)?;
        }

        Ok(())
    }

    fn write(&mut self, tbl: &mut Table) -> Result<bool, DbError> {
        let lines = TableManagerV1::convert_to_str(&tbl.get_lines());
        let file_created = file::write(&self.tbl_path, TBL_VERSION, &lines)?;

        Ok(file_created)
    }

    fn read(&self) -> Result<Table, DbError> {
        let raw = file::read(&self.tbl_path).unwrap_or(Vec::new());

        let mut lines = vec![];
        if raw.len() > 0 {
            lines = reader::read(&raw)?;
        }

        Table::new(&self.tbl_name, lines)
    }

    fn is_of_type(base_path: &str, tbl: &str) -> bool {

        let of_type = match TableManagerV1::get_fullpath(base_path, tbl) {
            Ok(p) => {
                let mut is_v1 = false;
                let contents =file::read(&p).unwrap_or(Vec::new());
                if contents.len() > 0 {
                    is_v1 = contents[0] == TBL_VERSION;
                }
                
                is_v1
            },
            Err(_) => false
        };


        of_type
    }
}

impl TableManagerV1 {
    pub fn new(base_path: &str, tbl: &str) -> Result<TableManagerV1, DbError> {
        let fullpath = TableManagerV1::get_fullpath(base_path, tbl)?;

        let m = TableManagerV1 {
            tbl_path: String::from(fullpath),
            tbl_name: String::from(tbl)
        };

        Ok(m)
    }

    fn get_fullpath(base_path: &str, tbl: &str) -> Result<String, DbError> {
        let with_ext = tbl.to_owned() + TBL_EXT;
        let fullpath = std::path::Path::new(base_path).join(with_ext); 
        let fullpath = match fullpath.to_str() {
            Some(p) => p,
            None => return Err(DbError::Custom(String::from("The path to the table is empty")))
        };

        Ok(String::from(fullpath))
    }


    fn convert_to_str(lines: &Vec<&mut Line>) -> Vec<String> {
        let mut str_lines: Vec<String> = Vec::new();
        for line in lines {
            str_lines.push(line_to_str(&line));
        }

        str_lines
    }
}

#[test]
fn test_is_of_type() {
    let tbl = "test_of_type_v1";
    let base_path = "/tmp";
    let fullpath = TableManagerV1::get_fullpath(base_path, tbl).unwrap();

    // File does not exists
    file::remove_file(&fullpath).unwrap();
    assert_eq!(TableManagerV1::is_of_type(base_path, tbl), false);


    // Check empty file
    file::write(&fullpath, "", &vec![]).unwrap();
    assert_eq!(TableManagerV1::is_of_type(base_path, tbl), false);

    // Check file with the correct version
    file::write(&fullpath, TBL_VERSION, &vec![]).unwrap();
    assert_eq!(TableManagerV1::is_of_type(base_path, tbl), true);

    // Check file with the correct version (and data)
    file::write(&fullpath, TBL_VERSION, &vec!["line1".to_owned(), "line2".to_owned()]).unwrap();
    assert_eq!(TableManagerV1::is_of_type(base_path, tbl), true);

    // Check file with wrong version
    file::write(&fullpath, "V2.1", &vec![]).unwrap();
    assert_eq!(TableManagerV1::is_of_type(base_path, tbl), false);
}

#[test]
fn test_drop() {
    let tbl = "test_delete";
    let mut m = TableManagerV1::new("/tmp/", tbl).unwrap();
    m.drop().unwrap();

    _insert(&mut m);
    _insert(&mut m);
    _insert(&mut m);
    assert_eq!(_count_lines(&m.tbl_path), 4);

    let mut m = TableManagerV1::new("/tmp/", tbl).unwrap();
    assert_eq!(_count_lines(&m.tbl_path), 4);
    m.drop().unwrap();
    assert_eq!(_count_lines(&m.tbl_path), 0);

    _insert(&mut m);
    _insert(&mut m);
    _insert(&mut m);
    assert_eq!(_count_lines(&m.tbl_path), 4);

    let mut m = TableManagerV1::new("/tmp/", tbl).unwrap();
    _insert(&mut m);
    _insert(&mut m);
    assert_eq!(_count_lines(&m.tbl_path), 6);

    m.drop().unwrap();
    assert_eq!(_count_lines(&m.tbl_path), 0);
}


#[test]
fn test_insert() {
    let mut m = TableManagerV1::new("/tmp/", "test_insert").unwrap();
    m.drop().unwrap();

    _insert(&mut m);
    _insert(&mut m);
    _insert(&mut m);
    _insert(&mut m);

    assert_eq!(_count_lines(&m.tbl_path), 5);

    m.drop().unwrap();
}

#[test]
fn test_read() {
   let mut m = TableManagerV1::new("/tmp", "test_read").unwrap();

   _insert(&mut m);
   _insert(&mut m);
   _insert(&mut m);

   let mut table = m.read().unwrap();
   assert_eq!(table.get_lines().len(), 3);

   m.drop().unwrap();
}

fn _insert(m: &mut TableManagerV1) {
    let mut table = m.read().unwrap();
    let mut line = Line::new();
    line.add("Col1", Type::from_str("123")).unwrap();

    table.insert(line);
    
    assert_eq!(m.write(&mut table).is_ok(), true);
}

fn _count_lines(path: &str) -> usize {
    let file = File::open(path);

    let count = match file {
        Ok(file) => io::BufReader::new(file).lines().count(),
        Err(_) => 0
    };

    count
}
