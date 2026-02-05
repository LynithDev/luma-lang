mod tokens;

use std::str::Chars;

use luma_core::{CodeSource, Span};
pub use tokens::*;

use crate::{CompilerContext, CompilerStage};

pub struct LexerStage;

impl<'stage> CompilerStage<'stage> for LexerStage {
    type Input = &'stage [CodeSource];
    type Output = Vec<TokenList>;

    fn name() -> &'static str {
        "lexer"
    }

    fn process(self, _ctx: &CompilerContext, input: Self::Input) -> Self::Output {
        input.iter()
            .map(Self::tokenize)
            .collect()
    }
}

impl LexerStage {
    pub fn tokenize(input: &CodeSource) -> TokenList {
        let state = Tokenizer::new(input);
        state.process()
    }
}

struct Tokenizer<'a> {
    chars: Chars<'a>,
    tokens: TokenList,

    cursor: usize,
    span_start: usize,
    lexeme: String,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a CodeSource) -> Self {
        Self {
            chars: input.content.chars(),
            tokens: Vec::new(),

            cursor: 0,
            span_start: 0,
            lexeme: String::new(),
        }
    }

    pub fn process(mut self) -> Vec<Token> {
        self.scan_source();
        self.tokens
    }

    fn scan_source(&mut self) {
        loop {
            self.span_start = self.cursor;
            self.lexeme.clear();

            let Some(char) = self.advance() else {
                // end of source
                break;
            };

            let token_kind = match self.scan(char) {
                Some(kind) => kind,
                None => continue,
            };

            // now we add the token
            let token = Token {
                span: Span::new(self.span_start, self.cursor),
                kind: token_kind,
                lexeme: self.lexeme.clone(),
            };

            self.tokens.push(token);
        }
    }

    /// Scan a character (or more)
    fn scan(&mut self, c: char) -> Option<TokenKind> {
        // helper macro to match the next character and return different values based on that
        macro_rules! match_next {
            (
                $( $expected:expr => $on_match:expr ),+,
                else => $on_no_match:expr
            ) => {{
                if false { unreachable!()}
                $( else if self.match_next($expected) { $on_match } )+
                else { $on_no_match }
            }};
        }

        Some(match c {
            '\0' | '\n' | '\r' | '\t' | ' ' => return None,
            ';' => TokenKind::Semicolon,
            ',' => TokenKind::Comma,
            ':' => TokenKind::Colon,
            '.' => {
                if self.match_next('.') {
                    match_next!(
                        '=' => TokenKind::DotDotEqual,
                        else => TokenKind::DotDot
                    )
                } else if self.peek().is_some_and(|c| c.is_ascii_digit()) {
                    self.scan_numeric_literal(c)
                } else {
                    TokenKind::Dot
                }
            }
            '(' => TokenKind::LeftParen,
            ')' => TokenKind::RightParen,
            '{' => TokenKind::LeftBrace,
            '}' => TokenKind::RightBrace,
            '[' => TokenKind::LeftBracket,
            ']' => TokenKind::RightBracket,
            '+' => match_next!('=' => TokenKind::PlusEqual, else => TokenKind::Plus),
            '-' => match_next!('=' => TokenKind::MinusEqual, else => TokenKind::Minus),
            '/' => match_next!(
                '=' => TokenKind::SlashEqual,
                '/' => {
                    self.scan_single_line_comment();
                    return None;
                },
                '*' => {
                    self.scan_multi_line_comment();
                    return None;
                },
                else => TokenKind::Slash
            ),
            '*' => match_next!('=' => TokenKind::AsteriskEqual, else => TokenKind::Asterisk),
            '%' => match_next!('=' => TokenKind::PercentEqual, else => TokenKind::Percent),
            '=' => match_next!('=' => TokenKind::EqualEqual, else => TokenKind::Equal),
            '!' => match_next!('=' => TokenKind::BangEqual, else => TokenKind::Bang),
            '>' => match_next!(
                '=' => TokenKind::GreaterEqual,
                '>' => TokenKind::GreaterThanGreaterThan,
                else => TokenKind::Greater
            ),
            '<' => match_next!(
                '=' => TokenKind::LessEqual,
                '<' => match_next!(
                    '=' => TokenKind::LessThanLessThanEqual,
                    else => TokenKind::LessThanLessThan
                ),
                else => TokenKind::Less
            ),
            '&' => match_next!(
                '=' => TokenKind::AmpersandEqual,
                '&' => match_next!(
                    '=' => TokenKind::AmpersandAmpersandEqual,
                    else => TokenKind::AmpersandAmpersand
                ),
                else => TokenKind::Ampersand
            ),
            '|' => match_next!(
                '|' => match_next!(
                    '=' => TokenKind::PipePipeEqual,
                    else => TokenKind::PipePipe
                ),
                '=' => TokenKind::PipeEqual,
                else => TokenKind::Pipe
            ),
            '^' => match_next!(
                '=' => TokenKind::CaretEqual,
                else => TokenKind::Caret
            ),
            '\'' => self.scan_char_literal(),
            '"' => self.scan_string_literal(),
            '0'..='9' => self.scan_numeric_literal(c),
            _ => self.scan_identifier_or_keyword(),
        })
    }

    /// Scan a single line comment.
    /// Consumes characters until a newline or end of source is reached.
    fn scan_single_line_comment(&mut self) {
        while let Some(c) = self.peek() {
            if c == '\n' {
                break;
            }

            self.advance();
        }
    }

    /// Scan a multi line comment.
    /// Consumes characters until the closing `*/` is found or end of source is reached.
    fn scan_multi_line_comment(&mut self) {
        while let Some(c) = self.advance() {
            if c == '*' && self.match_next('/') {
                break;
            }
        }
    }

    /// Scan a char literal.
    fn scan_char_literal(&mut self) -> TokenKind {
        let c = match self.advance() {
            Some('\\') => {
                // escape sequence
                self.advance().and_then(escaped).unwrap_or('\\')
            }
            Some(c) => c,
            None => {
                // unterminated char literal
                return TokenKind::CharLiteral;
            }
        };

        // expect closing single quote
        if !self.match_next('\'') {
            todo!("handle unterminated char literal");
        }

        self.lexeme = c.to_string();

        TokenKind::CharLiteral
    }

    /// Scan a string literal.
    fn scan_string_literal(&mut self) -> TokenKind {
        let mut builder = String::new();

        while let Some(c) = self.advance() {
            match c {
                '"' => {
                    // end of string literal

                    self.lexeme = builder;
                    break;
                },
                '\\' => {
                    if let Some(escaped_char) = self.peek().and_then(escaped) {
                        // valid escape sequence
                        self.advance();
                        builder.push(escaped_char);
                    } else {
                        // invalid escape sequence, just add the backslash
                        builder.push('\\');
                    }
                }
                _ => {
                    // regular character
                    builder.push(c);
                }
            }
        }

        TokenKind::StringLiteral
    }

    /// Scan a numeric literal (integer or float)
    fn scan_numeric_literal(&mut self, c: char) -> TokenKind {
        enum NumberRadix {
            Decimal,
            Hexadecimal,
            Octal,
            Binary,
        }

        impl NumberRadix {
            fn is_valid_digit(&self, c: char) -> bool {
                match self {
                    NumberRadix::Decimal => c.is_ascii_digit(),
                    NumberRadix::Hexadecimal => c.is_ascii_hexdigit(),
                    NumberRadix::Octal => matches!(c, '0'..='7'),
                    NumberRadix::Binary => matches!(c, '0' | '1'),
                }
            }
        }

        let mut radix = NumberRadix::Decimal;
        let mut is_float = false;
        let mut num = String::new();

        // scan prefix
        match c {
            '.' => {
                is_float = true;
                num.push('.');
            }
            '0' => {
                num.push('0');
                match self.peek() {
                    Some('x' | 'X') => {
                        radix = NumberRadix::Hexadecimal;
                        self.advance();
                    }
                    Some('o' | 'O') => {
                        radix = NumberRadix::Octal;
                        self.advance();
                    }
                    Some('b' | 'B') => {
                        radix = NumberRadix::Binary;
                        self.advance();
                    }
                    Some('d' | 'D') => {
                        // explicit decimal, just advance
                        self.advance();
                    }
                    Some('_' | '0'..='9' | '.') => {}
                    _ => {
                        // just a single '0'
                        self.lexeme = num;
                        return TokenKind::IntLiteral;
                    }
                }
            }
            _ => num.push(c),
        }

        // scan number
        while let Some(c) = self.peek() {
            match c {
                '_' => {
                    self.advance();
                }

                '.' if !is_float && matches!(radix, NumberRadix::Decimal) => {
                    is_float = true;
                    num.push('.');
                    self.advance();
                }

                c if radix.is_valid_digit(c) => {
                    num.push(c);
                    self.advance();
                }

                _ => break,
            }
        }

        self.lexeme = num;

        if is_float {
            TokenKind::FloatLiteral
        } else {
            TokenKind::IntLiteral
        }
    }

    /// Scan an identifier or keyword.
    fn scan_identifier_or_keyword(&mut self) -> TokenKind {
        while let Some(c) = self.peek() {
            if c.is_ascii_alphanumeric() || c == '_' {
                // advance pushes to lexeme
                self.advance();
            } else {
                break;
            }
        }

        // check if its 'true' or 'false'
        if self.lexeme == "true" || self.lexeme == "false" {
            return TokenKind::BoolLiteral;
        }

        // check if its a keyword
        if let Some(keyword_kind) = TokenKind::try_from_keyword(&self.lexeme) {
            return keyword_kind;
        }

        // otherwise its an identifier
        TokenKind::Ident
    }

    /// Advance the lexer by one character.
    /// Will return [`None`] if at the end of the source otherwise returns the next character.
    fn advance(&mut self) -> Option<char> {
        let next = self.chars.next();

        if let Some(c) = next {
            self.cursor += c.len_utf8();
            self.lexeme.push(c);
        }

        next
    }

    /// Advances the lexer by one character if the next character matches the expected character.
    /// Returns `true` if the character was matched and advanced, otherwise returns `false`.
    #[must_use]
    fn match_next(&mut self, expected: char) -> bool {
        if self.peek() == Some(expected) {
            self.advance();
            true
        } else {
            false
        }
    }

    /// Peek the next character.
    /// Will return [`None`] if at the end of the source otherwise returns the next character.
    #[must_use]
    fn peek(&mut self) -> Option<char> {
        // clone is cheap for Chars (it's just the iterator)
        self.chars.clone().next()
    }
}

/// get char as escape sequence in a string literal.
fn escaped(c: char) -> Option<char> {
    Some(match c {
        'n' => '\n',
        'r' => '\r',
        't' => '\t',
        '\\' => '\\',
        '"' => '"',
        '\'' => '\'',
        '0' => '\0',
        _ => return None,
    })
}
