#[derive(Debug, PartialEq)]
pub enum Token {
    Illegal,
    EndOfFile,

    Id(String),
    Literal(i32),

    Assign,
    Plus,
    Minus,
    Multiply,


    Semicolon,
    LeftParenthesis,
    RightParenthesis,
}
