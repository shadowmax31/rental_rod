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
        self.version = lexer.consume().unwrap().to_owned();
        lexer.consume_and_check("#")?;

        loop {
            match lexer.peek() {
                None => break,
                Some(c) => {
                    if c == "[" {
                        self.lines.push(Self::parse_line(lexer)?);
                    }
                    else {
                        let msg = String::from("Unexpected token!");
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
        let id = String::from(lexer.consume().unwrap());
        lexer.consume_and_check("\"")?;
        lexer.consume_if(" ");

        let mut fields: Vec<Field> = vec![];
        loop {
            let peek = lexer.peek();
            if peek.is_none() || peek.unwrap() == "]" {
                break;
            }
             

            let col = String::from(lexer.consume().unwrap()); // Col
            lexer.consume_and_check(":")?;

            let val = Self::loop_for_value(lexer)?;

            fields.push(Field::new(&col, &val));

            lexer.consume_if(" ");
        }

        let id = Uuid::parse_str(&id).unwrap();

        lexer.consume_and_check("]")?;

        Ok(Line::new_with_id(id, fields))
    }

    fn loop_for_value(lexer: &mut Lexer) -> Result<String, DbError> {
        let mut value = String::from("");
        lexer.consume_and_check("\"")?;

        loop {
            let peek = lexer.peek().unwrap();

            if peek == "\"" {
                let peek = lexer.peek_at(1).unwrap();
                if peek == "\"" {
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
                value.push_str(lexer.consume().unwrap());
            }
        }

        Ok(value)
    }

}

impl std::fmt::Debug for Parser {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        println!("Debug - Parser");
        for line in &self.lines {
            println!("ID: {}", line.get_id());
            for f in line.get_fields() {
                println!("{}: {}", f.get_name(), f.get());
            }
        }

        Ok(())
    }
}

#[test]
fn test_parser() {
    let to_parser = "#v1.0#[_id:\"5435c914-a918-4cc7-8354-e55ff04d9e25\" col1:\"123\" col2:\"456\" col3:\"789\"][_id:\"3b3f4537-1b8b-4577-999f-e650ea76e190\" name:\"client\" full:\"Mike Mike\" col3:\"Using \"\" in a text\"]";

    let mut l = Lexer::new(to_parser);
    let p = Parser::new(&mut l).unwrap();
    let lines = p.lines;

    assert_eq!(p.version, "v1.0");

    assert_eq!(lines[0].get_id(), &Uuid::parse_str("5435c914-a918-4cc7-8354-e55ff04d9e25").unwrap());
    assert_eq!(lines[0].get_fields()[0].get_name(), "col1");
    assert_eq!(lines[0].get_fields()[0].get(), "123");
    assert_eq!(lines[0].get_fields()[2].get(), "789");

    assert_eq!(lines[1].get_fields()[0].get_name(), "name");
    assert_eq!(lines[1].get_fields()[0].get(), "client");
    assert_eq!(lines[1].get_fields()[1].get(), "Mike Mike");
    assert_eq!(lines[1].get_fields()[2].get_name(), "col3");
    assert_eq!(lines[1].get_fields()[2].get(), "Using \" in a text");
}
