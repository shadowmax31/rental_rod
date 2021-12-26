use std::fs::File;

#[derive(Debug)]
pub struct Field {
    name: String,
    value: String
}

impl Field {
    pub fn new(name: &str, value: &str) -> Field {
        Field { 
            name: String::from(name), 
            value: String::from(value) 
        }
    }

    pub fn set(&mut self, value: &str) {
        self.value = String::from(value);
    }

    pub fn get(&self) -> &str {
        &self.value
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

}