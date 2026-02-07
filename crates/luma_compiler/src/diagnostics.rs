use luma_core::CodeSourceId;
use luma_diagnostic::define_diagnostics;

define_diagnostics! {
    pub enum CompilerError {
        // general errors
        #[Error("invalid source id", "the source id '{id:?}' does not exist in the source manager")]
        InvalidSourceId {
            id: CodeSourceId,
        },
    }
}

