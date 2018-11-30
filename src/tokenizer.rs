use token::Token;
use token::Token::*;

use std::iter::Peekable;
use std::str::Chars;

pub struct Tokenizer<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &str) -> Tokenizer {
        Tokenizer {
            input: input.chars().peekable(),
        }
    }

    fn read_identifier(&mut self, first: char) -> Token {
        let mut ident = String::new();
        ident.push(first);

        while let Some(&c) = self.input.peek() {
            if !c.is_numeric() && !(c.is_alphabetic() || c == '_') {
                break;
            }
            ident.push(self.input.next().unwrap());
        }

        Id(ident)
    }

    fn read_literal(&mut self, first: char) -> Token {
        let mut literal = String::new();
        literal.push(first);

        while let Some(&c) = self.input.peek() {
            if !c.is_numeric() {
                break;
            }
            literal.push(self.input.next().unwrap());
        }

        if literal.len() > 1 && literal.starts_with('0') {
            return Illegal;
        }

        Literal(literal.parse().unwrap())
    }

    pub fn next(&mut self) -> Token {
        while self.input.peek() != None && self.input.peek().unwrap().is_whitespace() {
            self.input.next();
        }

        match self.input.next() {
            Some('=') => Assign,
            Some('+') => Plus,
            Some('-') => Minus,
            Some('*') => Multiply,
            Some(';') => Semicolon,
            Some('(') => LeftParenthesis,
            Some(')') => RightParenthesis,

            Some(ch @ _) => {
                if ch.is_alphabetic() || ch == '_' {
                    self.read_identifier(ch)
                } else if ch.is_numeric() {
                    self.read_literal(ch)
                } else {
                    Illegal
                }
            }

            None => EndOfFile,
        }
    }
}
