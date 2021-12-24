mod parser;
mod lexer;

use parser::Parser;
use lexer::Lexer;

use crate::db::line::Line;

pub fn read(lines: &Vec<String>) -> Result<Vec<Line>, String> {
    let str_lines = &vec_to_str(lines);
    let mut lexer = Lexer::new(str_lines);

    let p = Parser::new(&mut lexer)?;
    

    Ok(p.lines)
}

fn vec_to_str(lines: &Vec<String>) -> String {
    let mut s = String::from("");

    for line in lines {
        s.push_str(line);
    }

    s
}
