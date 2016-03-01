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

trait TokenParser {
    fn new() -> Self;
    fn matches(&mut self, c: &char) -> bool;
    fn get_token(&self) -> Option<Token>;
}

pub struct NilTokenParser<'a> {
    iter: Chars<'a>,
    last_state: Option<bool>,
}

impl<'a> TokenParser for NilTokenParser<'a> {
    fn new() -> NilTokenParser<'a> {
        NilTokenParser {
            iter: "nil".chars(),
            last_state: None,
        }
    }

    fn matches(&mut self, c: &char) -> bool {
        let mut local_state: bool = false;

        if let Some(ch) = self.iter.next() {
            if *c == ch {
                local_state = true;
            }
        }

        if let Some(internal_state) = self.last_state {
            self.last_state = Some(internal_state && local_state);
        } else {
            self.last_state = Some(local_state);
        }

        return self.last_state.unwrap();
    }

    fn get_token(&self) -> Option<Token> {
        if let Some(s) = self.last_state {
            if s {
                return Some(Token::Nil);
            }
        }

        return None;
    }
}

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
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::TokenParser;

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
    fn nil_token_parser_test() {
        let mut parser = NilTokenParser::new();

        // Matches up to 'nil'
        assert!(parser.matches(&'n'));
        assert!(parser.matches(&'i'));
        assert!(parser.matches(&'l'));

        // Failes to match beyond 'nil'
        assert!(!parser.matches(&'l'));
    }
}

