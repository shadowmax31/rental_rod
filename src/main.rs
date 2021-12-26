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
    let db = Db::new(PATH).unwrap_or_else(|error| {
        print_error("Error when initializing the database", error);
        
        process::exit(1)
    });
    
    match db.table("test") {
        Err(err) => print_error("Error when reading the table", err),
        Ok(table)  => {
            for line in table.lines {
                println!("{:?}", line);
            }
        }
    };

    // let line = new_test_line().unwrap();
    // db.insert("btc", &line).unwrap_or_else(|error| {
    //     print_error("Error during the insert", error);
    // });
}

fn print_error(details: &str, error: DbError) {
    eprintln!("{}: {}", details, error);
}

fn new_test_line() -> Line {
    let mut line = Line::new();

    line.add("col1", "123");
    line.add("col2", "456");
    line.add("col3", "789");

    line
}
