use uuid::Uuid;

use super::field::Field;
use super::db_error::DbError;
use super::field_type::Type;

#[derive(Debug)]
pub struct Line {
    id: Uuid,
    fields: Vec<Field>
}

impl Line {
    pub fn new() -> Line {
        Line::new_with_id(Uuid::new_v4(), vec![])
    }

    pub fn new_with_id(id: Uuid, fields: Vec<Field>) -> Line {
        Line { id, fields }
    }

    pub fn get_id(&self) -> &Uuid {
        &self.id
    }

    pub fn has_with(&mut self, field_name: &str, with: &str) -> bool {
        if let Some(f) = self.get(field_name) {
            return f.get() == &Type::from_str(with);
        }

        false
    }

    pub fn get_fields(&self) -> &Vec<Field> {
        &self.fields
    }

    pub fn add(&mut self, field_name: &str, value: Type) -> Result<(), DbError> {
        let fields = self.get_fields_name();
        if fields.contains(&field_name) {
            return Err(DbError::Custom(String::from("The field [") + field_name + "] already exists"));
        }

        let f = Field::new(field_name, value);
        self.fields.push(f);

        Ok(())
    }

    pub fn remove(&mut self, field_name: &str) {
        if let Some(i) = self.get_index(field_name) {
            self.fields.remove(i);
        }
    }

    pub fn get_index(&self, field_name: &str) -> Option<usize> {
        let mut i: usize = 0;
        for field in &self.fields {
            if field.get_name() == field_name {
                return Some(i);
            }

            i += 1;
        }

        None
    }

    pub fn get(&mut self, field_name: &str) -> Option<&mut Field> {
        if let Some(i) = self.get_index(field_name) {
            return Some(&mut self.fields[i]);
        }

        None
    }

    pub fn get_fields_name(&self) -> Vec<&str> {
        let mut fields: Vec<&str> = Vec::new();
        for f in &self.fields {
            fields.push(&f.get_name());
        }

        fields
    }
}


#[test]
fn test_get() {
    let mut line = _init_line();

    assert_eq!(line.get("firstname").unwrap().get().to_string(), "Mike");
    assert_eq!(line.get("favorite_number").unwrap().get().to_string(), "1245");
    assert_eq!(line.get("lastname").unwrap().get().to_string(), "Johnson");

    assert_eq!(line.get("other_name").is_none(), true);
    assert_eq!(line.get("").is_none(), true);
    
    let mut empty_line = Line::new();
    assert_eq!(empty_line.get("firstname").is_none(), true);
}

#[test]
fn test_get_index() {
    let line = _init_line();

    assert_eq!(line.get_index("firstname").unwrap(), 0);
    assert_eq!(line.get_index("favorite_number").unwrap(), 2);
    assert_eq!(line.get_index("lastname").unwrap(), 1);

    assert_eq!(line.get_index("does_not_exist").is_none(), true);
}

#[test]
fn test_remove() {
    let mut line = _init_line();

    let index = line.get_index("favorite_number").unwrap();
    assert_eq!(index, 2);
    assert_eq!(line.get_fields().len(), 3);

    line.remove("lastname");
    assert_eq!(line.get_fields().len(), 2);

    line.remove("does_not_exist");
    assert_eq!(line.get_fields().len(), 2);

    let index = line.get_index("favorite_number").unwrap();
    assert_eq!(index, 1);
}

#[test]
fn test_get_field() {
    let line = _init_line();

    let names = line.get_fields_name();

    assert_eq!(names.contains(&"firstname"), true);
    assert_eq!(names.contains(&"lastname"), true);
    assert_eq!(names.contains(&"favorite_number"), true);

    assert_eq!(line.get_fields_name().contains(&"other"), false);


    let line = Line::new();
    assert_eq!(line.get_fields_name().len(), 0);
}

fn _init_line() -> Line {
    let mut line = Line::new();
    line.add("firstname", Type::from_str("Mike")).unwrap();
    line.add("lastname", Type::from_str("Johnson")).unwrap();
    line.add("favorite_number", Type::from_str("1245")).unwrap();

    line
}