use crate::{Visibility, VisibilityKind, ast::*};
use luma_core::{MaybeSpanned, Span};
use pretty_assertions::assert_eq;

use crate::{Parser, create_tokens};

#[test]
fn var_with_type_and_value() {
    // code:
    // var x: u32 = 42;
    // var y = 2.5;

    let (ast, _) = Parser::parse(&create_tokens![
        Var,
        Ident => "x",
        Colon,
        Ident => "u32",
        Equal,
        IntLiteral => "42",
        Semicolon,

        Var,
        Ident => "y",
        Equal,
        FloatLiteral => "2.5",
        Semicolon,
    ]);

    assert_eq!(
        ast,
        Ast::new(
            Span::default(),
            vec![
                Stmt::spanned(
                    Span::default(),
                    StmtKind::Var(VarDeclStmt {
                        symbol: Symbol::spanned(Span::default(), SymbolKind::named("x".to_string())),
                        ty: Some(Type::spanned(Span::default(), TypeKind::UInt32)),
                        initializer: Expr::spanned(
                            Span::default(),
                            ExprKind::Literal(LiteralExpr::Int(42)),
                        ),
                        visibility: MaybeSpanned::unspanned(VisibilityKind::default()),
                    }),
                ),
                Stmt::spanned(
                    Span::default(),
                    StmtKind::Var(VarDeclStmt {
                        symbol: Symbol::spanned(Span::default(), SymbolKind::named("y".to_string())),
                        ty: None,
                        initializer: Expr::spanned(
                            Span::default(),
                            ExprKind::Literal(LiteralExpr::Float(2.5)),
                        ),
                        visibility: MaybeSpanned::unspanned(VisibilityKind::default()),
                    }),
                ),
            ],
        )
    );
}

#[test]
fn pub_var_visibility() {
    // code:
    // pub(module) var a = 5;
    
    let (ast, _) = Parser::parse(&create_tokens![
        Pub,
        LeftParen,
        Module,
        RightParen,
        Var,
        Ident => "a",
        Equal,
        IntLiteral => "5",
        Semicolon,
    ]);

    assert_eq!(
        ast,
        Ast::new(
            Span::default(),
            vec![
                Stmt::spanned(
                    Span::default(),
                    StmtKind::Var(VarDeclStmt {
                        symbol: Symbol::spanned(Span::default(), SymbolKind::named("a".to_string())),
                        ty: None,
                        initializer: Expr::spanned(
                            Span::default(),
                            ExprKind::Literal(LiteralExpr::Int(5)),
                        ),
                        visibility: Visibility::spanned(
                            Span::default(),
                            VisibilityKind::Module,
                        ),
                    }),
                ),
            ],
        )
    );
}