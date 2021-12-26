use super::line::Line;
use super::field::Field;
use super::db_error::DbError;
use uuid::Uuid;

pub struct Table {
    name: String,
    lines: Vec<Line>
}

/**
 * The Table object should allow to do every operation on the Database.
 * 
 * It should allow to delete lines, update lines, delele fields (all in memory)
 * It does not know about any file format.
 * 
 * To manage the undos, it should also manage a "dirty" field to know what was changed (maybe...)
 */
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
            if !checked.contains(&line.get_id()) {
                checked.push(line.get_id());
            }
            else {
                found = Some(line.get_id());
                break;
            }
        }

        found
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_lines(&mut self) -> Vec<&mut Line> {
        self.find_where(|_| true)
    }

    pub fn find(&mut self, name: &str, value: &str) -> Vec<&mut Line> {
        self.find_where(|line| {
            let field = line.get(name);
            match field {
                Some(f) => f.get().to_string() == value,
                None => false
            }
        })
    }

    pub fn find_where<F>(&mut self, filter: F) -> Vec<&mut Line>
        where F: Fn(&mut Line) -> bool {
        let mut list: Vec<&mut Line> = Vec::new();

        for line in &mut self.lines {
            if filter(line) {
                list.push(line);
            }
        }
        
        list
    }

    pub fn find_by_id(&mut self, id: Uuid) -> Option<&mut Line> {
        let mut found: Option<&mut Line> = None;
        for line in &mut self.lines {
            if line.get_id() == &id {
                found = Some(line);
                break;
            }
        }

        found
    }

    pub fn insert(&mut self, line: Line) {
        self.lines.push(line);
    }

    pub fn print(&self) {
        for line in &self.lines {
            println!("{:?}", line);
        }
    }

}

#[test]
fn test_find_by_id() {
    let mut tbl = _init_basic_table();
    let line = tbl.find_by_id(Uuid::parse_str("00000000-0000-0000-0000-000000000000").unwrap());
    
    assert_eq!(line.is_none(), true);

    let line = tbl.find_by_id(Uuid::parse_str("84e4eedf-a383-457e-aa73-d26c646762ba").unwrap());
    assert_eq!(line.is_some(), true);
    let line = line.unwrap();
    assert_eq!("84e4eedf-a383-457e-aa73-d26c646762ba", line.get_id().to_string());

    let line = tbl.find_by_id(Uuid::parse_str("a60cbdfa-4c46-438c-8ad8-45bdd2063a56").unwrap());
    assert_eq!(line.is_some(), true);
    let line = line.unwrap();
    assert_eq!("a60cbdfa-4c46-438c-8ad8-45bdd2063a56", line.get_id().to_string());
}

#[test]
fn test_should_not_allow_identical_id() {

    // Using the same object
    let mut lines: Vec<Line> = Vec::new();
    let id = Uuid::new_v4();

    let fields = vec![ Field::new_str("firstname", "Mike"), Field::new_str("lastname", "Johnson"), Field::new_str("favorite_number", "1245") ];
    let line = Line::new_with_id(id, fields);
    lines.push(line);

    let fields = vec![ Field::new_str("firstname", "Sean"), Field::new_str("lastname", "Smith"), Field::new_str("favorite_number", "256") ];
    let line = Line::new_with_id(id, fields);
    lines.push(line);

    let table = Table::new("test", lines);
    assert_eq!(table.is_err(), true);


    // Using different references of the same Uuid
    let id1 = Uuid::parse_str("187de314-404d-439b-8a68-58122ea12261").unwrap();
    let id2 = Uuid::parse_str("187de314-404d-439b-8a68-58122ea12261").unwrap();
    let mut lines: Vec<Line> = Vec::new();

    let fields = vec![ Field::new_str("firstname", "Mike"), Field::new_str("lastname", "Johnson"), Field::new_str("favorite_number", "1245") ];
    let line = Line::new_with_id(id1, fields);
    lines.push(line);

    let fields = vec![ Field::new_str("firstname", "Sean"), Field::new_str("lastname", "Smith"), Field::new_str("favorite_number", "256") ];
    let line = Line::new_with_id(id2, fields);
    lines.push(line);

    let table = Table::new("test", lines);
    assert_eq!(table.is_err(), true);

}

#[test]
fn test_find() {
    let mut table = _init_basic_table();

    let list = table.find("firstname", "Simon");
    assert_eq!(list.len(), 2);
    assert_eq!(list[0].get_id().to_string(), "a60cbdfa-4c46-438c-8ad8-45bdd2063a56");
    assert_eq!(list[1].get_id().to_string(), "49295823-29c2-1dba-2d14-ad498654ecc2");


    let list = table.find("favorite_number", "1245");
    assert_eq!(list.len(), 1);
    assert_eq!(list[0].get_id().to_string(), "84e4eedf-a383-457e-aa73-d26c646762ba");

    let list = table.find("a_field", "some value");
    assert_eq!(list.len(), 0);
}

#[test]
fn test_find_where() {
    let mut table = _init_basic_table();

    let lines = table.find_where(|line| {
        if let Some(field) = line.get("lastname") {
            return field.get().to_string().starts_with("S");
        }

        false
    });

    assert_eq!(lines.len(), 2);
    assert_eq!(lines[0].get_id().to_string(), String::from("187de314-404d-439b-8a68-58122ea12261"));
    assert_eq!(lines[1].get_id().to_string(), String::from("e4ee24eb-f84c-46ed-b8af-16e7891792e1"));


    let lines = table.find_where(|_| false);
    assert_eq!(lines.len(), 0);

    let lines = table.find_where(|_| true);
    assert_eq!(lines.len(), table.lines.len());
}


fn _init_basic_table() -> Table {
    let mut table = Table::new("test", vec![]).unwrap();

    let fields = vec![ Field::new_str("firstname", "Mike"), Field::new_str("lastname", "Johnson"), Field::new_str("favorite_number", "1245") ];
    let line = Line::new_with_id(Uuid::parse_str("84e4eedf-a383-457e-aa73-d26c646762ba").unwrap(), fields);
    table.insert(line);

    let fields = vec![ Field::new_str("firstname", "Sean"), Field::new_str("lastname", "Smith"), Field::new_str("favorite_number", "256") ];
    let line = Line::new_with_id(Uuid::parse_str("187de314-404d-439b-8a68-58122ea12261").unwrap(), fields);
    table.insert(line);

    let fields = vec![ Field::new_str("firstname", "Simon"), Field::new_str("lastname", "Neat"), Field::new_str("favorite_number", "540") ];
    let line = Line::new_with_id(Uuid::parse_str("a60cbdfa-4c46-438c-8ad8-45bdd2063a56").unwrap(), fields);
    table.insert(line);

    let fields = vec![ Field::new_str("firstname", "Simon"), Field::new_str("lastname", "Neat"), Field::new_str("favorite_number", "540") ];
    let line = Line::new_with_id(Uuid::parse_str("49295823-29c2-1dba-2d14-ad498654ecc2").unwrap(), fields);
    table.insert(line);

    let fields = vec![ Field::new_str("firstname", "Paul"), Field::new_str("lastname", "Silly"), Field::new_str("favorite_number", "12") ];
    let line = Line::new_with_id(Uuid::parse_str("e4ee24eb-f84c-46ed-b8af-16e7891792e1").unwrap(), fields);
    table.insert(line);

    let fields = vec![ Field::new_str("firstname", "Bob"), Field::new_str("lastname", "Bob"), Field::new_str("favorite_number", "760") ];
    let line = Line::new_with_id(Uuid::parse_str("9f77958d-378a-4aab-9763-c815cd74f2bd").unwrap(), fields);
    table.insert(line);

    table
}