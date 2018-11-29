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

    fn match_token(&mut self, expected_token: Token) -> Result<(), &'static str> {
        println!("{:?}", self.input_token);
        if self.input_token != expected_token {
            println!("expected {:?} got {:?}", expected_token, self.input_token);
            Err("couldn't match token")
        } else {
            self.input_token = self.tokenizer.next();
            Ok(())
        }
    }

    pub fn parse(&mut self) {
        self.input_token = self.tokenizer.next();
        match self.program() {
            Err(e) => println!("{:?}", e),
            Ok(()) => {
                match self.match_token(Token::EndOfFile) {
                    Err(e) => println!("{:?}", e),
                    _ => println!("{:?}", self.values),
                }
            }
        }
    }

    fn program(&mut self) -> Result<(), &'static str> {
        let _ = self.assignment()?;
        while self.input_token != Token::EndOfFile {
            let _ = self.assignment()?;
        }
        Ok(())
    }

    fn assignment(&mut self) -> Result<i32, &'static str> {
        match self.input_token {
            Token::Id(_) => {
                let id = self.read_id()?;
                self.consume();
                self.match_token(Token::Assign)?;
                let v = self.expression()?;
                self.match_token(Token::Semicolon)?;
                self.values.insert(id.to_string(), v);
                println!("{:?}", self.values);
                Ok(v)
            },
            _ => Err("Couldn't parse assignment"),
        }
    }

    fn expression(&mut self) -> Result<i32, &'static str> {
        match self.input_token {
            Token::Id(_) | Token::Literal(_) | Token::LeftParenthesis | Token::Plus | Token::Minus => {
                println!("{:?}", self.input_token);
                let t = self.term();
                let e_p = self.expression_prime()?;
                let x = e_p(t);
                println!("{:?}, {:?}, {:?}", self.input_token, x, t);
                x
            },
            _ => Err("Couldn't parse expression"),
        }
    }

    fn expression_prime(&mut self) -> Result<Box<Fn(Result<i32, &'static str>) -> Result<i32, &'static str>>, &'static str> {
        match self.input_token {
            Token::Plus => {
                self.match_token(Token::Plus)?;
                let t = self.term();
                let ep = self.expression_prime()?;
                Ok(Box::new(move |v| Ok(v? + ep(t)?)))
            },
            Token::Minus => {
                self.match_token(Token::Minus)?;
                let t = self.term();
                let ep = self.expression_prime()?;
                Ok(Box::new(move |v| Ok(v? - ep(t)?)))
            },
            Token::RightParenthesis | Token::EndOfFile | Token::Semicolon => Ok(Box::new(move |v| v)),
            _ => {
                println!("in expression prime, couldn't match, going to return 'empty' function {:?}", self.input_token);
                //Err("Couldn't parse expression prime")
                Ok(Box::new(move |v| v))
            },
        }
    }

    fn term(&mut self) -> Result<i32, &'static str> {
        match self.input_token {
            Token::Id(_) | Token::Literal(_) | Token::LeftParenthesis | Token::Plus | Token::Minus => {
                let f = self.factor();
                let tp  = self.term_prime()?;
                tp(f)
            },
            _ => {
                println!("in term, going to error {:?}", self.input_token);
                Err("Couldn't parse term")
            }
        }
    }

    fn term_prime(&mut self) -> Result<Box<Fn(Result<i32, &'static str>) -> Result<i32, &'static str>>, &'static str> {
        match self.input_token {
            Token::Multiply => {
                self.match_token(Token::Multiply)?;
                let f = self.factor();
                let tp = self.term_prime()?;
                Ok(Box::new(move |v| Ok(v? * tp(f)?)))
            },
            Token::RightParenthesis | Token::EndOfFile | Token::Semicolon => Ok(Box::new(move |v| v)),
            _ => {
                println!("in term prime, couldn't match, going to return 'empty' function {:?}", self.input_token);
                //Err("Couldn't parse term prime"),
                Ok(Box::new(move |v| v))
            }
        }
    }

    fn read_id(&mut self) -> Result<String, &'static str> {
        match self.input_token {
            Token::Id(ref id) => Ok(id.to_string()),
            _ => Err("Not an id"),
        }
    }

    fn consume(&mut self) {
        println!("{:?}", self.input_token);
        self.input_token = self.tokenizer.next();
    }

    fn factor(&mut self) -> Result<i32, &'static str> {
        match self.input_token {
            Token::Id(_) => {
                let id = self.read_id()?;
                self.consume();
                match self.values.get(&id) {
                    Some(v) => Ok(*v),
                    None => Err("uninitialized variable"),
                }
            }
            Token::Literal(lit) => {
                self.consume();
                Ok(lit)
            }
            Token::LeftParenthesis => {
                self.match_token(Token::LeftParenthesis)?;
                let val = self.expression();
                self.match_token(Token::RightParenthesis)?;
                val
            },
            Token::Plus => {
                self.match_token(Token::Plus)?;
                let val = self.factor()?;
                Ok(val)
            },
            Token::Minus => {
                self.match_token(Token::Minus)?;
                let val = self.factor()?;
                Ok((-1) * val)
            },
            _ => Err("Couldn't parse factor"),
        }
    }
}
