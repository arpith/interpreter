use token::Token;
use tokenizer::Tokenizer;
use std::collections::HashMap;

pub struct Parser<'a> {
    values: HashMap<String, i32>,
    tokenizer: Tokenizer<'a>,
    input_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(input: &str) -> Parser {
        Parser{
            tokenizer: Tokenizer::new(input),
            input_token: Token::EndOfFile,
            values: HashMap::new(),
        }
    }

    fn fatal_error(&mut self, err: &str) {
        println!("{:}",err);
    }

    fn match_token(&mut self, expected_token: Token) {
        println!("{:?}", self.input_token);
        if self.input_token != expected_token {
            println!("help!");
        } else {
            self.input_token = self.tokenizer.next();
        }
    }

    pub fn parse(&mut self) {
        self.input_token = self.tokenizer.next();
        self.program();
        self.match_token(Token::EndOfFile);
    }

    fn program(&mut self) {
        while self.input_token != Token::EndOfFile {
            self.assignment();
        }
    }

    fn assignment(&mut self) -> Result<(), &'static str> {
        match self.input_token {
            Token::Id(ref id) => {
                self.consume();
                self.match_token(Token::Assign);
                let v = self.expression()?;
                self.match_token(Token::Semicolon);
                self.values.insert(id.to_string(), v);
                Ok(())
            },
            _ => Err("Couldn't parse assignment"),
        }
    }

    fn expression(&mut self) -> Result<i32, &'static str> {
        match self.input_token {
            Token::Id(_) | Token::Literal(_) | Token::LeftParenthesis | Token::Plus | Token::Minus => {
                let t = self.term();
                let e_p = self.expression_prime()?;
                e_p(t)
            },
            _ => Err("Couldn't parse expression"),
        }
    }

    fn expression_prime(&mut self) -> Result<Box<Fn(Result<i32, &'static str>) -> Result<i32, &'static str>>, &'static str> {
        match self.input_token {
            Token::Plus => {
                self.match_token(Token::Plus);
                let t = self.term();
                let ep = self.expression_prime()?;
                Ok(Box::new(move |v| Ok(v? + ep(t)?)))
            },
            Token::Minus => {
                self.match_token(Token::Minus);
                let t = self.term();
                let ep = self.expression_prime()?;
                Ok(Box::new(move |v| Ok(v? - ep(t)?)))
            },
            Token::RightParenthesis | Token::EndOfFile => Ok(Box::new(move |v| v)),
            _ => Err("Couldn't parse expression prime"),
        }
    }

    fn term(&mut self) -> Result<i32, &'static str> {
        match self.input_token {
            Token::Id(_) | Token::Literal(_) | Token::LeftParenthesis => {
                let f = self.factor();
                let tp  = self.term_prime()?;
                tp(f)
            },
            _ => Err("Couldn't parse term"),
        }
    }

    fn term_prime(&mut self) -> Result<Box<Fn(Result<i32, &'static str>) -> Result<i32, &'static str>>, &'static str> {
        match self.input_token {
            Token::Multiply => {
                self.match_token(Token::Multiply);
                let f = self.factor();
                let tp = self.term_prime()?;
                Ok(Box::new(move |v| Ok(v? * tp(f)?)))
            },
            Token::RightParenthesis | Token::EndOfFile => Ok(Box::new(move |v| v)),
            _ => Err("Couldn't parse term prime"),
        }
    }

    fn consume(&mut self) {
        println!("{:?}", self.input_token);
        self.input_token = self.tokenizer.next();
    }

    fn factor(&mut self) -> Result<i32, &'static str> {
        match self.input_token {
            Token::Id(ref mut id) => {
                self.consume();
                match self.values.get(id) {
                    Some(v) => Ok(*v),
                    None => Err("uninitialized variable"),
                }
            }
            Token::Literal(lit) => {
                self.consume();
                Ok(lit)
            }
            Token::LeftParenthesis => {
                self.match_token(Token::LeftParenthesis);
                let val = self.expression();
                self.match_token(Token::RightParenthesis);
                val
            },
            Token::Plus => {
                self.match_token(Token::Plus);
                let val = self.factor()?;
                Ok(val + 1)
            },
            Token::Minus => {
                self.match_token(Token::Minus);
                let val = self.factor()?;
                Ok(val - 1)
            },
            _ => Err("Couldn't parse factor"),
        }
    }
}
