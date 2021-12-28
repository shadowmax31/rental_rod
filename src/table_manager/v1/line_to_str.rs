
use crate::db::field_type::Type;
use crate::db::line::Line;
use crate::db::field::Field;

/// Converts a Line to a string with the V1 format
pub fn line_to_str(line: &Line) -> String {
    let mut text = String::from("");
    
    append_field(&mut text, "_id", &line.get_id().to_hyphenated().to_string());
    let fields = fields_to_str(&line.get_fields());
    
    text.push_str(" ");
    text.push_str(&fields);
    
    text.insert_str(0, "[");
    text.push_str("]");
    
    text
}

fn fields_to_str(fields: &Vec<Field>) -> String {
    let mut text = String::from("");
    
    for field in fields {
        append_field_with_type(&mut text, &field.get_name(), &field.get());
    }
    
    text
}

fn append_field_with_type(fields: &mut String, name: &str, value: &Type) { 
    append_field(fields, name, &value.to_string());
    fields.push_str(":");
    fields.push_str(&value.get_type());
}

fn append_field(fields: &mut String, name: &str, value: &str) {
    if fields != "" {
        fields.push_str(" ");
    }
    
    fields.push_str(name);
    fields.push_str(":");
    fields.push_str("\"");
    fields.push_str(&value.to_string());
    fields.push_str("\"");
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use rust_decimal::Decimal;
    use uuid::Uuid;

    use crate::db::{line::Line, field::Field};
    use crate::table_manager::v1::line_to_str::line_to_str;

    #[test]
    fn test_ln_to_str() {
        let uid = "e88954bd-3ae5-4cc5-a1c5-839926790dda";
        let line = Line::new_with_id(uuid::Uuid::parse_str(uid).unwrap(), vec![ Field::new_str("col1", "1"), Field::new_str("col2", "2") ]);
        let expect = "[_id:\"".to_owned() + uid + "\" col1:\"1\":string col2:\"2\":string]";
    
        assert_eq!(line_to_str(&line), expect);
    }
    
    #[test]
    fn test_ln_str_with_type() {
        let uid = Uuid::new_v4();
        let line = Line::new_with_id(uid, vec![ Field::new_str("str", "SuperString"), Field::new_int("int", 123), Field::new_decimal("dec", Decimal::from_str("1.230555531").unwrap())]);
        let expect = "[_id:\"".to_owned() + &uid.to_string() + "\" str:\"SuperString\":string int:\"123\":integer dec:\"1.230555531\":decimal]";
    
        assert_eq!(line_to_str(&line), expect);
    }
}
