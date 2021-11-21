use table_manager_v1::TableManagerV1;

mod table_manager_v1;

pub enum TableManagerVersion {
    V1(TableManagerV1),
}

pub fn get_table_manager(base_path: &str, tbl: &str) -> TableManagerVersion {
    TableManagerVersion::V1(TableManagerV1::new(base_path, tbl))
}
