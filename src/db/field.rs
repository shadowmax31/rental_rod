#[derive(Debug)]
pub struct Field {
    pub name: String,
    pub value: String
}

impl Field {
    pub fn new(name: &str, value: &str) -> Field {
        Field { 
            name: String::from(name), 
            value: String::from(value) 
        }
    }
}