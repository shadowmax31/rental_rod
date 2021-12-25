use crate::table_manager::{TableManagerVersion, self};

use super::line::{Line, Field};
use uuid::Uuid;

pub struct Table {
    pub name: String,
    pub lines: Vec<Line>
}

impl Table {
    pub fn new(name: &str, lines: Vec<Line>) -> Table {
        Table { 
            name: String::from(name), 
            lines: lines 
        }
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

    let fields = vec![ Field::new("firstname", "Paul"), Field::new("lastname", "Silly"), Field::new("favorite_number", "12") ];
    let line = Line::new_with_id(Uuid::parse_str("e4ee24eb-f84c-46ed-b8af-16e7891792e1").unwrap(), fields);
    lines.push(line);

    let fields = vec![ Field::new("firstname", "Bob"), Field::new("lastname", "Bob"), Field::new("favorite_number", "760") ];
    let line = Line::new_with_id(Uuid::parse_str("9f77958d-378a-4aab-9763-c815cd74f2bd").unwrap(), fields);
    lines.push(line);

    Table::new("test", lines)
}