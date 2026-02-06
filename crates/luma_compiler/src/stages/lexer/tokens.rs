use crate::{OperatorKind, ast::{Symbol, SymbolKind}};
use luma_core::Span;
use strum::Display;

pub type TokenList = Vec<Token>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub lexeme: String,
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    #[must_use]
    pub fn as_symbol(&self) -> Symbol {
        Symbol {
            span: self.span,
            kind: SymbolKind::named(self.lexeme.clone()),
        }
    }
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

impl TryFrom<TokenKind> for OperatorKind {
    type Error = ();

    fn try_from(value: TokenKind) -> Result<Self, Self::Error> {
        match value {
            // others
            TokenKind::Equal => Ok(OperatorKind::Assign),
            TokenKind::Bang => Ok(OperatorKind::Not),

            // arithmetic
            TokenKind::Plus => Ok(OperatorKind::Add),
            TokenKind::Minus => Ok(OperatorKind::Subtract),
            TokenKind::Asterisk => Ok(OperatorKind::Multiply),
            TokenKind::Slash => Ok(OperatorKind::Divide),
            TokenKind::Percent => Ok(OperatorKind::Modulo),

            TokenKind::PlusEqual => Ok(OperatorKind::AddAssign),
            TokenKind::MinusEqual => Ok(OperatorKind::SubtractAssign),
            TokenKind::AsteriskEqual => Ok(OperatorKind::MultiplyAssign),
            TokenKind::SlashEqual => Ok(OperatorKind::DivideAssign),
            TokenKind::PercentEqual => Ok(OperatorKind::ModuloAssign),

            // logic
            TokenKind::AmpersandAmpersand => Ok(OperatorKind::And),
            TokenKind::PipePipe => Ok(OperatorKind::Or),

            TokenKind::AmpersandAmpersandEqual => Ok(OperatorKind::AndAssign),
            TokenKind::PipePipeEqual => Ok(OperatorKind::OrAssign),

            // comparison
            TokenKind::EqualEqual => Ok(OperatorKind::Equal),
            TokenKind::BangEqual => Ok(OperatorKind::NotEqual),
            TokenKind::Less => Ok(OperatorKind::LessThan),
            TokenKind::Greater => Ok(OperatorKind::GreaterThan),
            TokenKind::LessEqual => Ok(OperatorKind::LessThanOrEqual),
            TokenKind::GreaterEqual => Ok(OperatorKind::GreaterThanOrEqual),

            // bitwise
            TokenKind::Ampersand => Ok(OperatorKind::BitwiseAnd),
            TokenKind::Pipe => Ok(OperatorKind::BitwiseOr),
            TokenKind::Caret => Ok(OperatorKind::BitwiseXor),
            TokenKind::LessThanLessThan => Ok(OperatorKind::ShiftLeft),
            TokenKind::GreaterThanGreaterThan => Ok(OperatorKind::ShiftRight),

            TokenKind::AmpersandEqual => Ok(OperatorKind::BitwiseAndAssign),
            TokenKind::PipeEqual => Ok(OperatorKind::BitwiseOrAssign),
            TokenKind::CaretEqual => Ok(OperatorKind::BitwiseXorAssign),
            TokenKind::LessThanLessThanEqual => Ok(OperatorKind::ShiftLeftAssign),
            TokenKind::GreaterThanGreaterThanEqual => Ok(OperatorKind::ShiftRightAssign),

            _ => Err(()),
        }
    }
}