use crate::token::*;

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: Option<char>,
}

impl Lexer {
    pub fn new(inp: &str) -> Lexer {
        let mut lexer = Lexer {
            input: String::from(inp),
            position: 0,
            read_position: 0,
            ch: None,
        };
        lexer.read_char();
        lexer
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = Some('0');
        } else {
            self.ch = self.input.chars().nth(self.read_position as usize);
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        // let token: Token;
        println!(
            "[next_token()] self.ch {:?} position {}, read_pos: {}",
            self.ch, self.position, self.read_position
        );

        let token: Token = match self.ch {
            Some('=') => Token::new(ASSIGN, "="),
            Some(';') => Token::new(SEMICOLON, ";"),
            Some('+') => Token::new(PLUS, "+"),
            Some('(') => Token::new(LPAREN, "("),
            Some(')') => Token::new(RPAREN, ")"),
            Some('{') => Token::new(LBRACE, "{"),
            Some('}') => Token::new(RBRACE, "}"),
            Some(',') => Token::new(COMMA, ","),
            Some('0') => Token::new(EOF, ""),
            _ => panic!("We shouldn't reach here"),
        };
        self.read_char();
        token
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_token() {
        let input = "=+(){},;";
        let mut lexer = Lexer::new(input);

        let expected = vec![
            Token::new(ASSIGN, "="),
            Token::new(PLUS, "+"),
            Token::new(LPAREN, "("),
            Token::new(RPAREN, ")"),
            Token::new(LBRACE, "{"),
            Token::new(RBRACE, "}"),
            Token::new(COMMA, ","),
            Token::new(SEMICOLON, ";"),
        ];

        for exp in expected {
            let token = lexer.next_token();
            assert_eq!(token, exp);
        }
    }
}
