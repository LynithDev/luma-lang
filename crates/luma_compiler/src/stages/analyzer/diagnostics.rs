use crate::{ScopeId, TypeKind, ast::LiteralExpr};
use luma_diagnostic::{define_contexts, define_diagnostics};

define_diagnostics! {
    pub enum AnalyzerError {
        #[Error("unresolved identifier", "the identifier '{identifier}' could not be resolved in the current scope")]
        UnresolvedIdentifier {
            identifier: String,
        },
        #[Error("unidentified symbol", "the symbol '{name}' could not be identified")]
        UnidentifiedSymbol {
            name: String,
        },
        #[Error("unresolved named type", "the type '{name}' could not be resolved in the current scope")]
        UnresolvedType {
            name: String,
        },
        #[Error("unresolved struct field", "struct '{struct_name}' has no field named '{field_name}'")]
        UnresolvedStructField {
            struct_name: String,
            field_name: String,
        },
        #[Error("type inference could not infer the type")]
        TypeInferenceFailure,
        #[Error("type mismatch", "expected type '{expected}' but found '{found}'")]
        TypeMismatch {
            expected: TypeKind,
            found: TypeKind,
        },
        #[Error("literal type mismatch", "expected type '{expected}' but found '{literal}'")]
        LiteralTypeMismatch {
            literal: LiteralExpr,
            expected: TypeKind,
        },
    }
}

define_contexts! {
    pub enum AnalyzerErrorContext {
        #[Context("in scope depth {scope_id}")]
        ScopeContext {
            scope_id: ScopeId,
        },
        #[Context("in block expression")]
        BlockContext,
    }
}