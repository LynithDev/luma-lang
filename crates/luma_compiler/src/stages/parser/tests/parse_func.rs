use crate::{Type, TypeKind, Visibility, VisibilityKind, ast::*, stages::parser::tests::parse_ast};
use luma_core::Span;
use pretty_assertions::assert_eq;

#[test]
fn var_with_type_and_value() {
    let src = r#"
        
        func test(a: u32, b: f32, c: bool = false): (u32, f32, bool) {
            var x = a + 10;
            (x, b, c)
        };

    "#;

    let ast = parse_ast(src);

    assert_eq!(
        ast,
        Ast::new(
            Span::ZERO,
            vec![Stmt::new(
                Span::ZERO,
                StmtKind::Func(FuncDeclStmt {
                    visibility: Visibility::unspanned(VisibilityKind::Private),
                    symbol: Symbol::new(
                        Span::ZERO,
                        SymbolKind::named(String::from("test")),
                    ),

                    // (a: u32, b: f32, c: bool = false)
                    parameters: vec![
                        FuncParam {
                            symbol: Symbol::new(
                                Span::ZERO,
                                SymbolKind::named(String::from("a")),
                            ),
                            ty: Type::spanned(Span::ZERO, TypeKind::UInt32),
                            default_value: None,
                            span: Span::ZERO,
                            scope_id: None,
                        },
                        FuncParam {
                            symbol: Symbol::new(
                                Span::ZERO,
                                SymbolKind::named(String::from("b")),
                            ),
                            ty: Type::spanned(Span::ZERO, TypeKind::Float32),
                            default_value: None,
                            span: Span::ZERO,
                            scope_id: None,
                        },
                        FuncParam {
                            symbol: Symbol::new(
                                Span::ZERO,
                                SymbolKind::named(String::from("c")),
                            ),
                            ty: Type::spanned(Span::ZERO, TypeKind::Bool),
                            default_value: Some(Expr::new(
                                Span::ZERO,
                                ExprKind::Literal(LiteralExpr::Bool(false)),
                            )),
                            span: Span::ZERO,
                            scope_id: None,
                        },
                    ],
                    return_type: Some(Type::spanned(
                        // (u32, f32, bool)
                        Span::ZERO,
                        TypeKind::Tuple(vec![
                            Type::spanned(Span::ZERO, TypeKind::UInt32),
                            Type::spanned(Span::ZERO, TypeKind::Float32),
                            Type::spanned(Span::ZERO, TypeKind::Bool),
                        ]),
                    )),
                    body: Expr::new(
                        Span::ZERO,
                        ExprKind::Block(BlockExpr {
                            statements: vec![
                                // var x = a + 10;
                                Stmt::new(
                                    Span::ZERO,
                                    StmtKind::Var(VarDeclStmt {
                                        symbol: Symbol::new(
                                            Span::ZERO,
                                            SymbolKind::named(String::from("x")),
                                        ),
                                        ty: None,
                                        initializer: Expr::new(
                                            Span::ZERO,
                                            ExprKind::Binary(BinaryExpr {
                                                left: Box::new(Expr::new(
                                                    Span::ZERO,
                                                    ExprKind::Ident(IdentExpr {
                                                        symbol: SymbolKind::named(String::from(
                                                            "a"
                                                        ))
                                                    }),
                                                )),
                                                operator: Operator::new(
                                                    Span::ZERO,
                                                    OperatorKind::Add,
                                                ),
                                                right: Box::new(Expr::new(
                                                    Span::ZERO,
                                                    ExprKind::Literal(LiteralExpr::Int(10)),
                                                )),
                                            }),
                                        ),
                                        visibility: Visibility::unspanned(VisibilityKind::Private),
                                    }),
                                ),
                                ],
                                tail_expr: Some(Box::new(Expr::new(
                                    Span::ZERO,
                                    ExprKind::TupleLiteral(TupleExpr {
                                        elements: vec![
                                            Expr::new(
                                                Span::ZERO,
                                                ExprKind::Ident(IdentExpr {
                                                    symbol: SymbolKind::named(String::from(
                                                        "x"
                                                    ))
                                                }),
                                            ),
                                            Expr::new(
                                                Span::ZERO,
                                                ExprKind::Ident(IdentExpr {
                                                    symbol: SymbolKind::named(String::from(
                                                        "b"
                                                    ))
                                                }),
                                            ),
                                            Expr::new(
                                                Span::ZERO,
                                                ExprKind::Ident(IdentExpr {
                                                    symbol: SymbolKind::named(String::from(
                                                        "c"
                                                    ))
                                                }),
                                            ),
                                        ]
                                    }),
                                ))),
                        }),
                    ),
                }),
            )],
        )
    );
}
