use uuid::Uuid;

use crate::util::file;
use super::field::Field;

#[derive(Debug)]
pub struct Line {
    pub id: Uuid,
    pub fields: Vec<Field>
}

impl Line {
    pub fn new(fields: Vec<Field>) -> Line {
        Line::new_with_id(Uuid::new_v4(), fields)
    }

    pub fn new_with_id(id: Uuid, fields: Vec<Field>) -> Line {
        Line { id, fields }
    }

    pub fn get(&self, field_name: &str) -> Option<&str> {
        let mut found: Option<&str> = None;
        for field in &self.fields {
            if field.name == field_name {
                found = Some(&field.value);
                break;
            }
        }

        found
    }

    pub fn get_fields_name(&self) -> Vec<&str> {
        let mut fields: Vec<&str> = Vec::new();
        for f in &self.fields {
            fields.push(&f.name);
        }

        fields
    }
}


#[test]
fn test_get() {
    let line = _init_line();

    assert_eq!(line.get("firstname").unwrap(), "Mike");
    assert_eq!(line.get("favorite_number").unwrap(), "1245");
    assert_eq!(line.get("lastname").unwrap(), "Johnson");

    assert_eq!(line.get("other_name").is_none(), true);
    assert_eq!(line.get("").is_none(), true);
    
    let empty_line = Line::new(vec![]);
    assert_eq!(empty_line.get("firstname").is_none(), true);
}

#[test]
fn test_get_field() {
    let line = _init_line();

    let names = line.get_fields_name();

    assert_eq!(names.contains(&"firstname"), true);
    assert_eq!(names.contains(&"lastname"), true);
    assert_eq!(names.contains(&"favorite_number"), true);

    assert_eq!(line.get_fields_name().contains(&"other"), false);


    let line = Line::new(vec![]);
    assert_eq!(line.get_fields_name().len(), 0);
}

fn _init_line() -> Line {
    let fields = vec![ Field::new("firstname", "Mike"), Field::new("lastname", "Johnson"), Field::new("favorite_number", "1245") ];

    Line::new(fields)
}