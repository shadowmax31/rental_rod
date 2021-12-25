use table_manager_v1::TableManagerV1;

use crate::db::db_error::DbError;

mod table_manager_v1;
mod v1;

pub enum TableManagerVersion {
    V1(TableManagerV1),
}

pub fn get_table_manager(base_path: &str, tbl: &str) -> Result<TableManagerVersion, DbError> {
    Ok(TableManagerVersion::V1(TableManagerV1::new(base_path, tbl)?))
}
