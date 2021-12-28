use rust_decimal::Decimal;

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub enum Type {
    String(String),
    Integer(i64),
    Decimal(Decimal),
    Boolean(bool)
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

    pub fn from_bool(boolean: bool) -> Type {
        Type::Boolean(boolean)
    }

    pub fn get_type(&self) -> String {
        match self {
            Type::String(_) => "string",
            Type::Integer(_) => "integer",
            Type::Decimal(_) => "decimal",
            Type::Boolean(_) => "boolean"
        }.to_owned()
    }

    pub fn to_string(&self) -> String {
        match self {
            Type::String(v) => v.to_owned(),
            Type::Integer(v) => v.to_string(),
            Type::Decimal(v) => v.to_string(),
            Type::Boolean(v) => v.to_string()
        }
    }
}

#[cfg(test)]
mod test {
    use rust_decimal::{prelude::FromPrimitive, Decimal};
    use std::str::FromStr;

    use crate::db::field_type::Type;

    #[test]
    fn test_mix_and_match() {
        let int = Type::from_int(0);
        let str = Type::from_str("hello");
        let dec = Type::from_decimal(Decimal::from_f64(1.11).unwrap());
        let dec1 = Type::from_decimal(Decimal::from_i64(0).unwrap());
        let boolean = Type::from_bool(false);

        assert_ne!(int, str);
        assert_ne!(int, dec);
        assert_ne!(int, dec1);
        assert_ne!(int, boolean);

        assert_ne!(str, int);
        assert_ne!(str, dec);
        assert_ne!(str, dec1);
        assert_ne!(str, boolean);

        assert_ne!(dec, int);
        assert_ne!(dec, str);
        assert_ne!(dec, dec1);
        assert_ne!(dec, boolean);
    }

    #[test]
    fn test_bool_eq() {
        let b_true = Type::from_bool(true);
        let b_true1 = Type::from_bool(true);

        let b_false = Type::from_bool(false);

        assert_eq!(b_true, b_true);
        assert_eq!(b_true, b_true1);
        assert_ne!(b_true, b_false);
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
}