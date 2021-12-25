use super::line::Line;

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
}