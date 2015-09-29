
use self::ParserState::*;
use self::EdnEvent::*;
use self::ParserError::*;
use self::ErrorCode::*;

use std::io;
use std::collections::{HashMap, HashSet, LinkedList};

#[derive(Clone, Eq, PartialEq, PartialOrd, Debug)]
pub enum Edn {
    Nil,
    Boolean(bool),
    String(String),
    Character(char),
    Symbol(self::Symbol),
    Keyword(self::Keyword),
    Integer(i64),
    Float(f64),
    List(self::List),
    Vector(self::Vector),
    Map(self::Map),
    Set(self::Set),
    Tag(self::Tag),
}

pub type Symbol = String;
pub type Keyword = String;
pub type List = LinkedList<Edn>;
pub type Vector = Vec<Edn>;
pub type Set = HashSet<Edn>;
pub type Map = HashMap<Edn, Edn>;
pub type Tag = String;

#[derive(PartialEq, Clone, Debug)]
pub enum EdnEvent {
    NilValue,
    BooleanValue(bool),
    StringValue(String),
    CharacterValue(char),
    SymbolValue(Symbol),
    KeywordValue(Keyword),
    IntegerValue(i64),
    FloatValue(f64),
    ListValue(List),
    VectorValue(Vector),
    MapValue(Map),
    SetValue(Set),
    TagValue(Tag),
    Error(ParserError),
}

#[derive(PartialEq, Debug)]
enum ParserState {
    // Parse a value in an array, true means first element.
    ParseArray(bool),
    // Parse ',' or ']' after an element in an array.
    ParseArrayComma,
    // Parse a key:value in an object, true means first element.
    ParseObject(bool),
    // Parse ',' or ']' after an element in an object.
    ParseObjectComma,
    // Initial state.
    ParseStart,
    // Expecting the stream to end.
    ParseBeforeFinish,
    // Parsing can't continue.
    ParseFinished,
}

/// The errors that can arise while parsing a JSON stream.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ErrorCode {
    InvalidSyntax,
    InvalidNumber,
    EOFWhileParsingObject,
    EOFWhileParsingArray,
    EOFWhileParsingValue,
    EOFWhileParsingString,
    KeyMustBeAString,
    ExpectedColon,
    TrailingCharacters,
    TrailingComma,
    InvalidEscape,
    InvalidUnicodeCodePoint,
    LoneLeadingSurrogateInHexEscape,
    UnexpectedEndOfHexEscape,
    UnrecognizedHex,
    NotFourDigit,
    NotUtf8,
}

#[derive(Clone, PartialEq, Debug)]
pub enum ParserError {
    /// msg, line, col
    SyntaxError(ErrorCode, usize, usize),
    IoError(io::ErrorKind, String),
}

pub struct Parser<T> {
    rdr: T,
    ch: Option<char>,
    line: usize,
    col: usize,
    // We maintain a stack representing where we are in the logical structure
    // of the JSON stream.
    // stack: Stack,
    // A state machine is kept to make it possible to interrupt and resume parsing.
    state: ParserState,
}

impl<T: Iterator<Item=char>> Iterator for Parser<T> {
    type Item = EdnEvent;

    fn next(&mut self) -> Option<EdnEvent> {
        if self.state == ParseFinished {
            return None;
        }

        if self.state == ParseBeforeFinish {
            self.parse_whitespace();
            // Make sure there is no trailing characters.
            if self.eof() {
                self.state = ParseFinished;
                return None;
            } else {
                return Some(self.error_event(TrailingCharacters));
            }
        }

        Some(self.parse())
    }
}


impl<T: Iterator<Item=char>> Parser<T> {
    /// Creates the Edn parser.
    pub fn new(rdr: T) -> Parser<T> {
        let mut p = Parser {
            rdr: rdr,
            ch: Some('\x00'),
            line: 1,
            col: 0,
            //stack: Stack::new(),
            state: ParseStart,
        };
        p.bump();
        p
    }

    pub fn parse(&mut self) -> EdnEvent {
        NilValue
    }

    /*
    fn parse_start(&mut self) -> EdnEvent {
        let val = self.parse_value();
        self.state = match val {
            Error(_) => ParseFinished,
            ArrayStart => ParseArray(true),
            ObjectStart => ParseObject(true),
            _ => ParseBeforeFinish,
        };
        val
    }
    */

    fn parse_value(&mut self) -> EdnEvent {
        if self.eof() { return self.error_event(EOFWhileParsingValue); }
        match self.ch_or_null() {
            'n' => { self.parse_ident("il", NilValue) }
            't' => { self.parse_ident("rue", BooleanValue(true)) }
            'f' => { self.parse_ident("alse", BooleanValue(false)) }
            /*
            '0' ... '9' | '-' => self.parse_number(),
            '"' => match self.parse_str() {
                Ok(s) => StringValue(s),
                Err(e) => Error(e),
            },
            '[' => {
                self.bump();
                ArrayStart
            }
            '{' => {
                self.bump();
                ObjectStart
            }
            */
            _ => { self.error_event(InvalidSyntax) }
        }
    }

    fn eof(&self) -> bool { self.ch.is_none() }
    fn ch_or_null(&self) -> char { self.ch.unwrap_or('\x00') }
    fn bump(&mut self) {
        self.ch = self.rdr.next();

        if self.ch_is('\n') {
            self.line += 1;
            self.col = 1;
        } else {
            self.col += 1;
        }
    }

    fn next_char(&mut self) -> Option<char> {
        self.bump();
        self.ch
    }
    fn ch_is(&self, c: char) -> bool {
        self.ch == Some(c)
    }

    fn parse_ident(&mut self, ident: &str, value: EdnEvent) -> EdnEvent {
        if ident.chars().all(|c| Some(c) == self.next_char()) {
            self.bump();
            value
        } else {
            Error(SyntaxError(InvalidSyntax, self.line, self.col))
        }
    }

    fn parse_whitespace(&mut self) {
        while self.ch_is(' ') ||
              self.ch_is(',') ||
              self.ch_is('\n') ||
              self.ch_is('\t') ||
              self.ch_is('\r') { self.bump(); }
    }

    fn error_event(&mut self, reason: ErrorCode) -> EdnEvent {
        self.state = ParseFinished;
        Error(SyntaxError(reason, self.line, self.col))
    }
}
