pub struct Lexer<'a> {
    tokens: Vec<&'a str>,
    index: usize
}

impl<'a> Lexer<'a> {
    pub fn new(text: &'a str) -> Lexer<'a> {
        let mut l = Lexer { tokens: Vec::new(), index: 0 };
        l.init(text);

        l
    }

    fn init(&mut self, text: &'a str) {
        let mut start_token = 0;
        let mut i = 0;

        while i < text.len() {
            if let Some(c) = text.chars().nth(i) {
                if c == ' ' || c == '[' || c == ']' || c == '"' || c == '#' || c == ':' {
                    if start_token+1 < i {
                        self.tokens.push(&text[start_token+1..i]);
                    }

                    self.tokens.push(&text[i..i+1]);
                    start_token = i;
                }
            }

            i += 1;
        }
    }

    pub fn count(&self) -> usize {
        self.tokens.len()
    }

    pub fn peek(&self) -> Option<&str> {
        self.peek_at(0)
    }

    pub fn peek_at(&self, index: usize) -> Option<&str> {
        Self::internal_peek_at(&self.tokens, self.index + index)
    }

    pub fn consume(&mut self) -> Option<&str> {
        let s = Self::internal_peek_at(&self.tokens, self.index);
        Self::inc_index(&mut self.index);

        s
    }

    pub fn consume_if(&mut self, val_is: &str) {
        match self.peek() {
            Some(p) => {
                if p == val_is {
                    self.consume();
                }
            },
            None => {}
        }
    }


    pub fn consume_and_check(&mut self, check: &str) -> Result<(), String> {
        let val = self.peek();

        let mut consume = false;
        if let Some(v) = val {
            consume = v == check;
        }

        if consume {
            self.consume();
        }
        else {
            let mut msg = String::from("");
            msg.push_str("Expected [");
            msg.push_str(check);
            msg.push_str("]");
            msg.push_str(", but received [");
            msg.push_str(val.unwrap_or(""));
            msg.push_str("]");
            return Err(msg);
        }

        Ok(())
    }

    fn internal_peek_at(tokens: &Vec<&'a str>, index: usize) -> Option<&'a str> {
        if index >= tokens.len() {
            return None;
        }

        Some(tokens[index])
    }

    fn inc_index(index: &mut usize) {
        *index += 1;
    }
}

impl<'a> std::fmt::Debug for Lexer<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        println!("Debug: token");
        for token in &self.tokens {
            println!("{:?}", token);
        }

        Ok(())
    }
}

#[test]
fn test_peek_peek_at() {
    let s = "#v1.0[_id:\"1a4b2b21-c0ec-4857-8f06-454068c4cc6c\" col1:\"123\" col2:\"two  spaces#\" col3:\"789\"]";
    let lexer = Lexer::new(s);

    assert_eq!(lexer.peek().unwrap(), "#");
    assert_eq!(lexer.peek_at(1).unwrap(), "v1.0");
    assert_eq!(lexer.peek_at(2).unwrap(), "[");
    assert_eq!(lexer.peek_at(3).unwrap(), "_id");
    assert_eq!(lexer.peek_at(5).unwrap(), "\"");
}

#[test]
fn test_consume() {
    let s = "#v1.0[_id:\"1a4b2b21-c0ec-4857-8f06-454068c4cc6c\" col1:\"123\" col2:\"two  spaces#\" col3:\"789\"]";
    let mut lexer = Lexer::new(s);

    assert_eq!(lexer.consume().unwrap(), "#");
    assert_eq!(lexer.peek().unwrap(), "v1.0");
    assert_eq!(lexer.peek_at(1).unwrap(), "[");
    assert_eq!(lexer.consume().unwrap(), "v1.0");
    assert_eq!(lexer.peek().unwrap(), "[");
}

#[test]
fn test_consume_all() {
    let s = "#v1.0[_id:\"test\"]";
    let mut lexer = Lexer::new(s);

    let mut num_of_consume = 0;

    let mut last_peek = String::from("");
    let mut peek = lexer.peek();
    while peek.is_some() {
        last_peek = String::from(peek.unwrap());

        peek = lexer.consume();
        num_of_consume += 1;
    }

    assert_eq!(lexer.count(), num_of_consume - 1);
    assert_eq!(lexer.peek(), None);
    assert_eq!(last_peek, "]");
}
