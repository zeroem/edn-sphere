use std::str::Chars;

#[derive(Debug,Clone,PartialEq)]
pub enum Token {
    Nil,
    Boolean(bool),
    Whitespace(Vec<char>),
}

pub struct Parser<'a> {
    source: &'a String,
    iterator: Chars<'a>,
    current_character: Option<char>,
    character: i64,
    line: i64,
}

trait TokenParser {
    fn matches(&mut self, c: &char) -> bool;
    fn get_token(&self) -> Option<Token>;
}

pub struct KeywordTokenParser<'a> {
    iter: Chars<'a>,
    result: Token,
    last_state: Option<bool>,
}


impl<'a> KeywordTokenParser<'a> {
    fn new(keyword: &'a str, result: Token) -> KeywordTokenParser<'a> {
        KeywordTokenParser {
            iter: keyword.chars(),
            result: result,
            last_state: None,
        }
    }
}

impl<'a> TokenParser for KeywordTokenParser<'a> {
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
                return Some(self.result.clone());
            }
        }

        return None;
    }
}

impl<'a> Parser<'a> {
    fn is_whitespace(ch: &char) -> bool {
        ch.is_whitespace() || (*ch == ',')
    }

    fn new(source: &'a String) -> Parser<'a> {
        Parser { 
            source: source,
            iterator: source.chars(),
            current_character: None,
            character: 0,
            line: 1
        }
    }

    fn next_character(&mut self) -> Option<char> {
        let ch_opt = self.iterator.next();

        match ch_opt {
            Some(c) => {
                self.character += 1;
            },
            _ => {}
        }

        self.current_character = ch_opt;
        self.current_character
    }

    fn parse_whitespace(&mut self) -> Option<Token> {
        let mut ws: Vec<char> = vec!();

        while let Some(c) = self.current_character {
            if Parser::is_whitespace(&c) {
                if c == '\n' {
                    self.line += 1;
                    self.character = 0;
                }
                ws.push(c);
                self.next_character();
            } else {
                break;
            }
        }

        if !ws.is_empty() {
            Some(Token::Whitespace(ws))
        } else {
            None
        }
    }

    pub fn parse_value(&mut self) -> Option<Token> {
        let nil_parser   = &mut Box::new(KeywordTokenParser::new("nil", Token::Nil));
        let true_parser  = &mut Box::new(KeywordTokenParser::new("true", Token::Boolean(true)));
        let false_parser = &mut Box::new(KeywordTokenParser::new("false", Token::Boolean(false)));

        let value_parsers = &mut vec!(
            nil_parser,
            true_parser,
            false_parser,
            );

        while let Some(ch) = self.next_character() {
            if ! Parser::is_whitespace(&ch) {
                for p in value_parsers.iter_mut() {
                    p.matches(&ch);
                }
            } else {
                break;
            }
        }

        let ws = self.parse_whitespace();
        let tokens = value_parsers.iter().map(|p| p.get_token());

        for t in tokens {
            if let Some(_) = t {
                return t;
            }
        }

        return None;
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
        let mut parser = KeywordTokenParser::new("nil", Token::Nil);

        // Matches up to 'nil'
        assert!(parser.matches(&'n'));
        assert!(parser.matches(&'i'));
        assert!(parser.matches(&'l'));

        assert_eq!(Some(Token::Nil), parser.get_token());

        // Failes to match beyond 'nil'
        assert!(!parser.matches(&'l'));
        assert_eq!(None, parser.get_token());
    }

    #[test]
    fn whitespace_test() {
        assert!(Parser::is_whitespace(&' '));
        assert!(Parser::is_whitespace(&'\t'));
        assert!(Parser::is_whitespace(&','));
        assert!(!Parser::is_whitespace(&'f'));
    }

    #[test]
    fn parse_whitespace_test() {
        let s = " ";
        let string = &String::from(s);
        let mut p = Parser::new(string);
        p.next_character();
        assert_eq!(Some(Token::Whitespace(s.chars().collect())), p.parse_whitespace());

        let s = " \n ";
        let string = &String::from(s);
        let mut p = Parser::new(string);
        p.next_character();
        assert_eq!(Some(Token::Whitespace(s.chars().collect())), p.parse_whitespace());
        assert_eq!(2, p.line);
        assert_eq!(1, p.character);
    }

    #[test]
    fn value_parser_test() {
        assert_eq!(Some(Token::Nil), Parser::new(&String::from("nil")).parse_value());
        assert_eq!(Some(Token::Boolean(true)), Parser::new(&String::from("true")).parse_value());
        assert_eq!(Some(Token::Boolean(false)), Parser::new(&String::from("false")).parse_value());
        assert_eq!(None, Parser::new(&String::from("alskdjflsajkfsldf")).parse_value());
    }
}

