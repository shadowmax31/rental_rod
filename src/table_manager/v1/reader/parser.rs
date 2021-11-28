pub struct Parser<'a> {
    text: &'a String
}

impl<'a> Parser<'a> {
    pub fn new(text: &'a String) -> Parser<'a> {
        Parser { text }
    }

    pub fn parse(&self) -> () {
        //
    }
}
