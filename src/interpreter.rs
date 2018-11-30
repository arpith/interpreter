use token::Token;
use token::Token::*;
use tokenizer::Tokenizer;
use std::collections::HashMap;
use std;

type Result<T> = std::result::Result<T, &'static str>;
type Values = HashMap<String, i32>;
type Lambda<T> = Box<Fn(Result<T>) -> Result<T>>;

pub struct Interpreter<'a> {
    values: HashMap<String, i32>,
    tokenizer: Tokenizer<'a>,
    input_token: Token,
}

impl<'a> Interpreter<'a> {
    pub fn new(input: &str) -> Interpreter {
        Interpreter{
            tokenizer: Tokenizer::new(input),
            input_token: Token::EndOfFile,
            values: HashMap::new(),
        }
    }

    pub fn run(&mut self) -> Result<Values> {
        self.input_token = self.tokenizer.next();
        self.program()?;
        self.match_token(EndOfFile)?;
        Ok(self.values.clone())
    }

    fn match_token(&mut self, expected_token: Token) -> Result<()> {
        if self.input_token != expected_token {
            println!("expected {:?} got {:?}", expected_token, self.input_token);
            Err("couldn't match token")
        } else {
            self.input_token = self.tokenizer.next();
            Ok(())
        }
    }

    fn program(&mut self) -> Result<()> {
        let _ = self.assignment()?;
        while self.input_token != EndOfFile {
            let _ = self.assignment()?;
        }
        Ok(())
    }

    fn assignment(&mut self) -> Result<i32> {
        match self.input_token {
            Id(_) => {
                let id = self.read_id()?;
                self.consume();
                self.match_token(Assign)?;
                let v = self.expression()?;
                self.match_token(Semicolon)?;
                self.values.insert(id.to_string(), v);
                Ok(v)
            },
            _ => Err("Couldn't parse assignment"),
        }
    }

    fn expression(&mut self) -> Result<i32> {
        match self.input_token {
            Id(_) | Literal(_) | LeftParenthesis | Plus | Minus => {
                let t = self.term();
                let e_p = self.expression_prime()?;
                e_p(t)
            },
            _ => Err("Couldn't parse expression"),
        }
    }

    fn expression_prime(&mut self) -> Result<Lambda<i32>> {
        match self.input_token {
            Plus => {
                self.match_token(Plus)?;
                let t = self.term();
                let ep = self.expression_prime()?;
                Ok(Box::new(move |v| Ok(v? + ep(t)?)))
            },
            Minus => {
                self.match_token(Minus)?;
                let t = self.term();
                let ep = self.expression_prime()?;
                Ok(Box::new(move |v| Ok(v? - ep(t)?)))
            },
            RightParenthesis | EndOfFile | Semicolon => Ok(Box::new(move |v| v)),
            _ => Ok(Box::new(move |v| v)),
        }
    }

    fn term(&mut self) -> Result<i32> {
        match self.input_token {
            Id(_) | Literal(_) | LeftParenthesis | Plus | Minus => {
                let f = self.factor();
                let tp  = self.term_prime()?;
                tp(f)
            },
            _ => Err("Couldn't parse term"),
        }
    }

    fn term_prime(&mut self) -> Result<Lambda<i32>> {
        match self.input_token {
            Multiply => {
                self.match_token(Multiply)?;
                let f = self.factor();
                let tp = self.term_prime()?;
                Ok(Box::new(move |v| Ok(v? * tp(f)?)))
            },
            RightParenthesis | EndOfFile | Semicolon => Ok(Box::new(move |v| v)),
            _ => Ok(Box::new(move |v| v)),
        }
    }

    fn read_id(&mut self) -> Result<String> {
        match self.input_token {
            Id(ref id) => Ok(id.to_string()),
            _ => Err("Not an id"),
        }
    }

    fn consume(&mut self) {
        self.input_token = self.tokenizer.next();
    }

    fn factor(&mut self) -> Result<i32> {
        match self.input_token {
            Id(_) => {
                let id = self.read_id()?;
                self.consume();
                match self.values.get(&id) {
                    Some(v) => Ok(*v),
                    None => Err("uninitialized variable"),
                }
            }
            Literal(lit) => {
                self.consume();
                Ok(lit)
            }
            LeftParenthesis => {
                self.match_token(LeftParenthesis)?;
                let val = self.expression();
                self.match_token(RightParenthesis)?;
                val
            },
            Plus => {
                self.match_token(Plus)?;
                let val = self.factor()?;
                Ok(val)
            },
            Minus => {
                self.match_token(Minus)?;
                let val = self.factor()?;
                Ok((-1) * val)
            },
            _ => Err("Couldn't parse factor"),
        }
    }
}
