pub mod db_error;
pub mod line;
pub mod field;
pub mod table;

use std::io;
use db_error::DbError;

use crate::table_manager;

use table::Table;

pub struct Db {
    path: String
}

/**
 * This is the PUBLIC interface to the Database.
 *
 * It allows to get tables by name, it creates the basic file and folder structure.
 * 
 * Any specific configuration of the Database should go here. For example, it is planned
 * to have the option to use GIT to keep a "backup" of ealier stages of the Database. 
 * This is where the calls should be made.
 * 
 * I think that managing the undos should also be done here. At this point, it's hard
 * to be sure since the DB does not have enough feature yet.
 */
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

    pub fn table(&self, tbl: &str) -> Result<Table, DbError> {
        let manager = table_manager::get_table_manager(&self.path, tbl)?;
        
        let table = match manager {
            table_manager::TableManagerVersion::V1(m) => m.read()
        };

        match table {
            Ok(t) => Ok(t),
            Err(error) => Err(error)
        }
    }

    pub fn write(&self, tbl: Table) {
        // let manager = table_manager::get_table_manager(&self.path, tbl);
        
        // let lines = match manager {
        //     table_manager::TableManagerVersion::V1(m) => m.read()
        // };
    }

}
