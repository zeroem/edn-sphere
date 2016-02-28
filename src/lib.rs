use std::str::Chars;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Token {
    Nil,
}

pub struct Parser<'a> {
    source: &'a String,
    iterator: Chars<'a>,
    character: i64,
    line: i64,
}

pub enum ElementParserResult {
    Continue,
    None,
    Match(Token),
}

// impl ElementParser {
//     fn matches(&self, c: &char) -> ElementParserResult
// }

impl<'a> Parser<'a> {
    fn new(source: &'a String) -> Parser<'a> {
        Parser { 
            source: source,
            iterator: source.chars(),
            character: 0,
            line: 1
        }
    }

    fn next_character(&mut self) -> Option<char> {
        let ch_opt = self.iterator.next();

        match ch_opt {
            Some(_) => {
                self.character+= 1;
            },
            _ => {}
        }

        ch_opt
    }

    fn parse_nil(&mut self) -> Option<Token> {
        if "nil".chars().all(|c| Some(c) == self.next_character()) {
            Some(Token::Nil)
        } else {
            None
        }
    }

    fn parse_whitespace(&mut self) -> Option<Token> {
        
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initialization_test() {
        let source = String::from("");
        let p = Parser::new(&source);
        assert_eq!(source, *p.source);
        assert_eq!(0, p.character);
        assert_eq!(1, p.line);
    }

    #[test]
    fn next_character_test() {
        let source = String::from("str");
        let mut p = Parser::new(&source);

        let ch_opt = p.next_character();
        assert_eq!('s', ch_opt.unwrap());
        assert_eq!(1, p.character);

        let ch_opt = p.next_character();
        assert_eq!('t', ch_opt.unwrap());
        assert_eq!(2, p.character);

        let ch_opt = p.next_character();
        assert_eq!('r', ch_opt.unwrap());
        assert_eq!(3, p.character);

        let ch_opt = p.next_character();
        assert_eq!(None, ch_opt);
        assert_eq!(3, p.character);
    }
    
    #[test]
    fn parse_nil_test() {
        let source = String::from("nil");
        let mut p = Parser::new(&source);

        assert_eq!(Some(Token::Nil), p.parse_nil());

        let source = String::from("not-nil");
        let mut p = Parser::new(&source);

        assert_eq!(None, p.parse_nil());
    }
}
