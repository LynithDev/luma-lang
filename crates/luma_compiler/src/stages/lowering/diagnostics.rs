use luma_diagnostic::{define_contexts, define_diagnostics};

use crate::TypeKind;

define_diagnostics! {
    pub enum LoweringError {
        #[Error("missing scope id", "no scope id was found for the given node")]
        MissingScopeId,
        #[Error("missing symbol id", "no symbol id was found for the given node")]
        MissingSymbolId,
        #[Error("unknown type", "type could not be determined")]
        UnknownType,
        #[Error("mismatched nodes", "the expected node '{expected}' was not found, instead found '{found}'")]
        MismatchedNodes {
            expected: String,
            found: String,
        },
        #[Error("invalid type for int lowering", "the type '{found}' is not valid for integer lowering")]
        InvalidTypeForIntLowering {
            found: TypeKind,
        },
        #[Error("invalid type for float lowering", "the type '{found}' is not valid for float lowering")]
        InvalidTypeForFloatLowering {
            found: TypeKind,
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
