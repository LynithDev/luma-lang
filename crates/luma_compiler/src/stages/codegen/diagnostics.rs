use luma_diagnostic::define_diagnostics;

define_diagnostics! {
    pub enum CodegenError {
        #[Error("invalid instruction patch", "attempted to patch an instruction at an invalid position: {position}")]
        InvalidPatchPosition {
            position: usize,
        },
        #[Error("too many locals", "too many locals declared in a single chunk")]
        TooManyLocals,
        #[Error("too many constants", "too many constants in a single chunk")]
        TooManyConstants,
        #[Error("undefined local", "local with symbol id {symbol_id} was not found")]
        UndefinedLocal {
            symbol_id: usize,
        },
    }
}