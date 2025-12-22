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

    pub fn next_token(&mut self) -> Token {
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
            _ => {
                if is_ascii_alphabetic(self.ch) {
                    return self.read_identifier();
                }
                Token {
                    kind: TokenKind::Illegal,
                    literal: "<illegal>",
                }
            }
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

    fn read_identifier(&mut self) -> Token {
        let position = self.position;
        while is_ascii_alphabetic(self.ch) {
            self.read_char();
        }
        let literal = &self.input[position..self.position];

        return lookup_keyword(literal);
    }

    fn skip_whitespace(&mut self) {
        while is_whitespace(self.ch) {
            self.read_char();
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
}
