use crate::token::*;

fn is_letter(ch: char) -> bool {
    (ch >= 'a' && ch <= 'z') || (ch == '_') || (ch >= 'A' && ch <= 'Z')
}
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

    fn read_char(&mut self) {
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
    fn test_is_letter() {
        assert_eq!(true, is_letter('a'));
        assert_eq!(true, is_letter('z'));
        assert_eq!(true, is_letter('x'));
        assert_eq!(false, is_letter('$'));

        assert_eq!(true, is_letter('_'));
        assert_eq!(true, is_letter('A'));
        assert_eq!(true, is_letter('Z'));
        assert_eq!(true, is_letter('X'));
    }

    fn test_next_token(lexer: &mut Lexer, expected: Vec<Token>) {
        for exp in expected {
            let token = lexer.next_token();
            assert_eq!(token, exp);
        }
    }

    #[test]
    fn test_next_token_1() {
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

        test_next_token(&mut lexer, expected);
    }

    // TODO - parametrize tests using test-case crate?
    // #[test]
    fn test_next_token_2() {
        let input = r#"let five = 5;
        let ten = 10;
        
        let add = fn(x, y) {
            x+y;
        };
        
        let result = add(five, ten);"#;
        println!("{}", input);

        let mut lexer = Lexer::new(input);

        let expected = vec![
            Token::new(LET, "let"),
            Token::new(IDENT, "five"),
            Token::new(ASSIGN, "="),
            Token::new(INT, "5"),
            Token::new(SEMICOLON, ";"),
            Token::new(LET, "let"),
            Token::new(IDENT, "ten"),
            Token::new(ASSIGN, "="),
            Token::new(INT, "10"),
            Token::new(SEMICOLON, ";"),
            Token::new(LET, "let"),
            Token::new(IDENT, "add"),
            Token::new(ASSIGN, "="),
            Token::new(FUNCTION, "fn"),
            Token::new(LPAREN, "("),
            Token::new(IDENT, "x"),
            Token::new(COMMA, ","),
            Token::new(IDENT, "y"),
            Token::new(RPAREN, ")"),
            Token::new(LBRACE, "{"),
            Token::new(IDENT, "x"),
            Token::new(PLUS, "+"),
            Token::new(IDENT, "y"),
            Token::new(SEMICOLON, ";"),
            Token::new(RBRACE, "}"),
            Token::new(SEMICOLON, ";"),
            Token::new(LET, "let"),
            Token::new(IDENT, "result"),
            Token::new(ASSIGN, "="),
            Token::new(IDENT, "add"),
            Token::new(LPAREN, "("),
            Token::new(IDENT, "five"),
            Token::new(COMMA, ","),
            Token::new(IDENT, "ten"),
            Token::new(RPAREN, ")"),
            Token::new(SEMICOLON, ";"),
        ];

        test_next_token(&mut lexer, expected);
    }
}
