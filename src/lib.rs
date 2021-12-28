pub mod db;
mod table_manager;
mod util;


#[cfg(test)]
mod test {
    use crate::db::{Db, line::Line, field_type::Type};

    #[test]
    fn test() {
        // let mut db = Db::new("/tmp/git_test").unwrap();
        // db.set_use_git(true).unwrap();

        // let mut table = db.table("new table").unwrap();
        // let mut line = Line::new();
        // line.add("field", Type::from_str("This is a field")).unwrap();
        // line.add("age", Type::from_int(20)).unwrap();

        // table.insert(line);

        // db.write(&mut table).unwrap();

        // table.insert(Line::new());
        // db.write(&mut table);

        // db.drop("new table");
    }
}