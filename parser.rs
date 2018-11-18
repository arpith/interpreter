use token::Token;
use tokenizer::Tokenizer;

pub struct Parser<'a> {
    tokenizer: Tokenizer<'a>,
    input_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(input: &str) -> Parser {
        Parser{ tokenizer: Tokenizer::new(input), input_token: Token::EndOfFile }
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

    fn assignment(&mut self) {
        match self.input_token {
            Token::Id(_) => {
                self.consume();
                self.match_token(Token::Assign);
                self.expression();
                self.match_token(Token::Semicolon);
            },
            _ => println!("Couldn't parse assignment"),
        }
    }

    fn expression(&mut self) {
        match self.input_token {
            Token::Id(_) | Token::Literal(_) | Token::LeftParenthesis | Token::Plus | Token::Minus => {
                self.term();
                self.expression_prime();
            },
            _ => println!("Couldn't parse expression {:?}", self.input_token),
        }
    }

    fn expression_prime(&mut self) {
        match self.input_token {
            Token::Plus => {
                self.match_token(Token::Plus);
                self.term();
                self.expression_prime();
            },
            Token::Minus => {
                self.match_token(Token::Minus);
                self.term();
                self.expression_prime();
            },
            Token::RightParenthesis | Token::EndOfFile => {
                return;
            },
            _ => println!("Couldn't parse expression prime"),
        }
    }

    fn term(&mut self) {
        match self.input_token {
            Token::Id(_) | Token::Literal(_) | Token::LeftParenthesis => {
                self.factor();
                self.term_prime();
            },
            _ => println!("Couldn't parse term {:?}", self.input_token),
        }
    }

    fn term_prime(&mut self) {
        match self.input_token {
            Token::Multiply => {
                self.match_token(Token::Multiply);
                self.factor();
                self.term_prime();
            },
            Token::RightParenthesis | Token::EndOfFile => {
                return;
            },
            _ => println!("Couldn't parse term prime"),
        }
    }

    fn consume(&mut self) {
        println!("{:?}", self.input_token);
        self.input_token = self.tokenizer.next();
    }

    fn factor(&mut self) {
        match self.input_token {
            Token::Id(_) => self.consume(),
            Token::Literal(_) => self.consume(),
            Token::LeftParenthesis => {
                self.match_token(Token::LeftParenthesis);
                self.expression();
                self.match_token(Token::RightParenthesis);
            },
            Token::Plus => {
                self.match_token(Token::Plus);
                self.factor();
            },
            Token::Minus => {
                self.match_token(Token::Minus);
                self.factor();
            },
            _ => println!("Couldn't parse factor"),
        }
    }
}
