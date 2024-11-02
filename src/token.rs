
// type TokenType<'a> = &'a str;
#[derive(PartialEq, Debug)]
pub struct Token {
    pub token_type: String,
    pub literal: String
}

impl Token {
    pub fn new(token_type: &str, literal: &str) -> Token {
        Token {
            token_type: String::from(token_type),
            literal: String::from(literal),
        }
    }
}

// mod TokenTypes {
pub const ILLEGAL: &str = "ILLEGAL";
pub const EOF: &str = "EOF";

// identifiers + literals
pub const IDENT: &str = "IDENT";
pub const INT: &str = "INT";

// operators
pub const PLUS: &str = "+";
pub const ASSIGN: &str = "=";

// delimiters
pub const COMMA: &str = ",";
pub const SEMICOLON: &str = ";";
pub const LPAREN: &str = "(";
pub const RPAREN: &str = ")";
pub const LBRACE: &str = "{";
pub const RBRACE: &str = "}";

// keywords
pub const FUNCTION: &str = "FUNCTION";
pub const LET: &str = "LET";
// }