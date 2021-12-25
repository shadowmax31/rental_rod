use crate::table_manager::{TableManagerVersion, self};

use super::line::Line;
use super::field::Field;
use super::db_error::DbError;
use uuid::Uuid;

pub struct Table {
    pub name: String,
    pub lines: Vec<Line>
}

impl Table {
    pub fn new(name: &str, lines: Vec<Line>) -> Result<Table, DbError> {
        if let Some(id) = Table::check_for_duplicate_id(&lines) {
            let msg = String::from("The id [") + &id.to_string() + "] is used multiple times";
            return Err(DbError::Custom(msg));
        }

        Ok (Table { 
            name: String::from(name), 
            lines: lines 
        })
    }

    fn check_for_duplicate_id(lines: &Vec<Line>) -> Option<&Uuid> {
        let mut found = None;

        let mut checked: Vec<&Uuid> = Vec::new();
        for line in lines {
            if !checked.contains(&&line.id) {
                checked.push(&line.id);
            }
            else {
                found = Some(&line.id);
                break;
            }
        }

        found
    }

    pub fn find(&self, name: &str, value: &str) -> Vec<&Line> {
        let mut list: Vec<&Line> = Vec::new();

        for line in &self.lines {
            if let Some(found) = line.get(name) {
                if value == found {
                    list.push(&line);
                }
            }
        }

        list
    }

    pub fn find_by_id(&self, id: Uuid) -> Option<&Line> {
        let mut found: Option<&Line> = None;
        for line in &self.lines {
            if line.id == id {
                found = Some(line);
                break;
            }
        }

        found
    }

    pub fn insert(&mut self, line: Line) {
        self.lines.push(line);
    }

}

#[test]
fn test_find_by_id() {
    let tbl = _init_basic_table();
    let line = tbl.find_by_id(Uuid::parse_str("00000000-0000-0000-0000-000000000000").unwrap());
    
    assert_eq!(line.is_none(), true);

    let line = tbl.find_by_id(Uuid::parse_str("84e4eedf-a383-457e-aa73-d26c646762ba").unwrap());
    assert_eq!(line.is_some(), true);
    let line = line.unwrap();
    assert_eq!("84e4eedf-a383-457e-aa73-d26c646762ba", line.id.to_string());

    let line = tbl.find_by_id(Uuid::parse_str("a60cbdfa-4c46-438c-8ad8-45bdd2063a56").unwrap());
    assert_eq!(line.is_some(), true);
    let line = line.unwrap();
    assert_eq!("a60cbdfa-4c46-438c-8ad8-45bdd2063a56", line.id.to_string());
}

#[test]
fn test_should_not_allow_identical_id() {

    // Using the same object
    let mut lines: Vec<Line> = Vec::new();
    let id = Uuid::new_v4();

    let fields = vec![ Field::new("firstname", "Mike"), Field::new("lastname", "Johnson"), Field::new("favorite_number", "1245") ];
    let line = Line::new_with_id(id, fields);
    lines.push(line);

    let fields = vec![ Field::new("firstname", "Sean"), Field::new("lastname", "Smith"), Field::new("favorite_number", "256") ];
    let line = Line::new_with_id(id, fields);
    lines.push(line);

    let table = Table::new("test", lines);
    assert_eq!(table.is_err(), true);


    // Using different references of the same Uuid
    let id1 = Uuid::parse_str("187de314-404d-439b-8a68-58122ea12261").unwrap();
    let id2 = Uuid::parse_str("187de314-404d-439b-8a68-58122ea12261").unwrap();
    let mut lines: Vec<Line> = Vec::new();

    let fields = vec![ Field::new("firstname", "Mike"), Field::new("lastname", "Johnson"), Field::new("favorite_number", "1245") ];
    let line = Line::new_with_id(id1, fields);
    lines.push(line);

    let fields = vec![ Field::new("firstname", "Sean"), Field::new("lastname", "Smith"), Field::new("favorite_number", "256") ];
    let line = Line::new_with_id(id2, fields);
    lines.push(line);

    let table = Table::new("test", lines);
    assert_eq!(table.is_err(), true);

}

#[test]
fn test_find() {
    let table = _init_basic_table();

    let list = table.find("firstname", "Simon");
    assert_eq!(list.len(), 2);
    assert_eq!(list[0].id.to_string(), "a60cbdfa-4c46-438c-8ad8-45bdd2063a56");
    assert_eq!(list[1].id.to_string(), "49295823-29c2-1dba-2d14-ad498654ecc2");


    let list = table.find("favorite_number", "1245");
    assert_eq!(list.len(), 1);
    assert_eq!(list[0].id.to_string(), "84e4eedf-a383-457e-aa73-d26c646762ba");

    let list = table.find("a_field", "some value");
    assert_eq!(list.len(), 0);
}


fn _init_basic_table() -> Table {
    let mut lines: Vec<Line> = Vec::new();

    let fields = vec![ Field::new("firstname", "Mike"), Field::new("lastname", "Johnson"), Field::new("favorite_number", "1245") ];
    let line = Line::new_with_id(Uuid::parse_str("84e4eedf-a383-457e-aa73-d26c646762ba").unwrap(), fields);
    lines.push(line);

    let fields = vec![ Field::new("firstname", "Sean"), Field::new("lastname", "Smith"), Field::new("favorite_number", "256") ];
    let line = Line::new_with_id(Uuid::parse_str("187de314-404d-439b-8a68-58122ea12261").unwrap(), fields);
    lines.push(line);

    let fields = vec![ Field::new("firstname", "Simon"), Field::new("lastname", "Neat"), Field::new("favorite_number", "540") ];
    let line = Line::new_with_id(Uuid::parse_str("a60cbdfa-4c46-438c-8ad8-45bdd2063a56").unwrap(), fields);
    lines.push(line);

    let fields = vec![ Field::new("firstname", "Simon"), Field::new("lastname", "Neat"), Field::new("favorite_number", "540") ];
    let line = Line::new_with_id(Uuid::parse_str("49295823-29c2-1dba-2d14-ad498654ecc2").unwrap(), fields);
    lines.push(line);

    let fields = vec![ Field::new("firstname", "Paul"), Field::new("lastname", "Silly"), Field::new("favorite_number", "12") ];
    let line = Line::new_with_id(Uuid::parse_str("e4ee24eb-f84c-46ed-b8af-16e7891792e1").unwrap(), fields);
    lines.push(line);

    let fields = vec![ Field::new("firstname", "Bob"), Field::new("lastname", "Bob"), Field::new("favorite_number", "760") ];
    let line = Line::new_with_id(Uuid::parse_str("9f77958d-378a-4aab-9763-c815cd74f2bd").unwrap(), fields);
    lines.push(line);

    Table::new("test", lines).unwrap()
}