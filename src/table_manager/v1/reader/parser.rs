use rust_decimal::Decimal;
use rust_decimal::prelude::*;
use uuid::Uuid;

use super::lexer::Lexer;
use crate::db::db_error::DbError;
use crate::db::line::Line;
use crate::db::field::Field;

pub struct Parser {
    version: String,
    pub lines: Vec<Line>
}

impl Parser {
    pub fn new(lexer: &mut Lexer) -> Result<Parser, DbError> {
        let mut p = Parser { version: String::from(""), lines: vec![] };
        p.init(lexer)?;

        Ok(p)
    }

    pub fn init(&mut self, lexer: &mut Lexer) -> Result<(), DbError> {
        lexer.consume_and_check("#")?;

        lexer.consume_and_check("v1.0")?;
        self.version = String::from("v1.0");

        lexer.consume_and_check("#")?;

        loop {
            match lexer.peek() {
                None => break,
                Some(c) => {
                    if c == "[" {
                        self.lines.push(Self::parse_line(lexer)?);
                    }
                    else {
                        let msg = String::from("Unexpected token! [") + c + "]";
                        return Err(DbError::Custom(msg));
                    }
                }
            }
        }

        Ok(())
    }

    fn parse_line(lexer: &mut Lexer) -> Result<Line, DbError> {
        lexer.consume_and_check("[")?;

        lexer.consume_and_check("_id")?;
        lexer.consume_and_check(":")?;
        lexer.consume_and_check("\"")?;
        let id = String::from(lexer.consume_err_if_none()?);
        lexer.consume_and_check("\"")?;
        lexer.consume_if(" ");

        let mut fields: Vec<Field> = vec![];
        loop {
            let peek = lexer.peek();
            let should_break = match peek {
                Some(p) => p == "]",
                None => true
            };

            if should_break {
                break;
            }
             

            let col = String::from(lexer.consume_err_if_none()?); 
            lexer.consume_and_check(":")?;

            let val = Self::loop_for_value(lexer)?;

            fields.push(Parser::build_field(lexer, &col, &val)?);

            lexer.consume_if(" ");
        }

        let id = match Uuid::parse_str(&id) {
            Ok(id) => id,
            Err(error) => return Err(DbError::Custom(error.to_string()))
        };

        lexer.consume_and_check("]")?;

        Ok(Line::new_with_id(id, fields))
    }

    fn build_field(lexer: &mut Lexer, name: &str, value: &str) -> Result<Field, DbError> {
        lexer.consume_and_check(":")?;

        let type_name = lexer.consume_err_if_none()?;
        let field = match type_name {
            "string" => Field::new_str(name, value),
            "integer" => Field::new_int(name, Parser::str_to_int(value)?),
            "decimal" => Field::new_decimal(name, Parser::str_to_decimal(value)?),
            "boolean" => Field::new_bool(name, Parser::str_to_bool(value)),
            _ => {
                let msg = String::from("The type [") + type_name + "] is not supported";
                return Err(DbError::Custom(msg));
            }
        };

        Ok(field)
    }

    fn str_to_int(value: &str) -> Result<i64, DbError> {
        match i64::from_str(value) {
            Ok(i) => Ok(i),
            Err(error) => Err(DbError::Custom(error.to_string()))
        }
    }

    fn str_to_bool(value: &str) -> bool {
        let mut b = false;
        if value == "true" {
            b = true;
        }

        b
    }

    fn str_to_decimal(value: &str) -> Result<Decimal, DbError> {
        match Decimal::from_str(value) {
            Ok(i) => Ok(i),
            Err(error) => Err(DbError::Custom(error.to_string()))
        }
    }

    fn loop_for_value(lexer: &mut Lexer) -> Result<String, DbError> {
        let mut value = String::from("");
        lexer.consume_and_check("\"")?;

        loop {
            match lexer.peek() {
                Some(peek) => {
                    if peek == "\"" {
                        if Parser::is_escaped_quote(lexer) {
                            lexer.consume();
                            lexer.consume();
                            value.push_str("\"");
                        }
                        else {
                            lexer.consume_and_check("\"")?;
                            break;
                        }
                    }
                    else {
                        value.push_str(lexer.consume_err_if_none()?);
                    }
                },
                None => {
                    let msg =String::from("Could not find the end of the value [") + &value + "]"; 
                    return Err(DbError::Custom(msg));
                }
            }
        }

        Ok(value)
    }

    fn is_escaped_quote(lexer: &mut Lexer) -> bool {
        let mut escaped = false;
        if let Some(peek) = lexer.peek() {
            if let Some(peek_at) = lexer.peek_at(1) {
                if peek == "\"" && peek_at == "\"" {
                    escaped = true;
                }
            }
        }

        escaped
    }

}

impl std::fmt::Debug for Parser {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        println!("Debug - Parser");
        for line in &self.lines {
            println!("ID: {}", line.get_id());
            for f in line.get_fields() {
                println!("{}: {}", f.get_name(), f.get().to_string());
            }
        }

        Ok(())
    }
}


#[cfg(test)]
mod test {
    use uuid::Uuid;

    use crate::db::db_error::DbError;
    use crate::table_manager::v1::reader::{lexer::Lexer, parser::Parser};

    #[test]
    fn test_parser() {
        let to_parser = String::from("#v1.0#[_id:\"5435c914-a918-4cc7-8354-e55ff04d9e25\" col1:\"123\":integer col2:\"456\":integer col3:\"789.2333\":decimal]") +
                                                "[_id:\"3b3f4537-1b8b-4577-999f-e650ea76e190\" name:\"client\":string full:\"Mike Mike\":string col3:\"Using \"\" in a text\":string]" +
                                                "[_id:\"7810da2f-84c7-4897-a0e1-8d92ecefadb4\" name:\"Brackets in a value []\":string full:\"\":string]";
    
        let mut l = Lexer::new(&to_parser);
        let p = Parser::new(&mut l).unwrap();
        let lines = p.lines;
    
        assert_eq!(p.version, "v1.0");
    
        assert_eq!(lines[0].get_id(), &Uuid::parse_str("5435c914-a918-4cc7-8354-e55ff04d9e25").unwrap());
        assert_eq!(lines[0].get_fields()[0].get_name(), "col1");
        assert_eq!(lines[0].get_fields()[0].get().to_string(), "123");
        assert_eq!(lines[0].get_fields()[2].get().to_string(), "789.2333");
    
        assert_eq!(lines[1].get_fields()[0].get_name(), "name");
        assert_eq!(lines[1].get_fields()[0].get().to_string(), "client");
        assert_eq!(lines[1].get_fields()[1].get().to_string(), "Mike Mike");
        assert_eq!(lines[1].get_fields()[2].get_name(), "col3");
        assert_eq!(lines[1].get_fields()[2].get().to_string(), "Using \" in a text");
    
        assert_eq!(lines[2].get_fields()[0].get().to_string(), "Brackets in a value []");
    }
    
    #[test]
    fn test_invalid_format() {
        // Missing version
        let parser = _get_parser_from_str("##[_id:\"5435c914-a918-4cc7-8354-e55ff04d9e25\" col1:\"123\":string col2:\"456\":string col3:\"789\":string]");
        assert_eq!(parser.is_err(), true);
        assert_eq!(_unwrap_custom_error(&parser.unwrap_err()), "Expected [v1.0], but received [#]");
    
        // Missing first [
        let parser = _get_parser_from_str("#v1.0#_id:\"5435c914-a918-4cc7-8354-e55ff04d9e25\" col1:\"123\" col2:\"456\" col3:\"789\"]");
        assert_eq!(parser.is_err(), true);
        assert_eq!(_unwrap_custom_error(&parser.unwrap_err()), "Unexpected token! [_id]");
    
        // Missing last ]
        let parser = _get_parser_from_str("#v1.0#[_id:\"5435c914-a918-4cc7-8354-e55ff04d9e25\"");
        assert_eq!(parser.is_err(), true);
        assert_eq!(_unwrap_custom_error(&parser.unwrap_err()), "Expected []], but received []");
    
        // Missing first " (on id)
        let parser = _get_parser_from_str("#v1.0#[_id:5435c914-a918-4cc7-8354-e55ff04d9e25\" col1:\"123\"]:string");
        assert_eq!(parser.is_err(), true);
        assert_eq!(_unwrap_custom_error(&parser.unwrap_err()), "Expected [\"], but received [5435c914-a918-4cc7-8354-e55ff04d9e25]");
    
        // Missing first " (on field)
        let parser = _get_parser_from_str("#v1.0#[_id:\"5435c914-a918-4cc7-8354-e55ff04d9e25\" col1:123\"]:string");
        assert_eq!(parser.is_err(), true);
        assert_eq!(_unwrap_custom_error(&parser.unwrap_err()), "Expected [\"], but received [123]");
    
        // Missing last " (on id)
        let parser = _get_parser_from_str("#v1.0#[_id:\"5435c914-a918-4cc7-8354-e55ff04d9e25 col1:\"123\"]:string");
        assert_eq!(parser.is_err(), true);
        assert_eq!(_unwrap_custom_error(&parser.unwrap_err()), "Expected [\"], but received [ ]");
    
        // Missing last " (on field)
        let parser = _get_parser_from_str("#v1.0#[_id:\"5435c914-a918-4cc7-8354-e55ff04d9e25\" col1:\"123]:string");
        assert_eq!(parser.is_err(), true);
        assert_eq!(_unwrap_custom_error(&parser.unwrap_err()), "Could not find the end of the value [123]:]");
    
        // Missing :
        let parser = _get_parser_from_str("#v1.0#[_id:\"5435c914-a918-4cc7-8354-e55ff04d9e25\" col1 \"123\":string col2:\"456\":string col3:\"789\":string][_id:\"3b3f4537-1b8b-4577-999f-e650ea76e190\" name:\"client\":string full:\"Mike Mike\":string col3:\"Using \"\" in a text\":string]");
        assert_eq!(parser.is_err(), true);
        assert_eq!(_unwrap_custom_error(&parser.unwrap_err()), "Expected [:], but received [ ]");
    
        // Using ; instead of :
        let parser = _get_parser_from_str("#v1.0#[_id;\"5435c914-a918-4cc7-8354-e55ff04d9e25\" col1:\"123\":string col2:\"456\":string col3:\"789\":string]");
        assert_eq!(parser.is_err(), true);
        assert_eq!(_unwrap_custom_error(&parser.unwrap_err()), "Expected [_id], but received [_id;]");
    
        // Using : before type
        let parser = _get_parser_from_str("#v1.0#[_id:\"5435c914-a918-4cc7-8354-e55ff04d9e25\" col1:\"123\" string col2:\"456\":string col3:\"789\":string]");
        assert_eq!(parser.is_err(), true);
        assert_eq!(_unwrap_custom_error(&parser.unwrap_err()), "Expected [:], but received [ ]");
    
        // Using unsupported type
        let parser = _get_parser_from_str("#v1.0#[_id:\"5435c914-a918-4cc7-8354-e55ff04d9e25\" col1:\"123\":ssstringg col2:\"456\":string col3:\"789\":string]");
        assert_eq!(parser.is_err(), true);
        assert_eq!(_unwrap_custom_error(&parser.unwrap_err()), "The type [ssstringg] is not supported");
    }
    
    fn _get_parser_from_str(to_parse: &str) -> Result<Parser, DbError> {
        let mut l = Lexer::new(to_parse);
    
        Parser::new(&mut l)
    }
    
    fn _unwrap_custom_error(error: &DbError) -> String {
        match error {
            DbError::Custom(msg) => msg.to_owned(), 
            _ => "".to_owned()
        }
    }
}
