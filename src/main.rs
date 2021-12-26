mod db;
mod table_manager;
mod util;

use db::Db;
use db::field_type::Type;
use db::line::Line;
use db::field::Field;

use db::db_error::DbError;
use uuid::Uuid;
use std::process;

static PATH: &str = ".";

fn main() {
    let db = match Db::new(PATH) {
        Ok(db) => db,
        Err(error) => {
            print_error("Error when initializing the database", error);
            process::exit(1)
        }
    };
    
    match db.table("test") {
        Err(err) => print_error("Error when reading the table", err),
        Ok(mut table)  => {
            // if let Ok(line) = new_test_line() {
                // table.insert(line);
            // }
            table.print();

            if let Err(error) = db.write(&mut table) {
                print_error("Error when writing the table", error);
            }
        }
    };
}

fn print_error(details: &str, error: DbError) {
    eprintln!("{}: {}", details, error);
}

fn new_test_line() -> Result<Line, DbError> {
    let mut line = Line::new();

    line.add("col1", Type::from_str("123"))?;
    line.add("col2", Type::from_str("456"))?;
    line.add("col3", Type::from_str("789"))?;

    Ok(line)
}
