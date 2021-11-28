mod parser;
mod lexer;

use lexer::Lexer;
use parser::Parser;
use crate::db::line::Line;

pub fn read(lines: &Vec<String>) -> Vec<Line> {
    let str_lines = &vec_to_str(lines);
    let mut lexer = Lexer::new(str_lines);

    let p = Parser::new(&mut lexer);

    println!("{:?}", p);

    Vec::new()
}

fn vec_to_str(lines: &Vec<String>) -> String {
    let mut s = String::from("");

    for line in lines {
        s.push_str(line);
    }

    s
}
