use crate::token::{Token, TokenKind, lookup_keyword};

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

    pub fn next_token(&mut self) -> Token<'a> {
        self.skip_whitespace();

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
            Some(b'+') => {
                if self.peek_char() == Some(b'=') {
                    self.read_char();
                    Token {
                        kind: TokenKind::AddAssign,
                        literal: "+=",
                    }
                } else if self.peek_char() == Some(b'+') {
                    self.read_char();
                    Token {
                        kind: TokenKind::Increment,
                        literal: "++",
                    }
                } else {
                    Token {
                        kind: TokenKind::Plus,
                        literal: "+",
                    }
                }
            }
            Some(b'-') => {
                if self.peek_char() == Some(b'=') {
                    self.read_char();
                    Token {
                        kind: TokenKind::SubtractAssign,
                        literal: "-=",
                    }
                } else if self.peek_char() == Some(b'-') {
                    self.read_char();
                    Token {
                        kind: TokenKind::Decrement,
                        literal: "--",
                    }
                } else {
                    Token {
                        kind: TokenKind::Minus,
                        literal: "-",
                    }
                }
            }
            Some(b'*') => {
                if self.peek_char() == Some(b'=') {
                    self.read_char();
                    Token {
                        kind: TokenKind::MultiplyAssign,
                        literal: "*=",
                    }
                } else {
                    Token {
                        kind: TokenKind::Asterisk,
                        literal: "*",
                    }
                }
            }
            Some(b'%') => {
                if self.peek_char() == Some(b'=') {
                    self.read_char();
                    Token {
                        kind: TokenKind::ModuloAssign,
                        literal: "%=",
                    }
                } else {
                    Token {
                        kind: TokenKind::Percent,
                        literal: "%",
                    }
                }
            }
            Some(b'^') => {
                if self.peek_char() == Some(b'=') {
                    self.read_char();
                    Token {
                        kind: TokenKind::PowerAssign,
                        literal: "^=",
                    }
                } else {
                    Token {
                        kind: TokenKind::Caret,
                        literal: "^",
                    }
                }
            }
            Some(b'!') => {
                if self.peek_char() == Some(b'=') {
                    self.read_char();
                    Token {
                        kind: TokenKind::NotEqual,
                        literal: "!=",
                    }
                } else if self.peek_char() == Some(b'~') {
                    self.read_char();
                    Token {
                        kind: TokenKind::NoMatch,
                        literal: "!~",
                    }
                } else {
                    Token {
                        kind: TokenKind::ExclamationMark,
                        literal: "!",
                    }
                }
            }
            Some(b'>') => {
                if self.peek_char() == Some(b'=') {
                    self.read_char();
                    Token {
                        kind: TokenKind::GreaterThanOrEqual,
                        literal: ">=",
                    }
                } else if self.peek_char() == Some(b'>') {
                    self.read_char();
                    Token {
                        kind: TokenKind::Append,
                        literal: ">>",
                    }
                } else {
                    Token {
                        kind: TokenKind::GreaterThan,
                        literal: ">",
                    }
                }
            }
            Some(b'<') => {
                if self.peek_char() == Some(b'=') {
                    self.read_char();
                    Token {
                        kind: TokenKind::LessThanOrEqual,
                        literal: "<=",
                    }
                } else {
                    Token {
                        kind: TokenKind::LessThan,
                        literal: "<",
                    }
                }
            }
            Some(b'|') => {
                if self.peek_char() == Some(b'|') {
                    self.read_char();
                    Token {
                        kind: TokenKind::Or,
                        literal: "||",
                    }
                } else {
                    Token {
                        kind: TokenKind::Pipe,
                        literal: "|",
                    }
                }
            }
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
            Some(b'=') => {
                if self.peek_char() == Some(b'=') {
                    self.read_char();
                    Token {
                        kind: TokenKind::Equal,
                        literal: "==",
                    }
                } else {
                    Token {
                        kind: TokenKind::Assign,
                        literal: "=",
                    }
                }
            }
            Some(b'/') => {
                if self.peek_char() == Some(b'=') {
                    self.read_char();
                    Token {
                        kind: TokenKind::DivideAssign,
                        literal: "/=",
                    }
                } else {
                    Token {
                        kind: TokenKind::Division,
                        literal: "/",
                    }
                }
            }
            Some(b'&') => {
                if self.peek_char() == Some(b'&') {
                    self.read_char();
                    Token {
                        kind: TokenKind::And,
                        literal: "&&",
                    }
                } else {
                    Token {
                        kind: TokenKind::Illegal,
                        literal: "<illegal>",
                    }
                }
            }
            ch if is_ascii_alphabetic(ch) => self.read_identifier(),
            ch if is_digit(ch) => self.read_number(),
            Some(b'.')
                if self
                    .peek_char()
                    .map_or(false, |arg0: u8| is_digit(Some(arg0))) =>
            {
                self.read_number()
            }
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

    fn read_identifier(&mut self) -> Token<'a> {
        let position = self.position;
        while is_ascii_alphabetic(self.ch) {
            self.read_char();
        }
        let literal = &self.input[position..self.position];

        return lookup_keyword(literal);
    }

    fn read_number(&mut self) -> Token<'a> {
        let position = self.position;
        let mut got_digit = false;

        // consume leading digits
        if self.ch != Some(b'.') {
            got_digit = true;
            while is_digit(self.ch) {
                self.read_char();
            }
            if self.ch == Some(b'.') {
                self.read_char();
            }
        } else {
            // consume the dot.
            self.read_char();
        }

        // consume trailing digits
        while is_digit(self.ch) {
            got_digit = true;
            self.read_char();
        }

        if !got_digit {
            return Token {
                kind: TokenKind::Illegal,
                literal: "<illegal>",
            };
        }

        let literal = &self.input[position..self.position];

        Token {
            kind: TokenKind::Number,
            literal: literal,
        }
    }

    fn skip_whitespace(&mut self) {
        while is_whitespace(self.ch) {
            self.read_char();
        }
    }

    fn peek_char(&self) -> Option<u8> {
        if self.read_position >= self.input.len() {
            None
        } else {
            Some(self.input.as_bytes()[self.read_position])
        }
    }
}

fn is_ascii_alphabetic(ch: Option<u8>) -> bool {
    match ch {
        Some(byte) => (byte >= b'a' && byte <= b'z') || (byte >= b'A' && byte <= b'Z'),
        None => false,
    }
}

fn is_whitespace(ch: Option<u8>) -> bool {
    match ch {
        Some(byte) => byte == b' ',
        None => false,
    }
}

fn is_digit(ch: Option<u8>) -> bool {
    match ch {
        Some(byte) => byte >= b'0' && byte <= b'9',
        None => false,
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
    fn next_pipe_token() {
        let expected = Token {
            kind: TokenKind::Pipe,
            literal: "|",
        };
        let input = "|";
        let mut lexer = Lexer::new(input);

        let token = lexer.next_token();

        assert_eq!(expected, token);
    }

    #[test]
    fn next_one_character_token() {
        let input = "{}()[],;\n+-*/%^!><|?:~$=";
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
                kind: TokenKind::Division,
                literal: "/",
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
                kind: TokenKind::Assign,
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

    #[test]
    fn next_while_token() {
        let expected = Token {
            kind: TokenKind::While,
            literal: "while",
        };
        let input = "while";
        let mut lexer = Lexer::new(input);

        let token = lexer.next_token();

        assert_eq!(expected, token);
    }

    #[test]
    fn next_identifier_token() {
        let input = "BEGIN END break continue delete do else exit for function if in next print printf return while";
        let mut lexer = Lexer::new(input);

        let expected_tokens = vec![
            Token {
                kind: TokenKind::Begin,
                literal: "BEGIN",
            },
            Token {
                kind: TokenKind::End,
                literal: "END",
            },
            Token {
                kind: TokenKind::Break,
                literal: "break",
            },
            Token {
                kind: TokenKind::Continue,
                literal: "continue",
            },
            Token {
                kind: TokenKind::Delete,
                literal: "delete",
            },
            Token {
                kind: TokenKind::Do,
                literal: "do",
            },
            Token {
                kind: TokenKind::Else,
                literal: "else",
            },
            Token {
                kind: TokenKind::Exit,
                literal: "exit",
            },
            Token {
                kind: TokenKind::For,
                literal: "for",
            },
            Token {
                kind: TokenKind::Function,
                literal: "function",
            },
            Token {
                kind: TokenKind::If,
                literal: "if",
            },
            Token {
                kind: TokenKind::In,
                literal: "in",
            },
            Token {
                kind: TokenKind::Next,
                literal: "next",
            },
            Token {
                kind: TokenKind::Print,
                literal: "print",
            },
            Token {
                kind: TokenKind::Printf,
                literal: "printf",
            },
            Token {
                kind: TokenKind::Return,
                literal: "return",
            },
            Token {
                kind: TokenKind::While,
                literal: "while",
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

    #[test]
    fn next_number_token() {
        let input = "123 4567 890 42.0 .75 0.001";
        let mut lexer = Lexer::new(input);

        let expected_tokens = vec![
            Token {
                kind: TokenKind::Number,
                literal: "123",
            },
            Token {
                kind: TokenKind::Number,
                literal: "4567",
            },
            Token {
                kind: TokenKind::Number,
                literal: "890",
            },
            Token {
                kind: TokenKind::Number,
                literal: "42.0",
            },
            Token {
                kind: TokenKind::Number,
                literal: ".75",
            },
            Token {
                kind: TokenKind::Number,
                literal: "0.001",
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

    #[test]
    fn next_or_token() {
        let expected = Token {
            kind: TokenKind::Or,
            literal: "||",
        };
        let input = "||";
        let mut lexer = Lexer::new(input);

        let token = lexer.next_token();

        assert_eq!(expected, token);
    }

    #[test]
    fn next_two_character_token() {
        let input = "+= -= *= /= %= ^= || && !~ == <= >= != ++ -- >>";
        let mut lexer = Lexer::new(input);

        let expected_tokens = vec![
            Token {
                kind: TokenKind::AddAssign,
                literal: "+=",
            },
            Token {
                kind: TokenKind::SubtractAssign,
                literal: "-=",
            },
            Token {
                kind: TokenKind::MultiplyAssign,
                literal: "*=",
            },
            Token {
                kind: TokenKind::DivideAssign,
                literal: "/=",
            },
            Token {
                kind: TokenKind::ModuloAssign,
                literal: "%=",
            },
            Token {
                kind: TokenKind::PowerAssign,
                literal: "^=",
            },
            Token {
                kind: TokenKind::Or,
                literal: "||",
            },
            Token {
                kind: TokenKind::And,
                literal: "&&",
            },
            Token {
                kind: TokenKind::NoMatch,
                literal: "!~",
            },
            Token {
                kind: TokenKind::Equal,
                literal: "==",
            },
            Token {
                kind: TokenKind::LessThanOrEqual,
                literal: "<=",
            },
            Token {
                kind: TokenKind::GreaterThanOrEqual,
                literal: ">=",
            },
            Token {
                kind: TokenKind::NotEqual,
                literal: "!=",
            },
            Token {
                kind: TokenKind::Increment,
                literal: "++",
            },
            Token {
                kind: TokenKind::Decrement,
                literal: "--",
            },
            Token {
                kind: TokenKind::Append,
                literal: ">>",
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

    #[test]
    fn is_ascii_alphabetic_lowercase() {
        assert!(is_ascii_alphabetic(Some(b'a')));
        assert!(is_ascii_alphabetic(Some(b'z')));
        assert!(is_ascii_alphabetic(Some(b'm')));
    }

    #[test]
    fn is_ascii_alphabetic_uppercase() {
        assert!(is_ascii_alphabetic(Some(b'A')));
        assert!(is_ascii_alphabetic(Some(b'Z')));
        assert!(is_ascii_alphabetic(Some(b'M')));
    }

    #[test]
    fn is_ascii_alphabetic_digits() {
        assert!(!is_ascii_alphabetic(Some(b'0')));
        assert!(!is_ascii_alphabetic(Some(b'5')));
        assert!(!is_ascii_alphabetic(Some(b'9')));
    }

    #[test]
    fn is_ascii_alphabetic_special_chars() {
        assert!(!is_ascii_alphabetic(Some(b'!')));
        assert!(!is_ascii_alphabetic(Some(b' ')));
        assert!(!is_ascii_alphabetic(Some(b'{')));
        assert!(!is_ascii_alphabetic(Some(b'=')));
    }

    #[test]
    fn is_ascii_alphabetic_none() {
        assert!(!is_ascii_alphabetic(None));
    }

    #[test]
    fn is_whitespace_space() {
        assert!(is_whitespace(Some(b' ')));
    }

    #[test]
    fn is_whitespace_special_chars() {
        assert!(!is_whitespace(Some(b'!')));
        assert!(!is_whitespace(Some(b'{')));
        assert!(!is_whitespace(Some(b'=')));
    }

    #[test]
    fn is_whitespace_none() {
        assert!(!is_whitespace(None));
    }

    #[test]
    fn is_digit_valid() {
        assert!(is_digit(Some(b'0')));
        assert!(is_digit(Some(b'5')));
        assert!(is_digit(Some(b'9')));
    }

    #[test]
    fn is_digit_invalid() {
        assert!(!is_digit(Some(b'a')));
        assert!(!is_digit(Some(b'z')));
        assert!(!is_digit(Some(b'A')));
        assert!(!is_digit(Some(b'Z')));
        assert!(!is_digit(Some(b'!')));
        assert!(!is_digit(Some(b' ')));
        assert!(!is_digit(Some(b'{')));
        assert!(!is_digit(Some(b'=')));
    }

    #[test]
    fn is_digit_none() {
        assert!(!is_digit(None));
    }
}
