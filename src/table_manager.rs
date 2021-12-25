
use crate::db::db_error::DbError;
use v1::table_manager_v1::TableManagerV1;

mod v1;
pub enum TableManagerVersion {
    V1(TableManagerV1),
}

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
pub fn get_table_manager(base_path: &str, tbl: &str) -> Result<TableManagerVersion, DbError> {
    Ok(TableManagerVersion::V1(TableManagerV1::new(base_path, tbl)?))
}
