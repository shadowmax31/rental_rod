use crate::db::{db_error::DbError, table::Table};
use v1::table_manager_v1::TableManagerV1;

mod v1;

/**
 * The TableManger is used to convert the Table Object to a File and a File to a Table Object
 * It should :
 * - drop the table
 * - create the table
 * - write the table
 * - read the table
 *
 * It does not need to manage individual lines.
 * It will basically overwrite a file with the content of the Table Object
 *
 * If needed, it should also manage locks on the table file
 */
pub fn get_table_manager(base_path: &str, tbl: &str) -> Result<impl TableManager, DbError> {
    if TableManagerV1::is_of_type(base_path, tbl) {
        Ok(TableManagerV1::new(base_path, tbl)?)
    } else {
        // Default version
        Ok(TableManagerV1::new(base_path, tbl)?)
    }
}

pub trait TableManager {
    fn drop(&self) -> Result<(), DbError>;

    #[allow(dead_code)]
    fn create(&self) -> Result<(), std::io::Error>;
    fn write(&mut self, tbl: &mut Table) -> Result<bool, DbError>;
    fn read(&self) -> Result<Table, DbError>;
    fn is_of_type(base_path: &str, tbl: &str) -> bool;
}
