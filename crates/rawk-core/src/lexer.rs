use crate::token::{Token, TokenKind, lookup_functions, lookup_keyword};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LexErrorKind {
    UnsupportedCharacter,
    LoneAmpersand,
    InvalidLineContinuation,
    InvalidNumber,
    UnterminatedString,
    UnterminatedRegex,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LexError<'a> {
    pub kind: LexErrorKind,
    pub literal: &'a str,
    pub start: usize,
}

#[derive(Debug)]
pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
    read_position: usize,
    ch: Option<u8>,
    errors: Vec<LexError<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        let mut lexer = Lexer {
            input: src,
            position: 0,
            read_position: 0,
            ch: None,
            errors: Vec::new(),
        };

        lexer.read_char();
        lexer
    }

    pub fn next_token(&mut self) -> Token<'a> {
        self.next_token_impl(false)
    }

    pub fn next_token_regex_aware(&mut self) -> Token<'a> {
        self.next_token_impl(true)
    }

    pub fn errors(&self) -> &[LexError<'a>] {
        &self.errors
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    fn next_token_impl(&mut self, allow_regex: bool) -> Token<'a> {
        self.skip_whitespace();
        self.skip_comment();

        let start = self.position;
        let token = match self.ch {
            Some(b'{') => Token::new(TokenKind::LeftCurlyBrace, "{", start),
            Some(b'}') => Token::new(TokenKind::RightCurlyBrace, "}", start),
            Some(b'(') => Token::new(TokenKind::LeftParen, "(", start),
            Some(b')') => Token::new(TokenKind::RightParen, ")", start),
            Some(b'[') => Token::new(TokenKind::LeftSquareBracket, "[", start),
            Some(b']') => Token::new(TokenKind::RightSquareBracket, "]", start),
            Some(b',') => Token::new(TokenKind::Comma, ",", start),
            Some(b';') => Token::new(TokenKind::Semicolon, ";", start),
            Some(b'\n') => Token::new(TokenKind::NewLine, "<newline>", start),
            Some(b'+') => {
                if self.peek_char() == Some(b'=') {
                    self.read_char();
                    Token::new(TokenKind::AddAssign, "+=", start)
                } else if self.peek_char() == Some(b'+') {
                    self.read_char();
                    Token::new(TokenKind::Increment, "++", start)
                } else {
                    Token::new(TokenKind::Plus, "+", start)
                }
            }
            Some(b'-') => {
                if self.peek_char() == Some(b'=') {
                    self.read_char();
                    Token::new(TokenKind::SubtractAssign, "-=", start)
                } else if self.peek_char() == Some(b'-') {
                    self.read_char();
                    Token::new(TokenKind::Decrement, "--", start)
                } else {
                    Token::new(TokenKind::Minus, "-", start)
                }
            }
            Some(b'*') => {
                if self.peek_char() == Some(b'*') {
                    if self.peek_next_char() == Some(b'=') {
                        self.read_char();
                        self.read_char();
                        Token::new(TokenKind::PowerAssign, "**=", start)
                    } else {
                        self.read_char();
                        Token::new(TokenKind::Caret, "**", start)
                    }
                } else if self.peek_char() == Some(b'=') {
                    self.read_char();
                    Token::new(TokenKind::MultiplyAssign, "*=", start)
                } else {
                    Token::new(TokenKind::Asterisk, "*", start)
                }
            }
            Some(b'%') => {
                if self.peek_char() == Some(b'=') {
                    self.read_char();
                    Token::new(TokenKind::ModuloAssign, "%=", start)
                } else {
                    Token::new(TokenKind::Percent, "%", start)
                }
            }
            Some(b'^') => {
                if self.peek_char() == Some(b'=') {
                    self.read_char();
                    Token::new(TokenKind::PowerAssign, "^=", start)
                } else {
                    Token::new(TokenKind::Caret, "^", start)
                }
            }
            Some(b'!') => {
                if self.peek_char() == Some(b'=') {
                    self.read_char();
                    Token::new(TokenKind::NotEqual, "!=", start)
                } else if self.peek_char() == Some(b'~') {
                    self.read_char();
                    Token::new(TokenKind::NoMatch, "!~", start)
                } else {
                    Token::new(TokenKind::ExclamationMark, "!", start)
                }
            }
            Some(b'>') => {
                if self.peek_char() == Some(b'=') {
                    self.read_char();
                    Token::new(TokenKind::GreaterThanOrEqual, ">=", start)
                } else if self.peek_char() == Some(b'>') {
                    self.read_char();
                    Token::new(TokenKind::Append, ">>", start)
                } else {
                    Token::new(TokenKind::GreaterThan, ">", start)
                }
            }
            Some(b'<') => {
                if self.peek_char() == Some(b'=') {
                    self.read_char();
                    Token::new(TokenKind::LessThanOrEqual, "<=", start)
                } else {
                    Token::new(TokenKind::LessThan, "<", start)
                }
            }
            Some(b'|') => {
                if self.peek_char() == Some(b'|') {
                    self.read_char();
                    Token::new(TokenKind::Or, "||", start)
                } else {
                    Token::new(TokenKind::Pipe, "|", start)
                }
            }
            Some(b'?') => Token::new(TokenKind::QuestionMark, "?", start),
            Some(b':') => Token::new(TokenKind::Colon, ":", start),
            Some(b'~') => Token::new(TokenKind::Tilde, "~", start),
            Some(b'$') => Token::new(TokenKind::DollarSign, "$", start),
            Some(b'=') => {
                if self.peek_char() == Some(b'=') {
                    self.read_char();
                    Token::new(TokenKind::Equal, "==", start)
                } else {
                    Token::new(TokenKind::Assign, "=", start)
                }
            }
            Some(b'/') => {
                if allow_regex {
                    self.read_regex()
                } else if self.peek_char() == Some(b'=') {
                    self.read_char();
                    Token::new(TokenKind::DivideAssign, "/=", start)
                } else {
                    Token::new(TokenKind::Division, "/", start)
                }
            }
            Some(b'&') => {
                if self.peek_char() == Some(b'&') {
                    self.read_char();
                    Token::new(TokenKind::And, "&&", start)
                } else {
                    self.illegal_token(
                        LexErrorKind::LoneAmpersand,
                        start,
                        "<illegal>",
                        &self.input[start..self.read_position],
                    )
                }
            }
            Some(b'\\') => {
                if self.peek_char() == Some(b'\n') {
                    self.read_char();
                    Token::new(TokenKind::NewLine, "<newline>", start)
                } else if self.peek_char() == Some(b'\r') && self.peek_next_char() == Some(b'\n') {
                    self.read_char();
                    self.read_char();
                    Token::new(TokenKind::NewLine, "<newline>", start)
                } else {
                    self.illegal_token(
                        LexErrorKind::InvalidLineContinuation,
                        start,
                        "<illegal>",
                        &self.input[start..self.read_position],
                    )
                }
            }
            Some(b'"') => self.read_string(),
            ch if is_ascii_alphabetic(ch) || ch == Some(b'_') => self.read_identifier(),
            ch if is_digit(ch) => self.read_number(),
            Some(b'.')
                if self
                    .peek_char()
                    .is_some_and(|arg0: u8| is_digit(Some(arg0))) =>
            {
                self.read_number()
            }
            None => return Token::new(TokenKind::Eof, "", start),
            _ => self.illegal_token(
                LexErrorKind::UnsupportedCharacter,
                start,
                "<illegal>",
                &self.input[start..self.read_position],
            ),
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
        while is_ascii_alphabetic(self.ch) || is_digit(self.ch) || self.ch == Some(b'_') {
            self.read_char();
        }
        let literal = &self.input[position..self.position];

        let token = if let Some(token_kind) = lookup_keyword(literal) {
            Token::new(token_kind, literal, position)
        } else if let Some(token_kind) = lookup_functions(literal) {
            Token::new(token_kind, literal, position)
        } else {
            Token::new(TokenKind::Identifier, literal, position)
        };

        self.rewind_one();
        token
    }

    fn read_number(&mut self) -> Token<'a> {
        let position = self.position;
        let mut got_digit = false;

        if self.ch == Some(b'0')
            && matches!(self.peek_char(), Some(b'x') | Some(b'X'))
            && !is_hex_digit(self.peek_next_char())
        {
            let literal = &self.input[position..position + 1];
            return Token::new(TokenKind::Number, literal, position);
        }

        // consume leading digits
        if self.ch != Some(b'.') {
            got_digit = true;

            if self.ch == Some(b'0')
                && matches!(self.peek_char(), Some(b'x') | Some(b'X'))
                && is_hex_digit(self.peek_next_char())
            {
                // hex number
                self.read_char(); // consume '0'
                self.read_char(); // consume 'x' or 'X'

                while matches!(
                    self.ch,
                    Some(b'0'..=b'9') | Some(b'a'..=b'f') | Some(b'A'..=b'F')
                ) {
                    self.read_char();
                }

                let literal = &self.input[position..self.position];
                match u64::from_str_radix(&literal[2..], 16) {
                    Ok(_) => {
                        let token = Token::new(TokenKind::Number, literal, position);
                        self.rewind_one();
                        return token;
                    }
                    Err(_) => {
                        if self.ch.is_some() {
                            self.rewind_one();
                        }
                        return self.illegal_token(
                            LexErrorKind::InvalidNumber,
                            position,
                            "<illegal>",
                            literal,
                        );
                    }
                }
            }
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

        if matches!(self.ch, Some(b'e') | Some(b'E')) {
            let exponent_sign = self.peek_char();
            let exponent_digit = if matches!(exponent_sign, Some(b'+') | Some(b'-')) {
                self.peek_next_char()
            } else {
                exponent_sign
            };

            if is_digit(exponent_digit) {
                self.read_char();
                if matches!(self.ch, Some(b'+') | Some(b'-')) {
                    self.read_char();
                }
                while is_digit(self.ch) {
                    self.read_char();
                }
            }
        }

        if !got_digit {
            let literal = &self.input[position..self.position];
            return self.illegal_token(LexErrorKind::InvalidNumber, position, "<illegal>", literal);
        }

        let literal = &self.input[position..self.position];

        let token = Token::new(TokenKind::Number, literal, position);
        self.rewind_one();
        token
    }

    fn read_string(&mut self) -> Token<'a> {
        // skip opening quote
        self.read_char();
        let position = self.position;
        let mut escaped = false;

        while let Some(ch) = self.ch {
            if !escaped && ch == b'"' {
                break;
            }
            escaped = !escaped && ch == b'\\';
            self.read_char();
        }

        let literal = &self.input[position..self.position];

        if self.ch != Some(b'"') {
            return self.illegal_token(
                LexErrorKind::UnterminatedString,
                position,
                literal,
                literal,
            );
        };

        Token::new(TokenKind::String, literal, position)
    }

    fn read_regex(&mut self) -> Token<'a> {
        // skip opening slash
        self.read_char();
        let position = self.position;
        let mut escaped = false;

        while let Some(ch) = self.ch {
            if !escaped && ch == b'/' {
                break;
            }
            if !escaped && ch == b'\n' {
                break;
            }
            escaped = !escaped && ch == b'\\';

            self.read_char();
        }

        let literal = &self.input[position..self.position];

        if self.ch != Some(b'/') {
            if self.ch == Some(b'\n') {
                self.rewind_one();
            }
            return self.illegal_token(LexErrorKind::UnterminatedRegex, position, literal, literal);
        }

        Token::new(TokenKind::Regex, literal, position)
    }

    fn skip_whitespace(&mut self) {
        while is_whitespace(self.ch) {
            self.read_char();
        }
    }

    fn skip_comment(&mut self) {
        if Some(b'#') == self.ch {
            while self.ch != Some(b'\n') && self.ch.is_some() {
                self.read_char();
            }
        }
    }

    fn peek_char(&self) -> Option<u8> {
        if self.read_position >= self.input.len() {
            None
        } else {
            Some(self.input.as_bytes()[self.read_position])
        }
    }

    fn peek_next_char(&self) -> Option<u8> {
        let next = self.read_position + 1;
        if next >= self.input.len() {
            None
        } else {
            Some(self.input.as_bytes()[next])
        }
    }

    fn rewind_one(&mut self) {
        if self.position == 0 {
            return;
        }
        self.read_position = self.position;
        self.position -= 1;
        self.ch = Some(self.input.as_bytes()[self.position]);
    }

    fn illegal_token(
        &mut self,
        kind: LexErrorKind,
        start: usize,
        token_literal: &'a str,
        diagnostic_literal: &'a str,
    ) -> Token<'a> {
        self.errors.push(LexError {
            kind,
            literal: diagnostic_literal,
            start,
        });
        Token::new(TokenKind::Illegal, token_literal, start)
    }
}

fn is_ascii_alphabetic(ch: Option<u8>) -> bool {
    match ch {
        Some(byte) => byte.is_ascii_alphabetic(),
        None => false,
    }
}

fn is_whitespace(ch: Option<u8>) -> bool {
    match ch {
        Some(byte) => byte == b' ' || byte == b'\t' || byte == b'\r',
        None => false,
    }
}

fn is_digit(ch: Option<u8>) -> bool {
    match ch {
        Some(byte) => byte.is_ascii_digit(),
        None => false,
    }
}

fn is_hex_digit(ch: Option<u8>) -> bool {
    match ch {
        Some(byte) => byte.is_ascii_hexdigit(),
        None => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_token(token: Token<'_>, kind: TokenKind, literal: &str) {
        assert_eq!(kind, token.kind);
        assert_eq!(literal, token.literal);
    }

    fn assert_lex_error(error: LexError<'_>, kind: LexErrorKind, literal: &str, start: usize) {
        assert_eq!(kind, error.kind);
        assert_eq!(literal, error.literal);
        assert_eq!(start, error.start);
    }

    #[test]
    fn empty_input_returns_eof_token() {
        let input = "";
        let mut lexer = Lexer::new(input);

        let token = lexer.next_token();

        assert_token(token, TokenKind::Eof, "");
    }

    #[test]
    fn next_left_curly_brace_token() {
        let expected_token = Token::new(TokenKind::LeftCurlyBrace, "{", 0);
        let input = "{";
        let mut lexer = Lexer::new(input);

        let token = lexer.next_token();

        assert_eq!(expected_token, token);
    }

    #[test]
    fn next_right_curly_brace_token() {
        let input = "}";
        let mut lexer = Lexer::new(input);

        let token = lexer.next_token();

        assert_token(token, TokenKind::RightCurlyBrace, "}");
    }

    #[test]
    fn next_pipe_token() {
        let input = "|";
        let mut lexer = Lexer::new(input);

        let token = lexer.next_token();

        assert_token(token, TokenKind::Pipe, "|");
    }

    #[test]
    fn next_one_character_token() {
        let input = "{}()[],;\n+-*/%^!><|?:~$=";
        let mut lexer = Lexer::new(input);
        let expected_tokens = vec![
            (TokenKind::LeftCurlyBrace, "{"),
            (TokenKind::RightCurlyBrace, "}"),
            (TokenKind::LeftParen, "("),
            (TokenKind::RightParen, ")"),
            (TokenKind::LeftSquareBracket, "["),
            (TokenKind::RightSquareBracket, "]"),
            (TokenKind::Comma, ","),
            (TokenKind::Semicolon, ";"),
            (TokenKind::NewLine, "<newline>"),
            (TokenKind::Plus, "+"),
            (TokenKind::Minus, "-"),
            (TokenKind::Asterisk, "*"),
            (TokenKind::Division, "/"),
            (TokenKind::Percent, "%"),
            (TokenKind::Caret, "^"),
            (TokenKind::ExclamationMark, "!"),
            (TokenKind::GreaterThan, ">"),
            (TokenKind::LessThan, "<"),
            (TokenKind::Pipe, "|"),
            (TokenKind::QuestionMark, "?"),
            (TokenKind::Colon, ":"),
            (TokenKind::Tilde, "~"),
            (TokenKind::DollarSign, "$"),
            (TokenKind::Assign, "="),
            (TokenKind::Eof, ""),
        ];

        for (expected_kind, expected_literal) in expected_tokens {
            let token = lexer.next_token();
            assert_token(token, expected_kind, expected_literal);
        }
    }

    #[test]
    fn next_while_token() {
        let expected_token = Token::new(TokenKind::While, "while", 1);
        let input = " while";
        let mut lexer = Lexer::new(input);

        let token = lexer.next_token();

        assert_eq!(expected_token, token);
    }

    #[test]
    fn next_identifier_token() {
        let input = "BEGIN END break continue delete do else exit for function if in next print printf return while";
        let mut lexer = Lexer::new(input);

        let expected_tokens = vec![
            (TokenKind::Begin, "BEGIN"),
            (TokenKind::End, "END"),
            (TokenKind::Break, "break"),
            (TokenKind::Continue, "continue"),
            (TokenKind::Delete, "delete"),
            (TokenKind::Do, "do"),
            (TokenKind::Else, "else"),
            (TokenKind::Exit, "exit"),
            (TokenKind::For, "for"),
            (TokenKind::Function, "function"),
            (TokenKind::If, "if"),
            (TokenKind::In, "in"),
            (TokenKind::Next, "next"),
            (TokenKind::Print, "print"),
            (TokenKind::Printf, "printf"),
            (TokenKind::Return, "return"),
            (TokenKind::While, "while"),
            (TokenKind::Eof, ""),
        ];

        for (expected_kind, expected_literal) in expected_tokens {
            let token = lexer.next_token();
            assert_token(token, expected_kind, expected_literal);
        }
    }

    #[test]
    fn next_number_token() {
        let input = "123 4567 890 42.0 .75 0.001";
        let mut lexer = Lexer::new(input);

        let expected_tokens = vec![
            (TokenKind::Number, "123"),
            (TokenKind::Number, "4567"),
            (TokenKind::Number, "890"),
            (TokenKind::Number, "42.0"),
            (TokenKind::Number, ".75"),
            (TokenKind::Number, "0.001"),
            (TokenKind::Eof, ""),
        ];

        for (expected_kind, expected_literal) in expected_tokens {
            let token = lexer.next_token();
            assert_token(token, expected_kind, expected_literal);
        }
    }

    #[test]
    fn hex_number_token() {
        let input = "0xAA 0xaa 0xFEED 0xBEAF";
        let mut lexer = Lexer::new(input);

        let expected_tokens = vec![
            (TokenKind::Number, "0xAA"),
            (TokenKind::Number, "0xaa"),
            (TokenKind::Number, "0xFEED"),
            (TokenKind::Number, "0xBEAF"),
            (TokenKind::Eof, ""),
        ];

        for (expected_kind, expected_literal) in expected_tokens {
            let token = lexer.next_token();
            assert_token(token, expected_kind, expected_literal);
        }
    }

    #[test]
    fn invalid_hex_number_token() {
        let input = "0xG1 5x03";
        let mut lexer = Lexer::new(input);

        let expected_tokens = vec![
            (TokenKind::Number, "0"),
            (TokenKind::Identifier, "xG1"),
            (TokenKind::Number, "5"),
            (TokenKind::Identifier, "x03"),
            (TokenKind::Eof, ""),
        ];

        for (expected_kind, expected_literal) in expected_tokens {
            let token = lexer.next_token();
            assert_token(token, expected_kind, expected_literal);
        }
    }

    #[test]
    fn next_or_token() {
        let expected_token = Token::new(TokenKind::Or, "||", 0);
        let input = "||";
        let mut lexer = Lexer::new(input);

        let token = lexer.next_token();

        assert_eq!(expected_token, token);
    }

    #[test]
    fn next_two_character_token() {
        let input = "+= -= *= /= %= ^= **= ** || && !~ == <= >= != ++ -- >>";
        let mut lexer = Lexer::new(input);

        let expected_tokens = vec![
            (TokenKind::AddAssign, "+="),
            (TokenKind::SubtractAssign, "-="),
            (TokenKind::MultiplyAssign, "*="),
            (TokenKind::DivideAssign, "/="),
            (TokenKind::ModuloAssign, "%="),
            (TokenKind::PowerAssign, "^="),
            (TokenKind::PowerAssign, "**="),
            (TokenKind::Caret, "**"),
            (TokenKind::Or, "||"),
            (TokenKind::And, "&&"),
            (TokenKind::NoMatch, "!~"),
            (TokenKind::Equal, "=="),
            (TokenKind::LessThanOrEqual, "<="),
            (TokenKind::GreaterThanOrEqual, ">="),
            (TokenKind::NotEqual, "!="),
            (TokenKind::Increment, "++"),
            (TokenKind::Decrement, "--"),
            (TokenKind::Append, ">>"),
            (TokenKind::Eof, ""),
        ];

        for (expected_kind, expected_literal) in expected_tokens {
            let token = lexer.next_token();
            assert_token(token, expected_kind, expected_literal);
        }
    }

    #[test]
    fn consume_comment() {
        let input = "# This is a comment\n123";
        let mut lexer = Lexer::new(input);

        let expected_tokens = vec![
            (TokenKind::NewLine, "<newline>"),
            (TokenKind::Number, "123"),
            (TokenKind::Eof, ""),
        ];

        for (expected_kind, expected_literal) in expected_tokens {
            let token = lexer.next_token();
            assert_token(token, expected_kind, expected_literal);
        }
    }

    #[test]
    fn expect_newline_after_backslash() {
        let input = "123 \\\n456";
        let mut lexer = Lexer::new(input);

        let expected_tokens = vec![
            (TokenKind::Number, "123"),
            (TokenKind::NewLine, "<newline>"),
            (TokenKind::Number, "456"),
            (TokenKind::Eof, ""),
        ];
        for (expected_kind, expected_literal) in expected_tokens {
            let token = lexer.next_token();
            assert_token(token, expected_kind, expected_literal);
        }
    }

    #[test]
    fn scientific_number_token() {
        let input = "1E2 12e-2 .75e+1";
        let mut lexer = Lexer::new(input);

        let expected_tokens = vec![
            (TokenKind::Number, "1E2"),
            (TokenKind::Number, "12e-2"),
            (TokenKind::Number, ".75e+1"),
            (TokenKind::Eof, ""),
        ];

        for (expected_kind, expected_literal) in expected_tokens {
            let token = lexer.next_token();
            assert_token(token, expected_kind, expected_literal);
        }
    }

    #[test]
    fn expect_newline_after_backslash_with_crlf() {
        let input = "123 \\\r\n456";
        let mut lexer = Lexer::new(input);

        let expected_tokens = vec![
            (TokenKind::Number, "123"),
            (TokenKind::NewLine, "<newline>"),
            (TokenKind::Number, "456"),
            (TokenKind::Eof, ""),
        ];
        for (expected_kind, expected_literal) in expected_tokens {
            let token = lexer.next_token();
            assert_token(token, expected_kind, expected_literal);
        }
    }

    #[test]
    fn backslash_without_newline_is_illegal() {
        let input = "123 \\ 456";
        let mut lexer = Lexer::new(input);
        let expected_tokens = vec![
            (TokenKind::Number, "123"),
            (TokenKind::Illegal, "<illegal>"),
            (TokenKind::Number, "456"),
            (TokenKind::Eof, ""),
        ];

        for (expected_kind, expected_literal) in expected_tokens {
            let token = lexer.next_token();
            assert_token(token, expected_kind, expected_literal);
        }
    }

    #[test]
    fn read_string_token() {
        let input = r#""Hello, World!" 123 "Hello, again!";"#;
        let mut lexer = Lexer::new(input);
        let expected_tokens = vec![
            (TokenKind::String, "Hello, World!"),
            (TokenKind::Number, "123"),
            (TokenKind::String, "Hello, again!"),
            (TokenKind::Semicolon, ";"),
            (TokenKind::Eof, ""),
        ];

        for (expected_kind, expected_literal) in expected_tokens {
            let token = lexer.next_token();
            assert_token(token, expected_kind, expected_literal);
        }
    }

    #[test]
    fn read_string_token_with_escaped_quote() {
        let input = r#""\"""#;
        let mut lexer = Lexer::new(input);

        let token = lexer.next_token();
        assert_token(token, TokenKind::String, r#"\""#);

        let token = lexer.next_token();
        assert_token(token, TokenKind::Eof, "");
    }

    #[test]
    fn read_regex_token_when_allowed() {
        let input = r"/foo\//";
        let mut lexer = Lexer::new(input);

        let token = lexer.next_token_regex_aware();

        assert_token(token, TokenKind::Regex, r"foo\/");
    }

    #[test]
    fn slash_is_division_when_regex_not_allowed() {
        let input = "/foo/";
        let mut lexer = Lexer::new(input);

        let token = lexer.next_token();

        assert_token(token, TokenKind::Division, "/");
    }

    #[test]
    fn unterminated_string_token() {
        let input = r#""This is an unterminated string"#;
        let mut lexer = Lexer::new(input);

        let token = lexer.next_token();
        assert_token(token, TokenKind::Illegal, "This is an unterminated string");
    }

    #[test]
    fn lone_ampersand_is_illegal() {
        let input = "&";
        let mut lexer = Lexer::new(input);

        let token = lexer.next_token();

        assert_token(token, TokenKind::Illegal, "<illegal>");
        assert_eq!(1, lexer.errors().len());
        assert_lex_error(lexer.errors()[0], LexErrorKind::LoneAmpersand, "&", 0);
    }

    #[test]
    fn unsupported_character_is_illegal() {
        let input = "@";
        let mut lexer = Lexer::new(input);

        let token = lexer.next_token();

        assert_token(token, TokenKind::Illegal, "<illegal>");
        assert_eq!(1, lexer.errors().len());
        assert_lex_error(
            lexer.errors()[0],
            LexErrorKind::UnsupportedCharacter,
            "@",
            0,
        );
    }

    #[test]
    fn unterminated_regex_token_is_illegal() {
        let input = r"/foo";
        let mut lexer = Lexer::new(input);

        let token = lexer.next_token_regex_aware();

        assert_token(token, TokenKind::Illegal, "foo");
        assert_eq!(1, lexer.errors().len());
        assert_lex_error(lexer.errors()[0], LexErrorKind::UnterminatedRegex, "foo", 1);
    }

    #[test]
    fn bare_dot_is_illegal() {
        let input = ".";
        let mut lexer = Lexer::new(input);

        let token = lexer.next_token();

        assert_token(token, TokenKind::Illegal, "<illegal>");
        assert_eq!(1, lexer.errors().len());
        assert_lex_error(
            lexer.errors()[0],
            LexErrorKind::UnsupportedCharacter,
            ".",
            0,
        );
    }

    #[test]
    fn backslash_without_newline_records_diagnostic() {
        let input = "\\";
        let mut lexer = Lexer::new(input);

        let token = lexer.next_token();

        assert_token(token, TokenKind::Illegal, "<illegal>");
        assert_eq!(1, lexer.errors().len());
        assert_lex_error(
            lexer.errors()[0],
            LexErrorKind::InvalidLineContinuation,
            "\\",
            0,
        );
    }

    #[test]
    fn has_errors_tracks_whether_diagnostics_were_recorded() {
        let mut clean_lexer = Lexer::new("123");
        assert!(!clean_lexer.has_errors());

        let token = clean_lexer.next_token();
        assert_token(token, TokenKind::Number, "123");
        assert!(!clean_lexer.has_errors());

        let mut error_lexer = Lexer::new("@");
        assert!(!error_lexer.has_errors());

        let token = error_lexer.next_token();
        assert_token(token, TokenKind::Illegal, "<illegal>");
        assert!(error_lexer.has_errors());
    }

    #[test]
    fn read_number_without_any_digits_is_invalid_number() {
        let mut lexer = Lexer::new(".");

        let token = lexer.read_number();

        assert_token(token, TokenKind::Illegal, "<illegal>");
        assert!(lexer.has_errors());
        assert_eq!(1, lexer.errors().len());
        assert_lex_error(lexer.errors()[0], LexErrorKind::InvalidNumber, ".", 0);
    }

    #[test]
    fn peek_next_char_returns_none_at_end_of_input() {
        let lexer = Lexer::new("a");

        assert_eq!(None, lexer.peek_next_char());
    }

    #[test]
    fn rewind_one_is_noop_when_position_is_zero() {
        let mut lexer = Lexer::new("a");

        lexer.rewind_one();

        assert_eq!(0, lexer.position);
        assert_eq!(1, lexer.read_position);
        assert_eq!(Some(b'a'), lexer.ch);
    }

    #[test]
    fn unterminated_string_records_diagnostic() {
        let input = r#""unterminated"#;
        let mut lexer = Lexer::new(input);

        let token = lexer.next_token();

        assert_token(token, TokenKind::Illegal, "unterminated");
        assert_eq!(1, lexer.errors().len());
        assert_lex_error(
            lexer.errors()[0],
            LexErrorKind::UnterminatedString,
            "unterminated",
            1,
        );
    }

    #[test]
    fn overflowing_hex_number_records_diagnostic_and_preserves_next_token() {
        let input = "0x10000000000000000z";
        let mut lexer = Lexer::new(input);

        let token = lexer.next_token();
        assert_token(token, TokenKind::Illegal, "<illegal>");

        let token = lexer.next_token();
        assert_token(token, TokenKind::Identifier, "z");

        assert_eq!(1, lexer.errors().len());
        assert_lex_error(
            lexer.errors()[0],
            LexErrorKind::InvalidNumber,
            "0x10000000000000000",
            0,
        );
    }

    #[test]
    fn unterminated_regex_before_newline_preserves_newline_token() {
        let input = "/foo\n123";
        let mut lexer = Lexer::new(input);

        let token = lexer.next_token_regex_aware();
        assert_token(token, TokenKind::Illegal, "foo");

        let token = lexer.next_token_regex_aware();
        assert_token(token, TokenKind::NewLine, "<newline>");

        let token = lexer.next_token();
        assert_token(token, TokenKind::Number, "123");

        assert_eq!(1, lexer.errors().len());
        assert_lex_error(lexer.errors()[0], LexErrorKind::UnterminatedRegex, "foo", 1);
    }

    #[test]
    fn built_in_functions() {
        let input = "atan2 close cos exp gsub index int length log match rand sin split sprintf sqrt srand sub substr system tolower toupper";
        let mut lexer = Lexer::new(input);
        let expected_tokens = vec![
            (TokenKind::Atan2, "atan2"),
            (TokenKind::Close, "close"),
            (TokenKind::Cos, "cos"),
            (TokenKind::Exp, "exp"),
            (TokenKind::Gsub, "gsub"),
            (TokenKind::Index, "index"),
            (TokenKind::Int, "int"),
            (TokenKind::Length, "length"),
            (TokenKind::Log, "log"),
            (TokenKind::Match, "match"),
            (TokenKind::Rand, "rand"),
            (TokenKind::Sin, "sin"),
            (TokenKind::Split, "split"),
            (TokenKind::Sprintf, "sprintf"),
            (TokenKind::Sqrt, "sqrt"),
            (TokenKind::Srand, "srand"),
            (TokenKind::Sub, "sub"),
            (TokenKind::Substr, "substr"),
            (TokenKind::System, "system"),
            (TokenKind::ToLower, "tolower"),
            (TokenKind::ToUpper, "toupper"),
            (TokenKind::Eof, ""),
        ];

        for (expected_kind, expected_literal) in expected_tokens {
            let token = lexer.next_token();
            assert_token(token, expected_kind, expected_literal);
        }
    }

    #[test]
    fn test_identifiers() {
        let input = "my_variable, anotherVar _privateVar var123";
        let mut lexer = Lexer::new(input);

        let expected_tokens = vec![
            (TokenKind::Identifier, "my_variable"),
            (TokenKind::Comma, ","),
            (TokenKind::Identifier, "anotherVar"),
            (TokenKind::Identifier, "_privateVar"),
            (TokenKind::Identifier, "var123"),
            (TokenKind::Eof, ""),
        ];

        for (expected_kind, expected_literal) in expected_tokens {
            let token = lexer.next_token();
            assert_token(token, expected_kind, expected_literal);
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
        assert!(is_whitespace(Some(b' ')), "space is considered whitespace");
        assert!(is_whitespace(Some(b'\t')), "tab is considered whitespace");
        assert!(
            is_whitespace(Some(b'\r')),
            "carriage return is considered whitespace"
        );
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

    #[test]
    fn is_hex_digit_valid() {
        assert!(is_hex_digit(Some(b'0')));
        assert!(is_hex_digit(Some(b'5')));
        assert!(is_hex_digit(Some(b'9')));
        assert!(is_hex_digit(Some(b'a')));
        assert!(is_hex_digit(Some(b'f')));
        assert!(is_hex_digit(Some(b'A')));
        assert!(is_hex_digit(Some(b'F')));
    }

    #[test]
    fn is_hex_digit_invalid() {
        assert!(!is_hex_digit(Some(b'g')));
        assert!(!is_hex_digit(Some(b'z')));
        assert!(!is_hex_digit(Some(b'G')));
        assert!(!is_hex_digit(Some(b'Z')));
        assert!(!is_hex_digit(Some(b'!')));
        assert!(!is_hex_digit(Some(b' ')));
        assert!(!is_hex_digit(Some(b'{')));
        assert!(!is_hex_digit(Some(b'=')));
        assert!(!is_hex_digit(None));
    }
}
