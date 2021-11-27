mod db;
mod table_manager;
mod util;

use db::Db;
use db::line::Line;
use db::line::Field;

use db::db_error::DbError;
use std::process;

//static PATH: &str = "/root/db";
static PATH: &str = "/tmp/db";

fn main() {
    let db = Db::new(PATH).unwrap_or_else(|error| {
        print_error("Error when initializing the database", error);
        
        process::exit(1)
    });
    
    let line = new_test_line().unwrap();
    db.insert("btc", &line).unwrap_or_else(|error| {
        print_error("Error during the insert", error);
    });

    for f in line.fields {
        println!("{}", f.value);
    }

}

fn print_error(details: &str, error: DbError) {
    eprintln!("{}: {}", details, error);
}

fn new_test_line() -> Option<Line> {
    let mut fields: Vec<Field> = Vec::new();
    let mut field = Field {
        name: String::from("col1"),
        value: String::from("123")
    };
    fields.push(field);

    field = Field {
        name: String::from("col2"),
        value: String::from("456")
    };
    fields.push(field);

    field = Field {
        name: String::from("col3"),
        value: String::from("789")
    };
    fields.push(field);

    Line::new(fields)
}
