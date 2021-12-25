use crate::db::line::Line;
use crate::db::field::Field;

/// Converts a Line to a string with the V1 format
/// ```
/// test_ln_to_str()
/// ```
pub fn line_to_str(line: &Line) -> String {
    let mut text = String::from("");
    
    append_field(&mut text, "_id", &line.id.to_hyphenated().to_string());
    let fields = fields_to_str(&line.fields);
    
    text.push_str(" ");
    text.push_str(&fields);
    
    text.insert_str(0, "[");
    text.push_str("]");
    
    text
}

fn fields_to_str(fields: &Vec<Field>) -> String {
    let mut text = String::from("");
    
    for field in fields {
        append_field(&mut text, &field.name, &field.value);
    }
    
    text
}

fn append_field(fields: &mut String, name: &str, value: &str) {
    if fields != "" {
        fields.push_str(" ");
    }
    
    fields.push_str(name);
    fields.push_str(":");
    fields.push_str("\"");
    fields.push_str(value);
    fields.push_str("\"");
}

#[test]
fn test_ln_to_str() {
    let uid = "e88954bd-3ae5-4cc5-a1c5-839926790dda";
    let line = Line {
        id: uuid::Uuid::parse_str(uid).unwrap(),
        fields: vec![ Field { name: String::from("col1"), value: String::from("1") }, Field { name: String::from("col2"), value: String::from("2") } ]
    };

    let expect = "[_id:\"".to_owned() + uid + "\" col1:\"1\" col2:\"2\"]";

    assert_eq!(line_to_str(&line), expect);
}
