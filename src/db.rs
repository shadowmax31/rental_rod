pub mod db_error;
pub mod line;
pub mod field;
pub mod field_type;
pub mod table;

use std::io;
use db_error::DbError;
use uuid::Uuid;

use table::Table;

use crate::table_manager;

use self::{line::Line, field_type::Type, field::Field};

pub struct Db {
    path: String,
    use_git: bool
}

enum Config {
    UseGit
}

impl Config {
    pub fn value(&self) -> &str {
        match self {
            Config::UseGit => "use_git",
        }
    }
}

/**
 * This is the PUBLIC interface to the Database.
 *
 * It allows to get tables by name, it creates the basic file and folder structure.
 * 
 * Any specific configuration of the Database should go here. For example, it is planned
 * to have the option to use GIT to keep a "backup" of ealier stages of the Database. 
 * This is where the calls should be made.
 * 
 * I think that managing the undos should also be done here. At this point, it's hard
 * to be sure since the DB does not have enough feature yet.
 */
impl Db {
    pub fn new(path: &str) -> Result<Db, DbError> {
        if path == "" {
            return Err(DbError::Custom(String::from("Missing database path")));
        }

        let mut db = Db { 
            path: String::from(path),
            use_git: false
         };
        db.init()?;

        return Ok(db);
    }

    fn init(&mut self) -> Result<(), DbError> {
        std::fs::create_dir_all(&self.path)?;
        
        self.init_default_config()?;

        Ok(())
    }

    fn init_default_config(&mut self) -> Result<(), DbError> {
        self.create_config_line()?;

        // Default config fields
        self.create_config_field(Config::UseGit.value(), Type::from_bool(false))?;


        if let Type::Boolean(b) = self.get_config(Config::UseGit.value())? {
            self.use_git = b;
        }

        Ok(())
    }

    fn get_config_table(&self) -> Result<Table, DbError> {
        Ok(self.table(".config")?)
    }

    fn get_config(&self, config_name: &str) -> Result<Type, DbError> {
        let mut table = self.get_config_table()?;
        let line = match table.find_by_id(&self.get_config_id()?) {
            Some(l) => l,
            None => return Err(DbError::Custom(String::from("The config line does not exist")))
        };

        let field = match line.get(config_name) {
            Some(f) => f,
            None => return Err(DbError::Custom(String::from("The config for [") + config_name + "] does not exist"))
        };

        Ok(field.get().clone())
    }

    fn create_config_line(&self) -> Result<(), DbError> {
        let mut table = self.get_config_table()?;
        let id = self.get_config_id()?;

        match table.find_by_id(&id) {
            None => {
                let line = Line::new_with_id(id, vec![]);
                table.insert(line);

                self.write(&mut table)?;
            },
            _ => ()
        };

        Ok(())
    }
    
    fn create_config_field(&self, config_name: &str, default_value: Type) -> Result<(), DbError> {
        let mut table = self.get_config_table()?;
        let line = match table.find_by_id(&self.get_config_id()?) {
            Some(line) => line,
            None => return Err(DbError::Custom(String::from("Missing configuration line from the config table")))
        };

        match line.get(config_name) {
            None => {
                line.add(config_name, default_value)?;

                self.write(&mut table)?;
            }
            _ => ()
        }

        Ok(())
    }

    fn get_config_id(&self) -> Result<Uuid, DbError> {
        match Uuid::parse_str("11111111-1111-1111-1111-111111111111") {
            Ok(id) => Ok(id),
            Err(error) => Err(DbError::Custom(error.to_string()))
        }
    }

    pub fn set_use_git(&self, use_git: bool) -> Result<(), DbError> {
        let id = self.get_config_id()?;

        // Get the config line from the config table
        let mut table = self.get_config_table()?;
        let line = match table.find_by_id(&id) {
            Some(l) => l,
            None => return Err(DbError::Custom(String::from("The config line was not found...")))
        };

        // Get the correct config field from the config line
        let field = match line.get(Config::UseGit.value()) {
            Some(l) => l,
            None => return Err(DbError::Custom(String::from("The field [") + Config::UseGit.value() + "] was not found..."))
        };

        // Set the value
        let value = Type::from_bool(use_git);
        field.set(value);


        // Write to the file
        self.write(&mut table)?;
        Ok(())
    }

    pub fn table(&self, tbl: &str) -> Result<Table, DbError> {
        let manager = table_manager::get_table_manager(&self.path, tbl)?;
        
        let table = match manager {
            table_manager::TableManagerVersion::V1(m) => m.read()
        };

        match table {
            Ok(t) => Ok(t),
            Err(error) => Err(error)
        }
    }

    pub fn tables(&self) -> Result<Vec<String>, DbError> {
        let paths = std::fs::read_dir(&self.path)?;

        let mut tables: Vec<String> = Vec::new();
        for path in paths {
            if let Some(name) = path?.file_name().to_str() {
                if !name.starts_with(".") {
                    tables.push(String::from(name));
                }
            }
        }

        Ok(tables)
    }

    pub fn drop(&self, tbl: &str) -> Result<(), DbError> {
        let manager = table_manager::get_table_manager(&self.path, tbl)?;
        
        match manager {
            table_manager::TableManagerVersion::V1(mut m) => m.drop()?
        };

        Ok(())
    }

    pub fn write(&self, table: &mut Table) -> Result<(), DbError> {
        let manager = table_manager::get_table_manager(&self.path, table.get_name())?;
        
        match manager {
            table_manager::TableManagerVersion::V1(mut m) => m.write(table)?
        };

        Ok(())
    }

}

#[test]
fn test_config() {
    let p = "/tmp/test_config";
    let db = _init_db(p, true);

    let value = db.get_config(Config::UseGit.value()).unwrap();
    match value {
        Type::Boolean(v) => assert_eq!(v, false),
        _ => assert!(false)
    }

    db.set_use_git(true).unwrap();

    let value = db.get_config(Config::UseGit.value()).unwrap();
    match value {
        Type::Boolean(v) => assert_eq!(v, true),
        _ => assert!(false)
    }

    // Reload the DB
    let db = _init_db(p, false);
    let value = db.get_config(Config::UseGit.value()).unwrap();
    match value {
        Type::Boolean(v) => assert_eq!(v, true),
        _ => assert!(false)
    }
}

#[test]
fn test_table_creation() {
    let p = "/tmp/test_insert/";
    let db = _init_db(p, true);

    assert_eq!(db.tables().unwrap().len(), 0);

    db.table("test1").unwrap();
    db.table("test2").unwrap();

    assert_eq!(db.tables().unwrap().len(), 2);

    let db = _init_db(p, false);
    assert_eq!(db.tables().unwrap().len(), 2);

    db.table("test1").unwrap();
    assert_eq!(db.tables().unwrap().len(), 2);
}

#[test]
fn test_drop_table() {
    let p = "/tmp/test_drop";
    let db = _init_db(p, true);

    assert_eq!(db.tables().unwrap().len(), 0);

    db.table("test1").unwrap();
    db.table("test2").unwrap();

    assert_eq!(db.tables().unwrap().len(), 2);

    db.drop("test1").unwrap();
    assert_eq!(db.tables().unwrap().len(), 1);

    // Drop existing table
    db.drop("test1").unwrap();
    assert_eq!(db.tables().unwrap().len(), 1);

    let db = _init_db(p, false);
    assert_eq!(db.tables().unwrap().len(), 1);

    db.drop("test2").unwrap();
    assert_eq!(db.tables().unwrap().len(), 0);
}

#[test]
fn test_write() {
    let p = "/tmp/test_drop";
    let db = _init_db(p, true);

    // Check that adding a line to the table in memory works
    let mut table = db.table("write").unwrap();
    table.insert(_new_test_line());
    assert_eq!(table.get_lines().len(), 1);

    // Fetch the table from the file. The line should not have been written
    let mut table = db.table("write").unwrap();
    assert_eq!(table.get_lines().len(), 0);

    // Writing an empty table, should produce an empty table
    db.write(&mut table).unwrap();
    assert_eq!(db.table("write").unwrap().get_lines().len(), 0);

    // Reload the db
    let db = _init_db(p, false);

    // The table should still be empty
    assert_eq!(db.table("write").unwrap().get_lines().len(), 0);

    // Insert a line and write the line
    let mut table = db.table("write").unwrap();
    table.insert(_new_test_line());
    db.write(&mut table).unwrap();

    // Fetch the table from the file. the line should be there
    assert_eq!(db.table("write").unwrap().get_lines().len(), 1);

    // Reload the Db and refetch the table. The line should be there
    let db = _init_db(p, false);
    assert_eq!(db.table("write").unwrap().get_lines().len(), 1);
}



fn _init_db(p: &str, fresh: bool) -> Db {
    if std::path::Path::new(p).exists() && fresh {
        std::fs::remove_dir_all(p).unwrap();
    }

    Db::new(p).unwrap()
}

fn _new_test_line() -> Line {
    let mut line = Line::new();

    line.add("col1", Type::from_str("123")).unwrap();
    line.add("col2", Type::from_str("456")).unwrap();
    line.add("col3", Type::from_str("789")).unwrap();

    line
}