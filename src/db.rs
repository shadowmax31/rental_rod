pub mod db_error;
pub mod line;
pub mod table;

use std::io;
use db_error::DbError;

use crate::table_manager;

use table::Table;

pub struct Db {
    path: String
}

impl Db {
    pub fn new(path: &str) -> Result<Db, DbError> {
        if path == "" {
            return Err(DbError::Custom(String::from("Missing database path")));
        }

        let db = Db { path: String::from(path) };
        db.init()?;

        return Ok(db);
    }

    fn init(&self) -> Result<(), io::Error> {
        std::fs::create_dir_all(&self.path)?;

        Ok(())
    }

    pub fn insert(&self, tbl: &str, line: &line::Line) -> Result<(), DbError> {
        if tbl == "" {
            return Err(DbError::Custom(String::from("Missing table name")));
        }

        let manager = table_manager::get_table_manager(&self.path, tbl);
        
        match manager {
            table_manager::TableManagerVersion::V1(m) => m.insert(line)?
        };

        Ok(())
    }

    pub fn table(&self, tbl: &str) -> Result<Table, DbError> {
        let manager = table_manager::get_table_manager(&self.path, tbl);
        
        let lines = match manager {
            table_manager::TableManagerVersion::V1(m) => m.read()
        };

        match lines {
            Ok(lines) => Ok(Table::new(tbl, lines)?),
            Err(msg) => Err(DbError::Custom(msg))
        }
    }

}
