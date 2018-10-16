#[derive(Debug, PartialEq)]
pub enum Token {
    Illegal,
    EndOfFile,

    Identifier(String),
    Literal(i32),

    Assign,
    Plus,
    Minus,

    Semicolon,
    LeftParenthesis,
    RightParenthesis,
}
