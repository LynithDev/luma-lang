#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    Keyword(KeywordKind),
    Literal(LiteralKind),
    Punctuation(PunctuationKind),
    Operator(OperatorKind),
    Identifier,
    EndOfFile,
}

impl std::fmt::Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenKind::Keyword(kind) => write!(f, "{kind}"),
            TokenKind::Literal(kind) => write!(f, "{kind}"),
            TokenKind::Punctuation(kind) => write!(f, "{kind}"),
            TokenKind::Operator(kind) => write!(f, "{kind}"),
            TokenKind::Identifier => write!(f, "identifier"),
            TokenKind::EndOfFile => write!(f, "eof"),
        }
    }
}

impl TokenKind {
    pub fn as_operator(&self) -> Option<&OperatorKind> {
        if let TokenKind::Operator(kind) = self {
            Some(kind)
        } else {
            None
        }
    }
}

#[derive(luma_core::Display, Debug, Clone, Copy, PartialEq, Eq)]
#[display(case = "snake_case")]
pub enum KeywordKind {
    If,
    Else,
    While,
    For,
    Function,
    #[display("let")]
    Var,
    Mut,
    Return,
    Break,
    Continue,
    Import,
    Class,
    This,
    Base,
    #[display("pub")]
    Public,
}

impl std::str::FromStr for KeywordKind {
    type Err = ();

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(match value {
            "if" => KeywordKind::If,
            "else" => KeywordKind::Else,
            "while" => KeywordKind::While,
            "for" => KeywordKind::For,
            "fn" => KeywordKind::Function,
            "let" => KeywordKind::Var,
            "mut" => KeywordKind::Mut,
            "return" => KeywordKind::Return,
            "break" => KeywordKind::Break,
            "continue" => KeywordKind::Continue,
            "import" => KeywordKind::Import,
            "class" => KeywordKind::Class,
            "this" => KeywordKind::This,
            "base" => KeywordKind::Base,
            "pub" => KeywordKind::Public,
            _ => return Err(()),
        })
    }
}

#[derive(luma_core::Display, Debug, Clone, Copy, PartialEq, Eq)]
#[display(case = "snake_case")]
pub enum LiteralKind {
    Integer,
    Decimal,
    String,
    Boolean,
}

#[derive(luma_core::Display, Debug, Clone, Copy, PartialEq, Eq)]
#[display(case = "snake_case")]
pub enum PunctuationKind {
    /// Value: `,`
    Comma,
    /// Value: `;`
    Semicolon,
    /// Value: `:`
    Colon,
    /// Value: `.`
    Dot,
    /// Value: `?`
    QuestionMark,
    /// Value: `(`
    LeftParen,
    /// Value: `)`
    RightParen,
    /// Value: `{`
    LeftBrace,
    /// Value: `}`
    RightBrace,
    /// Value: `[`
    LeftBracket,
    /// Value: `]`
    RightBracket,
}

#[derive(luma_core::Display, Debug, Clone, Copy, PartialEq, Eq)]
#[display(case = "snake_case")]
pub enum OperatorKind {
    /// Value: `+`
    Plus,
    /// Value: `-`
    Minus,
    /// Value: `*`
    Asterisk,
    /// Value: `/`
    Slash,
    /// Value: `%`
    Percent,
    /// Value: `=`
    Equals,
    /// Value: `==`
    EqualsEquals,
    /// Value: `!=`
    NotEquals,
    /// Value: `>`
    GreaterThan,
    /// Value: `<`
    LessThan,
    /// Value: `>=`
    GreaterThanOrEqual,
    /// Value: `<=`
    LessThanOrEqual,
    /// Value: `&&`
    And,
    /// Value: `||`
    Or,
    /// Value: `!`
    Not,
    /// Value: `~`
    BitwiseNot,
    /// Value: `&`
    BitwiseAnd,
    /// Value: `|`
    BitwiseOr,
    /// Value: `^`
    BitwiseXor,
    /// Value: `<<`
    ShiftLeft,
    /// Value: `>>`
    ShiftRight,
    /// Value: `+=`
    PlusEquals,
    /// Value: `-=`
    MinusEquals,
    /// Value: `*=`
    AsteriskEquals,
    /// Value: `/=`
    SlashEquals,
    /// Value: `%=`
    PercentEquals,
    /// Value: `&&=`
    AndEquals,
    /// Value: `||=`
    OrEquals,
    /// Value: `&=`
    BitwiseAndEquals,
    /// Value: `|=`
    BitwiseOrEquals,
    /// Value: `^=`
    BitwiseXorEquals,
    /// Value: `<<=`
    ShiftLeftEquals,
    /// Value: `>>=`
    ShiftRightEquals,
}


