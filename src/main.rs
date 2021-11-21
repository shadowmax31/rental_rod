mod db;
mod table_manager;
mod util;

use db::Db;
use db::db_error::DbError;
use std::process;

//static PATH: &str = "/root/db";
static PATH: &str = "/tmp/db";

fn main() {
    let db = Db::new(PATH).unwrap_or_else(|error| {
        print_error("Error when initializing the database", error);
        
        process::exit(1)
    });
    
    db.insert("btc").unwrap_or_else(|error| {
        print_error("Error during the insert", error);
    });

}

fn print_error(details: &str, error: DbError) {
    eprintln!("{}: {}", details, error);
}
