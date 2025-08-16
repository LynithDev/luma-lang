use crate::tokens::Token;

#[derive(Debug, Default)]
pub struct TokenStream {
    tokens: Vec<Token>,
    pos: usize,
}

impl TokenStream {
    #[must_use]
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            pos: 0,
        }
    }

    #[must_use]
    pub fn previous(&self) -> &Token {
        if self.pos == 0 {
            panic!("tried to access previous token, but stream is empty. should not happen");
        } else if self.is_at_end() {
            self.tokens.last().unwrap()
        } else {
            &self.tokens[self.pos - 1]
        }
    }

    #[must_use]
    pub fn current(&self) -> &Token {
        if self.is_at_end() {
            self.previous()
        } else {
            &self.tokens[self.pos]
        }
    }

    #[must_use]
    pub fn lookahead_by(&self, amount: usize) -> &Token {
        if self.pos + amount >= self.tokens.len() {
            self.tokens.last().unwrap()
        } else {
            &self.tokens[self.pos + amount]
        }
    }

    pub fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.pos += 1;
        }

        self.previous()
    }

    #[must_use]
    pub fn is_at_end(&self) -> bool {
        self.pos >= self.tokens.len()
    }
}

impl From<Vec<Token>> for TokenStream {
    fn from(value: Vec<Token>) -> Self {
        Self::new(value)
    }
}
