use std::str::FromStr;

pub mod tokens;

use tokens::*;
use luma_core::{Cursor, NumberRadix, Span};
use luma_diagnostic::{ReporterExt, DiagnosticResult, Reporter};

pub(crate) mod diagnostics;
use crate::diagnostics::LexerDiagnostic;


pub struct LumaLexer<'a> {
    /// The source code as a Vec<char>
    pub(crate) source: &'a [u8],
    /// The scanned list of tokens from the source
    pub(crate) tokens: Vec<Token>,
    /// The current line number in source
    pub(crate) line: usize,
    /// The current column number in source
    pub(crate) column: usize,
    /// The start of the current token
    pub(crate) start: usize,
    /// The length of the current token
    pub(crate) length: usize,
    /// Diagnostic reporter for errors and warnings
    pub (crate) reporter: Reporter,
}

impl<'a> LumaLexer<'a> {
    pub fn new(source: &'a [u8], reporter: &Reporter) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            line: 1,
            column: 1,
            start: 0,
            length: 0,
            reporter: reporter.with_name("lexer"),
        }
    }

    pub fn scan(&mut self) -> TokenStream {
        while !self.is_at_end() {
            match self.scan_token() {
                Ok(None) => {},
                Ok(Some(token)) => self.tokens.push(token),
                Err(err) => self.reporter.report(err),
            }
        }

        let token = self.token(TokenKind::EndOfFile);
        self.tokens.push(token);
        
        TokenStream::new(self.tokens.clone())
    }

    fn scan_token(&mut self) -> DiagnosticResult<Option<Token>> {
        self.start += self.length;
        self.length = 0;

        let c = self.advance().unwrap_or('\0');

        let token_kind: TokenKind = match c {
            '\n' => {
                self.newline();
                return Ok(None);
            }
            ' ' | '\0' | '\t' => return Ok(None),
            '\r' => return Ok(None),

            ',' => TokenKind::Punctuation(PunctuationKind::Comma),
            ';' => TokenKind::Punctuation(PunctuationKind::Semicolon),
            ':' => TokenKind::Punctuation(PunctuationKind::Colon),
            '.' => TokenKind::Punctuation(PunctuationKind::Dot),
            '?' => TokenKind::Punctuation(PunctuationKind::QuestionMark),
            '(' => TokenKind::Punctuation(PunctuationKind::LeftParen),
            ')' => TokenKind::Punctuation(PunctuationKind::RightParen),
            '{' => TokenKind::Punctuation(PunctuationKind::LeftBrace),
            '}' => TokenKind::Punctuation(PunctuationKind::RightBrace),
            '[' => TokenKind::Punctuation(PunctuationKind::LeftBracket),
            ']' => TokenKind::Punctuation(PunctuationKind::RightBracket),

            '+' => {
                if self.eat_if('=') {
                    TokenKind::Operator(OperatorKind::PlusEquals)
                } else {
                    TokenKind::Operator(OperatorKind::Plus)
                }
            }
            '-' => {
                if self.eat_if('=') {
                    TokenKind::Operator(OperatorKind::MinusEquals)
                } else {
                    TokenKind::Operator(OperatorKind::Minus)
                }
            }
            '*' => {
                if self.eat_if('=') {
                    TokenKind::Operator(OperatorKind::AsteriskEquals)
                } else {
                    TokenKind::Operator(OperatorKind::Asterisk)
                }
            }
            '/' => {
                if self.eat_if('/') {
                    self.comment();
                    return Ok(None);
                } else if self.eat_if('=') {
                    TokenKind::Operator(OperatorKind::SlashEquals)
                } else {
                    TokenKind::Operator(OperatorKind::Slash)
                }
            },
            '%' => {
                if self.eat_if('=') {
                    TokenKind::Operator(OperatorKind::PercentEquals)
                } else {
                    TokenKind::Operator(OperatorKind::Percent)
                }
            }

            '=' => {
                if self.eat_if('=') {
                    TokenKind::Operator(OperatorKind::EqualsEquals)
                } else {
                    TokenKind::Operator(OperatorKind::Equals)
                }
            }
            '!' => {
                if self.eat_if('=') {
                    TokenKind::Operator(OperatorKind::NotEquals)
                } else {
                    TokenKind::Operator(OperatorKind::Not)
                }
            }
            '>' => {
                if self.eat_if('=') {
                    TokenKind::Operator(OperatorKind::GreaterThanOrEqual)
                } else if self.eat_if('>') {
                    if self.eat_if('=') {
                        TokenKind::Operator(OperatorKind::ShiftRightEquals)
                    } else {
                        TokenKind::Operator(OperatorKind::ShiftRight)
                    }
                } else {
                    TokenKind::Operator(OperatorKind::GreaterThan)
                }
            }
            '<' => {
                if self.eat_if('=') {
                    TokenKind::Operator(OperatorKind::LessThanOrEqual)
                } else if self.eat_if('<') {
                    if self.eat_if('=') {
                        TokenKind::Operator(OperatorKind::ShiftLeftEquals)
                    } else {
                        TokenKind::Operator(OperatorKind::ShiftLeft)
                    }
                } else {
                    TokenKind::Operator(OperatorKind::LessThan)
                }
            }
            '&' => {
                if self.eat_if('&') {
                    if self.eat_if('=') {
                        TokenKind::Operator(OperatorKind::AndEquals)
                    } else {
                        TokenKind::Operator(OperatorKind::And)
                    }
                } else if self.eat_if('=') {
                    TokenKind::Operator(OperatorKind::BitwiseAndEquals)
                } else {
                    TokenKind::Operator(OperatorKind::BitwiseAnd)
                }
            }
            '|' => {
                if self.eat_if('|') {
                    if self.eat_if('=') {
                        TokenKind::Operator(OperatorKind::OrEquals)
                    } else {
                        TokenKind::Operator(OperatorKind::Or)
                    }
                } else if self.eat_if('=') {
                    TokenKind::Operator(OperatorKind::BitwiseOrEquals)
                } else {
                    TokenKind::Operator(OperatorKind::BitwiseOr)
                }
            }
            '~' => {
                TokenKind::Operator(OperatorKind::BitwiseNot)
            }
            '^' => {
                if self.eat_if('=') {
                    TokenKind::Operator(OperatorKind::BitwiseXorEquals)
                } else {
                    TokenKind::Operator(OperatorKind::BitwiseXor)
                }
            }

            '\'' => return self.string('\'').map(Some),
            '"' => return self.string('"').map(Some),

            _ if self.is_digit(c) => return self.number().map(Some),
            _ if self.is_alpha(c) || c == '_' => return self.identifier().map(Some),
            _ => {
                return Err(self.diagnostic(LexerDiagnostic::UnexpectedCharacter(c)));
            }
        };

        Ok(Some(self.token(token_kind)))
    }

    fn number(&mut self) -> DiagnosticResult<Token> {
        if self.current_is('0') {
            let next = self.peek();
            if self.is_alpha(next) {
                if NumberRadix::try_from(next).is_err() {
                    return Err(self.diagnostic(LexerDiagnostic::UnexpectedRadixIdentifier(next)));
                }

                self.advance();
            }
            self.advance();
        }

        while let Some(current) = self.current() && self.is_digit(current) {
            self.advance();
        }

        let mut is_decimal = false;

        if self.current_is('.') && self.is_digit(self.peek()) {
            is_decimal = true;
            self.advance();

            while let Some(current) = self.current() && self.is_digit(current) {
                self.advance();
            }
        }

        let kind = if is_decimal {
            TokenKind::Literal(LiteralKind::Decimal)
        } else {
            TokenKind::Literal(LiteralKind::Integer)
        };

        Ok(self.token(kind))
    }

    fn identifier(&mut self) -> DiagnosticResult<Token> {
        loop {
            let current = self.current().unwrap_or('\0');
            if !self.is_alphanumeric(current) && current != '_' {
                break;
            }

            self.advance();
        }

        let lexeme = self.get_lexeme();

        if lexeme == "true" || lexeme == "false" {
            return Ok(self.token(TokenKind::Literal(LiteralKind::Boolean)));
        }

        let kind = KeywordKind::from_str(&lexeme)
            .ok()
            .map(TokenKind::Keyword)
            .unwrap_or(TokenKind::Identifier);

        Ok(self.token_lexeme(kind, lexeme))
    }

    fn string(&mut self, terminator: char) -> DiagnosticResult<Token> {
        let mut escaped = false;
        let mut str = String::new();
        
        while !self.is_at_end() {
            let c = self.current().unwrap_or('\0');
            
            if escaped {
                let char = match c {
                    '"' => '"',
                    '\'' => '\'',
                    '\\' => '\\',
                    'n' => '\n',
                    't' => '\t',
                    'r' => '\r',
                    c => c,
                };

                str.push(char);
                escaped = false;
            } else if c == '\\' {
                escaped = true;
            } else if c == terminator {
                break;
            } else {
                str.push(c);
            }

            self.advance();
        }

        if self.is_at_end() {
            return Err(self.diagnostic(LexerDiagnostic::UnterminatedString));
        }

        self.advance();

        Ok(self.token_lexeme(TokenKind::Literal(LiteralKind::String), str))
    }

    fn comment(&mut self) {
        while let Some(c) = self.advance() {
            if c == '\n' {
                self.newline();
                break;
            }
        }
    }


    fn token(&mut self, kind: TokenKind) -> Token {
        self.token_lexeme(kind, self.get_lexeme())
    }

    fn token_lexeme(&mut self, kind: TokenKind, lexeme: String) -> Token {
        Token {
            kind,
            cursor: Cursor {
                column: self.column.saturating_sub(lexeme.len()),
                line: self.line,
            },
            span: Span {
                start: self.start,
                end: self.start + self.length,
            },
            lexeme,
        }
    }

    fn newline(&mut self) {
        self.line += 1;
        self.column = 1;
    }

    fn eat_if(&mut self, expected: char) -> bool {
        if self.current().is_some_and(|c| c == expected) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.current()?;

        self.length += 1;
        self.column += 1;

        Some(c)
    }

    #[inline]
    #[must_use]
    fn current(&self) -> Option<char> {
        self.peek_by(0)
    }

    #[inline]
    #[must_use]
    fn current_is(&self, expected: char) -> bool {
        Some(expected).eq(&self.current())
    }

    #[inline]
    #[must_use]
    fn peek(&self) -> char {
        self.peek_by(1).unwrap_or('\0')
    }

    #[must_use]
    fn peek_by(&self, offset: usize) -> Option<char> {
        let index = self.start + self.length + offset;
        if index < self.source.len() {
            Some(self.source[index] as char)
        } else {
            None
        }
    }

    #[must_use]
    fn get_lexeme(&self) -> String {
        self.source[self.start..self.start + self.length]
            .iter()
            .map(|&c| c as char)
            .collect()
    }

    #[must_use]
    fn is_at_end(&self) -> bool {
        self.start + self.length >= self.source.len()
    }

    #[must_use]
    fn is_digit(&self, c: char) -> bool {
        c.is_ascii_digit()
    }

    #[must_use]
    fn is_alpha(&self, c: char) -> bool {
        c.is_ascii_alphabetic()
    }

    #[must_use]
    fn is_alphanumeric(&self, c: char) -> bool {
        self.is_alpha(c) || self.is_digit(c)
    }
}

