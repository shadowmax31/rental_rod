use uuid::Uuid;

pub struct Line {
    pub id: Uuid,
    pub fields: Vec<Field>
}

pub struct Field {
    pub name: String,
    pub value: String
}

impl Line {
    pub fn new(fields: Vec<Field>) -> Option<Line> {
        let id = Uuid::new_v4();
        let line = Line { id, fields };

        Some(line)
    }
}
