#[derive(Debug, Clone)]
pub struct ParserContext {
    pub allow_struct_literal: bool,
}

impl Default for ParserContext {
    fn default() -> Self {
        Self {
            allow_struct_literal: true,
        }
    }
}