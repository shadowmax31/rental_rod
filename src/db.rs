//! This module contains the Database
//! 
//! This is the place where you can query the data.
pub mod db_error;
pub mod line;
pub mod field;

pub mod field_type;

pub mod table;

use std::process::Command;
use db_error::DbError;
use uuid::Uuid;

use table::Table;

use crate::table_manager::{self, TableManager};

use self::{line::Line, field_type::Type};

/**
* This is the PUBLIC interface to the Database.
*
* It allows to get tables by name, it creates the basic file and folder structure.
*/
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

impl Db {
    /**
    * Initialize a database
    */ 
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
    
    fn git_init(&self) -> Result<(), DbError> {
        if self.use_git {
            if !self.git_exists() {
                match Command::new("git")
                .arg("init")
                .arg(&self.path)
                .output() {
                    Err(error) => return Err(DbError::Custom(error.to_string())),
                    Ok(_) => ()
                };
            }
        }
        
        Ok(())
    }
    
    fn git_exists(&self) -> bool {
        std::path::Path::new(&self.path).join(".git").exists()
    }
    
    fn git_commit(&self, msg: &str) -> Result<(), DbError> {
        if self.use_git {
            self.git_init()?;
            match Command::new("git")
            .arg("-C")
            .arg(&self.path)
            .arg("add")
            .arg(".")
            .output() {
                Err(error) => return Err(DbError::Custom(error.to_string())),
                Ok(_) => ()
            };
            
            match Command::new("git")
            .arg("-C")
            .arg(&self.path)
            .arg("commit")
            .arg("-m")
            .arg(msg)
            .output() {
                Err(error) => return Err(DbError::Custom(error.to_string())),
                Ok(_) => ()
            };
            
        }
        
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
    
    /**
    * Allows to know if the database is using git to keep versions of itself
    */
    pub fn get_use_git(&self) -> bool {
        self.use_git
    }
    
    /**
    * Sets the use_git parameter
    */
    pub fn set_use_git(&mut self, use_git: bool, msg: Option<&str>) -> Result<(), DbError> {
        let id = self.get_config_id()?;
        
        // Get the config line from the config table
        let mut table = self.get_config_table()?;
        let line = match table.find_by_id(&id) {
            Some(l) => l,
            None => return Err(DbError::Custom(String::from("The config line was not found...")))
        };
        
        // Get the correct config field from the config line
        let field = match line.get_mut(Config::UseGit.value()) {
            Some(l) => l,
            None => return Err(DbError::Custom(String::from("The field [") + Config::UseGit.value() + "] was not found..."))
        };
        
        // Set the value
        let value = Type::from_bool(use_git);
        field.set(value);
        
        self.use_git = use_git;
        if self.use_git {
            let msg = match msg {
                Some(msg) => msg,
                None => "Commit all changes since last git activation"
            };
            
            self.git_commit(msg)?;
        }
        
        // Write to the file
        self.write(&mut table)?;
        
        Ok(())
    }
    
    /**
    * This is where we can query the database. It reads the information
    * on disk and returns an object in memory with the corresponding data
    */
    pub fn table(&self, tbl: &str) -> Result<Table, DbError> {
        let manager = table_manager::get_table_manager(&self.path, tbl)?;
        
        match manager.read() {
            Ok(t) => Ok(t),
            Err(error) => Err(error)
        }
    }
    
    /**
    * Lists the tables in the database
    * It will ignore system table. All table starting with a "." are considered as system table.
    */
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
    
    /**
    * Drops the table from the database
    */
    pub fn drop(&self, tbl: &str) -> Result<(), DbError> {
        let manager = table_manager::get_table_manager(&self.path, tbl)?;
        manager.drop()?;
        
        let msg = String::from("Drop table ") + "[" + tbl + "]";
        self.git_commit(&msg)?;
        
        Ok(())
    }
    
    /**
    * This writes (or commits) all the changes from a Table to the database
    */
    pub fn write(&self, table: &mut Table) -> Result<(), DbError> {
        let mut manager = table_manager::get_table_manager(&self.path, table.get_name())?;
        
        let msg: String;
        if manager.write(table)? {
            msg = String::from("Create table ") + "[" + table.get_name() + "]";
        }
        else {
            msg = String::from("Update table ") + "[" + table.get_name() + "]";
        }
        
        self.git_commit(&msg)?;
        Ok(())
    }
    
}

#[test]
fn test_dont_use_git() {
    let p = "/tmp/test_dont_use_git";
    let db = _init_db(p, true);
    assert_eq!(db.git_exists(), false);
    assert_eq!(db.use_git, false);
    
    let mut table = db.table("test").unwrap();
    table.insert(Line::new());
    db.write(&mut table).unwrap();
    
    let log = _git_log(&db);
    assert_eq!(db.git_exists(), false);
    assert_eq!(log.len(), 0);
    
    let db = _init_db(p, false);
    assert_eq!(db.use_git, false);
    let mut table = db.table("test").unwrap();
    
    let log = _git_log(&db);
    assert_eq!(db.git_exists(), false);
    assert_eq!(log.len(), 0);
    
    table.insert(Line::new());
    
    let log = _git_log(&db);
    assert_eq!(db.git_exists(), false);
    assert_eq!(log.len(), 0);
    db.write(&mut table).unwrap();
    
    let mut table = db.table("tbl").unwrap();
    table.insert(Line::new());
    db.write(&mut table).unwrap();
    
    let log = _git_log(&db);
    assert_eq!(db.git_exists(), false);
    assert_eq!(log.len(), 0);
}

#[test]
fn test_git_init_when_db_is_not_empty() {
    let p = "/tmp/test_git_db_not_empty";
    let db = _init_db(p, true);
    assert_eq!(db.git_exists(), false);
    
    let mut table = db.table("without").unwrap();
    table.insert(Line::new());
    db.write(&mut table).unwrap();
    
    assert_eq!(db.git_exists(), false);
    assert_eq!(db.use_git, false);
    assert_eq!(_git_log(&db).len(), 0);
    
    let mut db = _init_db(p, false);
    db.set_use_git(true, None).unwrap();
    assert_eq!(db.git_exists(), true);
    let log = _git_log(&db);
    assert_eq!(log.len(), 2);
    assert_eq!(log[0], "Update table [.config]");
    assert_eq!(log[1], "Commit all changes since last git activation");
}

#[test]
fn test_use_git() {
    let p = "/tmp/test_use_git";
    let mut db = _init_db(p, true);
    assert_eq!(db.git_exists(), false);
    assert_eq!(db.use_git, false);
    
    db.set_use_git(true, Some("Test batch commit message")).unwrap();
    assert_eq!(db.git_exists(), true);
    assert_eq!(db.use_git, true);
    
    let mut table = db.table("test").unwrap();
    table.insert(Line::new());
    db.write(&mut table).unwrap();
    
    let log = _git_log(&db);
    assert_eq!(log.len(), 3);
    
    assert_eq!(log[0], "Create table [test]");
    assert_eq!(log[1], "Update table [.config]");
    assert_eq!(log[2], "Test batch commit message");
    
    let db = _init_db(p, false);
    assert_eq!(db.use_git, true);
    
    let mut table = db.table("test").unwrap();
    
    let log = _git_log(&db);
    assert_eq!(log.len(), 3);
    
    table.insert(Line::new());
    
    let log = _git_log(&db);
    assert_eq!(log.len(), 3);
    db.write(&mut table).unwrap();
    
    let mut table = db.table("tbl").unwrap();
    table.insert(Line::new());
    db.write(&mut table).unwrap();
    
    let log = _git_log(&db);
    assert_eq!(log.len(), 5);
    assert_eq!(log[0], "Create table [tbl]");
    assert_eq!(log[1], "Update table [test]");
    assert_eq!(log[2], "Create table [test]");
    assert_eq!(log[3], "Update table [.config]");
    assert_eq!(log[4], "Test batch commit message");
    
}

#[test]
fn test_git_config() {
    let p = "/tmp/test_git_config";
    let mut db = _init_db(p, true);
    
    let value = db.get_config(Config::UseGit.value()).unwrap();
    match value {
        Type::Boolean(v) => assert_eq!(v, false),
        _ => assert!(false)
    }
    assert_eq!(db.use_git, false);
    
    db.set_use_git(true, None).unwrap();
    
    let value = db.get_config(Config::UseGit.value()).unwrap();
    match value {
        Type::Boolean(v) => assert_eq!(v, true),
        _ => assert!(false)
    }
    assert_eq!(db.use_git, true);
    
    // Reload the DB
    let db = _init_db(p, false);
    let value = db.get_config(Config::UseGit.value()).unwrap();
    match value {
        Type::Boolean(v) => assert_eq!(v, true),
        _ => assert!(false)
    }
    assert_eq!(db.use_git, true);
}

#[test]
fn test_table_creation() {
    let p = "/tmp/test_insert/";
    let db = _init_db(p, true);
    
    assert_eq!(db.tables().unwrap().len(), 0);
    
    db.table("test1").unwrap();
    db.table("test2").unwrap();
    
    assert_eq!(db.tables().unwrap().len(), 0);
    
    let mut table = db.table("test1").unwrap();
    db.write(&mut table).unwrap();
    let mut table = db.table("test2").unwrap();
    db.write(&mut table).unwrap();
    
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
    
    let mut table = db.table("test1").unwrap();
    db.write(&mut table).unwrap();
    let mut table = db.table("test2").unwrap();
    db.write(&mut table).unwrap();
    
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
    let p = "/tmp/test_write";
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

fn _git_log(db: &Db) -> Vec<String> {
    let mut formatted_lines = vec![];
    
    if db.git_exists() {
        let output = std::process::Command::new("git")
        .arg("-C")
        .arg(&db.path)
        .arg("log")
        .arg("--oneline")
        .output().unwrap();
        
        
        let lines = String::from_utf8(output.stdout).unwrap();
        
        lines.lines().for_each(|line| {
            let index = _get_first_space(line);
            let s = String::from(line.split_at(index + 1).1);
            formatted_lines.push(s);
        });
    }
    
    formatted_lines
}

fn _get_first_space(str: &str) -> usize {
    let mut index = 0;
    for c in str.chars() {
        if c == ' ' {
            break;
        }
        
        index += 1;
    }
    
    index
}