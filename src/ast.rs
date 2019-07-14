#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Type {
    Bang,                   // !   X
    Begin,     // reserved
    Call,      // reserved
    Comma,                  // ,   X
    Const,     // reserved
    Do,        // reserved
    Dot,                    // .   X
    End,       // reserved
    Equal,                  // =   X
    Greater,                // >   X
    GreaterEqual,           // >=  X
    Hash,                   // #   X
    Identifier,
    If,        // reserved
    LeftParen,              // (   X
    Less,                   // <   X
    LessEqual,              // <=  X
    Minus,                  // -   X
    Number,
    Odd,       // reserved
    Plus,                   // +   X
    Procedure, // reserved
    Question,               // ?   X
    RightParen,             // )   X
    ColonEqual,             // :=  X
    Semicolon,              // ;   X
    Slash,                  // /
    Star,                   // *
    Then,      // reserved
    Var,       // reserved
    While,     // reserved

    EOF,
}

#[derive(Debug)]
pub enum Literal {
    Number(i32)
}

#[derive(Debug)]
pub struct Token {
    pub r#type: Type,
    pub lexeme: String,
    pub literal: Option<Literal>,
    pub line: u32
}

impl Token {
    pub fn new(r#type: Type,
           lexeme: String,
           literal: Option<Literal>,
           line: u32) -> Token {
        Token {
            r#type, lexeme, literal, line
        }
    }
}
