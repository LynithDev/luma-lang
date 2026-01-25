use luma_core::{Operator, Span, ast::{Symbol, SymbolKind}};
use strum::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub lexeme: String,
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    #[must_use]
    pub fn as_symbol(&self) -> Symbol {
        Symbol::spanned(
            self.span,
            SymbolKind::named(self.lexeme.clone()),
        )
    }

    #[must_use]
    pub fn new(kind: TokenKind, lexeme: String) -> Self {
        Self {
            span: Span::default(),
            kind,
            lexeme,
        }
    }
}

#[macro_export]
macro_rules! create_tokens {
    (
        $(
            $kind:tt $(=> $lexeme:expr)?
        ),* $(,)?
    ) => {{
        use $crate::lexer::{Token, TokenKind};

        vec![
            $(
                Token::new(
                    TokenKind::$kind,
                    create_tokens!(@lexeme TokenKind::$kind $(, $lexeme)?)
                )
            ),*
        ]
    }};

    // explicit lexeme
    (@lexeme $kind:expr, $lexeme:expr) => {
        $lexeme.to_string()
    };

    // implicit lexeme via Display
    (@lexeme $kind:expr) => {
        $kind.to_string()
    };
}

#[derive(Display, Debug, Clone, PartialEq, Eq)]
#[strum(serialize_all = "snake_case")]
pub enum TokenKind {
    //
    // === Keywords ===
    //

    /// var
    #[strum(serialize = "var")]
    Var,
    /// func
    #[strum(serialize = "func")]
    Func,
    /// if
    #[strum(serialize = "if")]
    If,
    /// else
    #[strum(serialize = "else")]
    Else,
    /// return
    #[strum(serialize = "return")]
    Return,
    /// while
    #[strum(serialize = "while")]
    While,
    /// for
    #[strum(serialize = "for")]
    For,
    /// break
    #[strum(serialize = "break")]
    Break,
    /// continue
    #[strum(serialize = "continue")]
    Continue,
    /// struct
    #[strum(serialize = "struct")]
    Struct,
    /// pub
    #[strum(serialize = "pub")]
    Pub,
    /// import
    #[strum(serialize = "import")]
    Import,
    /// this
    #[strum(serialize = "this")]
    This,
    /// module
    #[strum(serialize = "module")]
    Module,
    /// as
    #[strum(serialize = "as")]
    As,

    //
    // === Punctuation ===
    //
    /// ,
    #[strum(serialize = ",")]
    Comma,
    /// ;
    #[strum(serialize = ";")]
    Semicolon,
    /// :
    #[strum(serialize = ":")]
    Colon,
    /// .
    #[strum(serialize = ".")]
    Dot,
    /// ..
    #[strum(serialize = "..")]
    DotDot,
    /// ..=
    #[strum(serialize = "..=")]
    DotDotEqual,
    /// (
    #[strum(serialize = "(")]
    LeftParen,
    /// )
    #[strum(serialize = ")")]
    RightParen,
    /// {
    #[strum(serialize = "{")]
    LeftBrace,
    /// }
    #[strum(serialize = "}}")]
    RightBrace,
    /// [
    #[strum(serialize = "[")]
    LeftBracket,
    /// ]
    #[strum(serialize = "]")]
    RightBracket,
    
    //
    // === Operators ===
    //
    /// +
    #[strum(serialize = "+")]
    Plus,
    /// +=
    #[strum(serialize = "+=")]
    PlusEqual,
    /// -
    #[strum(serialize = "-")]
    Minus,
    /// -=
    #[strum(serialize = "-=")]
    MinusEqual,
    /// *
    #[strum(serialize = "*")]
    Asterisk,
    /// *=
    #[strum(serialize = "*=")]
    AsteriskEqual,
    /// /
    #[strum(serialize = "/")]
    Slash,
    /// /=
    #[strum(serialize = "/=")]
    SlashEqual,
    /// %
    #[strum(serialize = "%")]
    Percent,
    /// %=
    #[strum(serialize = "%=")]
    PercentEqual,
    /// =
    #[strum(serialize = "=")]
    Equal,
    /// ==
    #[strum(serialize = "==")]
    EqualEqual,
    /// >
    #[strum(serialize = ">")]
    Greater,
    /// >=
    #[strum(serialize = ">=")]
    GreaterEqual,
    /// >>
    #[strum(serialize = ">>")]
    GreaterThanGreaterThan,
    /// >>=
    #[strum(serialize = ">>=")]
    GreaterThanGreaterThanEqual,
    /// <
    #[strum(serialize = "<")]
    Less,
    /// <=
    #[strum(serialize = "<=")]
    LessEqual,
    /// <<
    #[strum(serialize = "<<")]
    LessThanLessThan,
    /// <<=
    #[strum(serialize = "<<=")]
    LessThanLessThanEqual,
    /// ||
    #[strum(serialize = "||")]
    PipePipe,
    /// ||=
    #[strum(serialize = "||=")]
    PipePipeEqual,
    /// !
    #[strum(serialize = "!")]
    Bang,
    /// !=
    #[strum(serialize = "!=")]
    BangEqual,
    /// |
    #[strum(serialize = "|")]
    Pipe,
    /// |=
    #[strum(serialize = "|=")]
    PipeEqual,
    /// &
    #[strum(serialize = "&")]
    Ampersand,
    /// &=
    #[strum(serialize = "&=")]
    AmpersandEqual,
    /// &&
    #[strum(serialize = "&&")]
    AmpersandAmpersand,
    /// &&=
    #[strum(serialize = "&&=")]
    AmpersandAmpersandEqual,
    /// ^
    #[strum(serialize = "^")]
    Caret,
    /// ^=
    #[strum(serialize = "^=")]
    CaretEqual,

    //
    // === Literals ===
    //
    /// valid identifiers begin with a letter or underscore, followed by letters, digits or underscores
    #[strum(serialize = "<identifier>")]
    Ident,
    /// char literals enclosed in single quotes (e.g. 'a', 'b', '1')
    #[strum(serialize = "char")]
    CharLiteral,
    /// integer literals (e.g. 123, 0, 4567)
    #[strum(serialize = "int")]
    IntLiteral,
    /// floating-point literals (e.g. 3.14, 0.0, 2.718)
    #[strum(serialize = "float")]
    FloatLiteral,
    /// string literals enclosed in double quotes (e.g. "hello", "world")
    #[strum(serialize = "str")]
    StringLiteral,
    /// boolean literals: true or false
    #[strum(serialize = "bool")]
    BoolLiteral,
}

impl TokenKind {
    pub fn try_from_keyword(value: &str) -> Option<Self> {
        Some(match value {
            "var" => TokenKind::Var,
            "func" => TokenKind::Func,
            "if" => TokenKind::If,
            "else" => TokenKind::Else,
            "return" => TokenKind::Return,
            "while" => TokenKind::While,
            "for" => TokenKind::For,
            "break" => TokenKind::Break,
            "continue" => TokenKind::Continue,
            "struct" => TokenKind::Struct,
            "pub" => TokenKind::Pub,
            "import" => TokenKind::Import,
            "module" => TokenKind::Module,
            "as" => TokenKind::As,
            _ => return None,
        })
    }
}

impl TryFrom<TokenKind> for Operator {
    type Error = ();

    fn try_from(value: TokenKind) -> Result<Self, Self::Error> {
        match value {
            // others
            TokenKind::Equal => Ok(Operator::Assign),
            TokenKind::Bang => Ok(Operator::Not),

            // arithmetic
            TokenKind::Plus => Ok(Operator::Add),
            TokenKind::Minus => Ok(Operator::Subtract),
            TokenKind::Asterisk => Ok(Operator::Multiply),
            TokenKind::Slash => Ok(Operator::Divide),
            TokenKind::Percent => Ok(Operator::Modulo),

            TokenKind::PlusEqual => Ok(Operator::AddAssign),
            TokenKind::MinusEqual => Ok(Operator::SubtractAssign),
            TokenKind::AsteriskEqual => Ok(Operator::MultiplyAssign),
            TokenKind::SlashEqual => Ok(Operator::DivideAssign),
            TokenKind::PercentEqual => Ok(Operator::ModuloAssign),

            // logic
            TokenKind::AmpersandAmpersand => Ok(Operator::And),
            TokenKind::PipePipe => Ok(Operator::Or),

            TokenKind::AmpersandAmpersandEqual => Ok(Operator::AndAssign),
            TokenKind::PipePipeEqual => Ok(Operator::OrAssign),

            // comparison
            TokenKind::EqualEqual => Ok(Operator::Equal),
            TokenKind::BangEqual => Ok(Operator::NotEqual),
            TokenKind::Less => Ok(Operator::LessThan),
            TokenKind::Greater => Ok(Operator::GreaterThan),
            TokenKind::LessEqual => Ok(Operator::LessThanOrEqual),
            TokenKind::GreaterEqual => Ok(Operator::GreaterThanOrEqual),

            // bitwise
            TokenKind::Ampersand => Ok(Operator::BitwiseAnd),
            TokenKind::Pipe => Ok(Operator::BitwiseOr),
            TokenKind::Caret => Ok(Operator::BitwiseXor),
            TokenKind::LessThanLessThan => Ok(Operator::ShiftLeft),
            TokenKind::GreaterThanGreaterThan => Ok(Operator::ShiftRight),

            TokenKind::AmpersandEqual => Ok(Operator::BitwiseAndAssign),
            TokenKind::PipeEqual => Ok(Operator::BitwiseOrAssign),
            TokenKind::CaretEqual => Ok(Operator::BitwiseXorAssign),
            TokenKind::LessThanLessThanEqual => Ok(Operator::ShiftLeftAssign),
            TokenKind::GreaterThanGreaterThanEqual => Ok(Operator::ShiftRightAssign),

            _ => Err(()),
        }
    }
}