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
            Some(b'(') => Token {
                kind: TokenKind::LeftParen,
                literal: "(",
            },
            Some(b')') => Token {
                kind: TokenKind::RightParen,
                literal: ")",
            },
            Some(b'[') => Token {
                kind: TokenKind::LeftSquareBracket,
                literal: "[",
            },
            Some(b']') => Token {
                kind: TokenKind::RightSquareBracket,
                literal: "]",
            },
            Some(b',') => Token {
                kind: TokenKind::Comma,
                literal: ",",
            },
            Some(b';') => Token {
                kind: TokenKind::Semicolon,
                literal: ";",
            },
            Some(b'\n') => Token {
                kind: TokenKind::NewLine,
                literal: "<newline>",
            },
            Some(b'+') => Token {
                kind: TokenKind::Plus,
                literal: "+",
            },
            Some(b'-') => Token {
                kind: TokenKind::Minus,
                literal: "-",
            },
            Some(b'*') => Token {
                kind: TokenKind::Asterisk,
                literal: "*",
            },
            Some(b'%') => Token {
                kind: TokenKind::Percent,
                literal: "%",
            },
            Some(b'^') => Token {
                kind: TokenKind::Caret,
                literal: "^",
            },
            Some(b'!') => Token {
                kind: TokenKind::ExclamationMark,
                literal: "!",
            },
            Some(b'>') => Token {
                kind: TokenKind::GreaterThan,
                literal: ">",
            },
            Some(b'<') => Token {
                kind: TokenKind::LessThan,
                literal: "<",
            },
            Some(b'|') => Token {
                kind: TokenKind::Pipe,
                literal: "|",
            },
            Some(b'?') => Token {
                kind: TokenKind::QuestionMark,
                literal: "?",
            },
            Some(b':') => Token {
                kind: TokenKind::Colon,
                literal: ":",
            },
            Some(b'~') => Token {
                kind: TokenKind::Tilde,
                literal: "~",
            },
            Some(b'$') => Token {
                kind: TokenKind::DollarSign,
                literal: "$",
            },
            Some(b'=') => Token {
                kind: TokenKind::Equal,
                literal: "=",
            },
            None => Token {
                kind: TokenKind::Eof,
                literal: "",
            },
            _ => Token {
                kind: TokenKind::Illegal,
                literal: "<illegal>",
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
    fn next_left_curly_brace_token() {
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
    fn next_right_curly_brace_token() {
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
    fn next_one_character_token() {
        let input = "{}()[],;\n+-*%^!><|?:~$=";
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
                kind: TokenKind::LeftParen,
                literal: "(",
            },
            Token {
                kind: TokenKind::RightParen,
                literal: ")",
            },
            Token {
                kind: TokenKind::LeftSquareBracket,
                literal: "[",
            },
            Token {
                kind: TokenKind::RightSquareBracket,
                literal: "]",
            },
            Token {
                kind: TokenKind::Comma,
                literal: ",",
            },
            Token {
                kind: TokenKind::Semicolon,
                literal: ";",
            },
            Token {
                kind: TokenKind::NewLine,
                literal: "<newline>",
            },
            Token {
                kind: TokenKind::Plus,
                literal: "+",
            },
            Token {
                kind: TokenKind::Minus,
                literal: "-",
            },
            Token {
                kind: TokenKind::Asterisk,
                literal: "*",
            },
            Token {
                kind: TokenKind::Percent,
                literal: "%",
            },
            Token {
                kind: TokenKind::Caret,
                literal: "^",
            },
            Token {
                kind: TokenKind::ExclamationMark,
                literal: "!",
            },
            Token {
                kind: TokenKind::GreaterThan,
                literal: ">",
            },
            Token {
                kind: TokenKind::LessThan,
                literal: "<",
            },
            Token {
                kind: TokenKind::Pipe,
                literal: "|",
            },
            Token {
                kind: TokenKind::QuestionMark,
                literal: "?",
            },
            Token {
                kind: TokenKind::Colon,
                literal: ":",
            },
            Token {
                kind: TokenKind::Tilde,
                literal: "~",
            },
            Token {
                kind: TokenKind::DollarSign,
                literal: "$",
            },
            Token {
                kind: TokenKind::Equal,
                literal: "=",
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
