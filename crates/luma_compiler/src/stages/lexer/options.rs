use crate::options::define_options;

define_options! {
    pub struct LexerOptions {
        zeroed_spans: bool = false,
    }
}