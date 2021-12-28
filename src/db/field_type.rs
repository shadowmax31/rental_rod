use chrono::{Utc, DateTime};
use rust_decimal::Decimal;

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub enum Type {
    String(String),
    Integer(i64),
    Decimal(Decimal),
    Boolean(bool),
    DateTime(DateTime<Utc>)
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

    pub fn from_datetime(dt: DateTime<Utc>) -> Type {
        Type::DateTime(dt)
    }

    pub fn get_type(&self) -> String {
        match self {
            Type::String(_) => "string",
            Type::Integer(_) => "integer",
            Type::Decimal(_) => "decimal",
            Type::Boolean(_) => "boolean",
            Type::DateTime(_) => "datetime"
        }.to_owned()
    }

    pub fn to_string(&self) -> String {
        match self {
            Type::String(v) => v.to_owned(),
            Type::Integer(v) => v.to_string(),
            Type::Decimal(v) => v.to_string(),
            Type::Boolean(v) => v.to_string(),
            Type::DateTime(v) => v.to_rfc3339_opts(chrono::SecondsFormat::Millis, true)
        }
    }
}

#[cfg(test)]
mod test {
    use chrono::{offset::Utc, Duration, DateTime, NaiveDate, NaiveTime, NaiveDateTime, Timelike};
    use rust_decimal::{prelude::FromPrimitive, Decimal};
    use std::{str::FromStr, ops::Add};

    use crate::db::field_type::Type;

    #[test]
    fn test_to_string() {
        let int = Type::from_int(0);
        let str = Type::from_str("hello");
        let dec = Type::from_decimal(Decimal::from_f64(1.11).unwrap());
        let boolean = Type::from_bool(false);

        let date = NaiveDate::from_ymd(2021, 12, 15);
        let time = NaiveTime::from_hms(17, 18, 19);
        let datetime = DateTime::from_utc(NaiveDateTime::new(date, time), Utc);
        let datetime = Type::from_datetime(datetime);

        assert_eq!(int.to_string(), "0");
        assert_eq!(str.to_string(), "hello");
        assert_eq!(dec.to_string(), "1.11");
        assert_eq!(boolean.to_string(), "false");
        assert_eq!(datetime.to_string(), "2021-12-15T17:18:19.000Z");

        let date = NaiveDate::from_ymd(2020, 11, 14);
        let time = NaiveTime::from_hms(10, 20, 30).with_nanosecond(149294584).unwrap();
        let datetime = DateTime::from_utc(NaiveDateTime::new(date, time), Utc);
        let datetime = Type::from_datetime(datetime);


        assert_eq!(datetime.to_string(), "2020-11-14T10:20:30.149Z");
    }


    #[test]
    fn test_mix_and_match() {
        let int = Type::from_int(0);
        let str = Type::from_str("hello");
        let dec = Type::from_decimal(Decimal::from_f64(1.11).unwrap());
        let dec1 = Type::from_decimal(Decimal::from_i64(0).unwrap());
        let boolean = Type::from_bool(false);
        let dt = Type::from_datetime(Utc::now());

        assert_ne!(int, str);
        assert_ne!(int, dec);
        assert_ne!(int, dec1);
        assert_ne!(int, boolean);
        assert_ne!(int, dt);

        assert_ne!(str, int);
        assert_ne!(str, dec);
        assert_ne!(str, dec1);
        assert_ne!(str, boolean);
        assert_ne!(str, boolean);

        assert_ne!(dec, int);
        assert_ne!(dec, str);
        assert_ne!(dec, dec1);
        assert_ne!(dec, boolean);
        assert_ne!(dec, dt);
    }

    #[test]
    fn test_datetime_eq() {
        let now = Utc::now();
        let dt_now = Type::from_datetime(now);
        let dt_now1 = Type::from_datetime(now);

        let dt_later = Type::from_datetime(now.add(Duration::days(1)));

        assert_eq!(dt_now, dt_now);
        assert_eq!(dt_now, dt_now1);
        assert_ne!(dt_now, dt_later);
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