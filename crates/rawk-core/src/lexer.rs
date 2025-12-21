use crate::token::{Token, TokenKind};

pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
    read_position: usize,
    ch: Option<u8>,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        let mut lexer = Lexer {
            input: src,
            position: 0,
            read_position: 0,
            ch: None,
        };

        lexer.read_char();
        lexer
    }

    pub fn next_token(&mut self) -> Token {
        let token = match self.ch {
            Some(b'{') => Token {
                kind: TokenKind::LeftCurlyBrace,
                literal: "{",
            },
            Some(b'}') => Token {
                kind: TokenKind::RightCurlyBrace,
                literal: "}",
            },
            None => Token {
                kind: TokenKind::Eof,
                literal: "",
            },
            _ => Token {
                kind: TokenKind::Illegal,
                literal: "",
            },
        };

        self.read_char();
        token
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = None;
        } else {
            self.ch = Some(self.input.as_bytes()[self.read_position]);
        }
        self.position = self.read_position;
        self.read_position += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_lbrace_token() {
        let expected = Token {
            kind: TokenKind::LeftCurlyBrace,
            literal: "{",
        };
        let input = "{";
        let mut lexer = Lexer::new(input);

        let token = lexer.next_token();

        assert_eq!(expected, token);
    }

    #[test]
    fn next_rbrace_token() {
        let expected = Token {
            kind: TokenKind::RightCurlyBrace,
            literal: "}",
        };
        let input = "}";
        let mut lexer = Lexer::new(input);

        let token = lexer.next_token();

        assert_eq!(expected, token);
    }

    #[test]
    fn next_token() {
        let input = "{}";
        let mut lexer = Lexer::new(input);

        let expected_tokens = vec![
            Token {
                kind: TokenKind::LeftCurlyBrace,
                literal: "{",
            },
            Token {
                kind: TokenKind::RightCurlyBrace,
                literal: "}",
            },
            Token {
                kind: TokenKind::Eof,
                literal: "",
            },
        ];

        for expected in expected_tokens {
            let token = lexer.next_token();
            assert_eq!(expected, token);
        }
    }
}
