//! Flat file database
//! 
//! The goal of this project is to have a light-weight human readable
//! database.
//! 
//! It's simple to get started with there are no real (external) dependencies 
//! (other than git, and it's not even mandatory). 
//! 
//! At the moment, it's more of a play-thing to learn how rust
//! works, but it has the basic implementation to Create, Read, 
//! Update and Delete values from the database.
//! 
//! It was heavily inspired by [`TinyDB`]
//! 
//! [`TinyDB`]: https://github.com/msiemens/tinydb
//! 
//! # Example
//! ```
//! let db = Db::new("path/to/database/folder");
//! 
//! let mut table = db.table("table_name").unwrap();
//! table.insert(Line::new());
//! db.write(&mut table).unwrap();
//! ```

pub mod db;

mod table_manager;
mod util;


#[cfg(test)]
mod test {
    #[test]
    fn test() {
        //
    }
}