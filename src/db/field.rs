use std::fs::File;

use rust_decimal::Decimal;

use super::field_type::Type;

#[derive(Debug)]
pub struct Field {
    name: String,
    value: Type,
}

impl Field {
    pub fn new_str(name: &str, value: &str) -> Field {
        Field { 
            name: String::from(name), 
            value: Type::from_str(value)
        }
    }

    pub fn new_int(name: &str, value: i64) -> Field {
        Field {
            name: String::from(name),
            value: Type::from_int(value)
        }
    }

    pub fn new_decimal(name: &str, value: Decimal) -> Field {
        Field {
            name: String::from(name),
            value: Type::from_decimal(value)
        }
    }

    pub fn set(&mut self, value: Type) {
        self.value = value;
    }

    pub fn get(&self) -> &Type {
        &self.value
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

}