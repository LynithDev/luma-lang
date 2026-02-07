use luma_diagnostic::{define_contexts, define_diagnostics};

define_diagnostics! {
    pub enum LoweringError {
        #[Error("missing scope id", "no scope id was found for the given node")]
        MissingScopeId,
        #[Error("missing symbol id", "no symbol id was found for the given node")]
        MissingSymbolId,
        #[Error("unknown type", "type could not be determined")]
        UnknownType,
        #[Error("invalid node for int lowering", "the node '{found}' is not valid for integer lowering")]
        InvalidLiteralConversion {
            found: String,
        },
        #[Error("integer overflow", "the amount '{amount}' is too large to fit in '{target}'")]
        IntegerOverflow {
            amount: u64,
            target: String,
        },
        #[Error("float overflow", "the amount '{amount}' is too large to fit in '{target}'")]
        FloatOverflow {
            amount: f64,
            target: String,
        },
        #[Error("invalid cast", "the cast from '{from}' to '{to}' is not allowed")]
        InvalidCast {
            from: String,
            to: String,
        },
    }
}

define_contexts! {
    pub enum LoweringDiagnosticContext {
        #[Context("node defined here")]
        Node,
    }
}
