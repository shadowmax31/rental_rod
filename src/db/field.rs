//! Everything related to a field
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

use super::field_type::Type;

#[derive(Debug)]
/**
 * Represent a Single cell from a Table
 * 
 * It needs to have a name and a value
 */
pub struct Field {
    name: String,
    value: Type,
}

impl Field {
    /**
     * Create a new Field from a Type
     */
    pub fn new(name: &str, value: Type) -> Field {
        Field {
            name: String::from(name), 
            value: value
        }
    }

    /**
     * Create a new Field of type String
     */
    pub fn new_str(name: &str, value: &str) -> Field {
        Field::new(name, Type::from_str(value))
    }

    /**
     * Create a new Field of type i64
     */
    pub fn new_int(name: &str, value: i64) -> Field {
        Field::new(name, Type::from_int(value))
    }

    /**
     * Create a new Field of type Decimal
     */
    pub fn new_decimal(name: &str, value: Decimal) -> Field {
        Field::new(name, Type::from_decimal(value))
    }

    /**
     * Create a new Field of type bool
     */
    pub fn new_bool(name: &str, value: bool) -> Field {
        Field::new(name, Type::from_bool(value))
    }

    /**
     * Create a new Field of type DateTime<Utc>
     */
    pub fn new_datetime(name: &str, value: DateTime<Utc>) -> Field {
        Field::new(name, Type::from_datetime(value))
    }

    /**
     * Change the value of the Field
     */
    pub fn set(&mut self, value: Type) {
        self.value = value;
    }

    /**
     * Return the current value of the field
     */
    pub fn get(&self) -> &Type {
        &self.value
    }

    /**
     * Return the name of the field
     */
    pub fn get_name(&self) -> &str {
        &self.name
    }

}