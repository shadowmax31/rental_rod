mod db;
mod table_manager;
mod util;

use db::Db;
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
        Ok(table)  => {
            for line in table.get_lines() {
                println!("{:?}", line);
            }
        }
    };

    // let line = new_test_line();
    // match line {
    //     Ok(line) => {
    //         match db.table("btc") {
    //             Ok(mut table) => table.insert(line),
    //             Err(error) => print_error("Error during the insert", error)
    //         };
    //     },
    //     Err(error) => print_error("Error during the line creation", error)
    // };
}

fn print_error(details: &str, error: DbError) {
    eprintln!("{}: {}", details, error);
}

fn new_test_line() -> Result<Line, DbError> {
    let mut line = Line::new();

    line.add("col1", "123")?;
    line.add("col2", "456")?;
    line.add("col3", "789")?;

    Ok(line)
}
