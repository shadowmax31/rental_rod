use std::str::FromStr;

use rust_decimal::{Decimal, prelude::FromPrimitive};

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Type {
    String(String),
    Integer(i64),
    Decimal(Decimal)
}

impl Type {
    pub fn from_str(str: &str) -> Type {
        Type::String(String::from(str))
    }

    pub fn from_int(int: i64) -> Type {
        Type::Integer(int)
    }

    pub fn from_decimal(dec: Decimal) -> Type {
        Type::Decimal(dec)
    }

    pub fn get_type(&self) -> String {
        match self {
            Type::String(_) => "string",
            Type::Integer(_) => "integer",
            Type::Decimal(_) => "decimal"
        }.to_owned()
    }

    pub fn to_string(&self) -> String {
        match self {
            Type::String(v) => v.to_owned(),
            Type::Integer(v) => v.to_string(),
            Type::Decimal(v) => v.to_string()
        }
    }
}

#[test]
fn test_mix_and_match() {
    let int = Type::from_int(1);
    let str = Type::from_str("hello");
    let dec = Type::from_decimal(Decimal::from_f64(1.11).unwrap());
    let dec1 = Type::from_decimal(Decimal::from_i64(1).unwrap());

    assert_ne!(int, str);
    assert_ne!(int, dec);
    assert_ne!(int, dec1);

    assert_ne!(str, int);
    assert_ne!(str, dec);
    assert_ne!(str, dec1);

    assert_ne!(dec, int);
    assert_ne!(dec, str);
    assert_ne!(dec, dec1);
}

#[test]
fn test_int_eq() {
    let int1 = Type::from_int(1);
    let int1_1 = Type::from_int(1);

    let int2 = Type::from_int(2);

    assert_eq!(int1, int1);
    assert_eq!(int1, int1_1);
    assert_ne!(int1, int2);
}

#[test]
fn test_string_eq() {
    let hello = Type::from_str("hello");
    let hello_1 = Type::from_str("hello");

    let world = Type::from_str("world");

    assert_eq!(hello, hello);
    assert_eq!(hello, hello_1);
    assert_ne!(hello, world);
}

#[test]
fn test_decimal_eq() {
    let dec1 = Type::from_decimal(Decimal::from_f64(1.11).unwrap());
    let dec1_1 = Type::from_decimal(Decimal::from_f64(1.11).unwrap());

    let dec2 = Type::from_decimal(Decimal::from_str("2.2222").unwrap());

    assert_eq!(dec1, dec1);
    assert_eq!(dec1, dec1_1);
    assert_ne!(dec1, dec2);
}