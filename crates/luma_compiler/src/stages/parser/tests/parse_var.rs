use crate::{Type, TypeKind, Visibility, VisibilityKind, ast::*, stages::parser::tests::parse_ast};
use luma_core::Span;
use pretty_assertions::assert_eq;

#[test]
fn var_with_type_and_value() {
    let src = r#"
        var x: u32 = 42;
        var y = 2.5;
    "#;

    let ast = parse_ast(src);

    assert_eq!(
        ast,
        Ast::new(
            Span::ZERO,
            vec![
                Stmt::new(
                    Span::ZERO,
                    StmtKind::Var(VarDeclStmt {
                        symbol: Symbol::new(Span::ZERO, SymbolKind::named("x".to_string())),
                        ty: Some(Type::spanned(Span::ZERO, TypeKind::UInt32)),
                        initializer: Expr::new(
                            Span::ZERO,
                            ExprKind::Literal(LiteralExpr::Int(42)),
                        ),
                        visibility: Visibility::unspanned(VisibilityKind::default()),
                    }),
                ),
                Stmt::new(
                    Span::ZERO,
                    StmtKind::Var(VarDeclStmt {
                        symbol: Symbol::new(Span::ZERO, SymbolKind::named("y".to_string())),
                        ty: None,
                        initializer: Expr::new(
                            Span::ZERO,
                            ExprKind::Literal(LiteralExpr::Float(2.5)),
                        ),
                        visibility: Visibility::unspanned(VisibilityKind::default()),
                    }),
                ),
            ],
        )
    );
}

#[test]
fn pub_var_visibility() {
    let src = r#"
        pub(module) var a = 5;
    "#;

    let ast = parse_ast(src);
    
    assert_eq!(
        ast,
        Ast::new(
            Span::ZERO,
            vec![
                Stmt::new(
                    Span::ZERO,
                    StmtKind::Var(VarDeclStmt {
                        symbol: Symbol::new(Span::ZERO, SymbolKind::named("a".to_string())),
                        ty: None,
                        initializer: Expr::new(
                            Span::ZERO,
                            ExprKind::Literal(LiteralExpr::Int(5)),
                        ),
                        visibility: Visibility::spanned(
                            Span::ZERO,
                            VisibilityKind::Module,
                        ),
                    }),
                ),
            ],
        )
    );
}