use crate::{CompilerContext, Operator, OperatorKind, Type, TypeKind, Visibility, VisibilityKind, ast::*, compiler::run_stage};
use luma_core::Span;
use pretty_assertions::assert_eq;

use crate::{LexerStage, ParserStage};

#[test]
fn var_with_type_and_value() {
    let src = r#"
        
        func test(a: u32, b: f32, c: bool = false): (u32, f32, bool) {
            var x = a + 10;
            (x, b, c)
        };

    "#;

    let mut ctx = CompilerContext::new();
    ctx.sources.add_source(src.into());

    let tokens = run_stage(&ctx, LexerStage, vec![0.into()]).expect("lexing failed");
    let asts = run_stage(&ctx, ParserStage, &tokens).expect("parsing failed");

    let ast = asts.into_iter().next().expect("expected at least one AST");

    assert_eq!(
        ast,
        Ast::new(
            Span::void(),
            vec![Stmt::new(
                Span::void(),
                StmtKind::Func(FuncDeclStmt {
                    visibility: Visibility::unspanned(VisibilityKind::Private),
                    symbol: Symbol::new(
                        Span::void(),
                        SymbolKind::named(String::from("test")),
                    ),

                    // (a: u32, b: f32, c: bool = false)
                    parameters: vec![
                        FuncParam {
                            symbol: Symbol::new(
                                Span::void(),
                                SymbolKind::named(String::from("a")),
                            ),
                            ty: Type::spanned(Span::void(), TypeKind::UInt32),
                            default_value: None,
                            span: Span::void(),
                        },
                        FuncParam {
                            symbol: Symbol::new(
                                Span::void(),
                                SymbolKind::named(String::from("b")),
                            ),
                            ty: Type::spanned(Span::void(), TypeKind::Float32),
                            default_value: None,
                            span: Span::void(),
                        },
                        FuncParam {
                            symbol: Symbol::new(
                                Span::void(),
                                SymbolKind::named(String::from("c")),
                            ),
                            ty: Type::spanned(Span::void(), TypeKind::Bool),
                            default_value: Some(Expr::new(
                                Span::void(),
                                ExprKind::Literal(LiteralExpr::Bool(false)),
                            )),
                            span: Span::void(),
                        },
                    ],
                    return_type: Some(Type::spanned(
                        // (u32, f32, bool)
                        Span::void(),
                        TypeKind::Tuple(vec![
                            Type::spanned(Span::void(), TypeKind::UInt32),
                            Type::spanned(Span::void(), TypeKind::Float32),
                            Type::spanned(Span::void(), TypeKind::Bool),
                        ]),
                    )),
                    body: Expr::new(
                        Span::void(),
                        ExprKind::Block(BlockExpr {
                            statements: vec![
                                // var x = a + 10;
                                Stmt::new(
                                    Span::void(),
                                    StmtKind::Var(VarDeclStmt {
                                        symbol: Symbol::new(
                                            Span::void(),
                                            SymbolKind::named(String::from("x")),
                                        ),
                                        ty: None,
                                        initializer: Expr::new(
                                            Span::void(),
                                            ExprKind::Binary(BinaryExpr {
                                                left: Box::new(Expr::new(
                                                    Span::void(),
                                                    ExprKind::Ident(IdentExpr {
                                                        symbol: SymbolKind::named(String::from(
                                                            "a"
                                                        ))
                                                    }),
                                                )),
                                                operator: Operator::new(
                                                    Span::void(),
                                                    OperatorKind::Add,
                                                ),
                                                right: Box::new(Expr::new(
                                                    Span::void(),
                                                    ExprKind::Literal(LiteralExpr::Int(10)),
                                                )),
                                            }),
                                        ),
                                        visibility: Visibility::unspanned(VisibilityKind::Private),
                                    }),
                                ),
                                // (x, b, c)
                                Stmt::new(
                                    Span::void(),
                                    StmtKind::Expr(Expr::new(
                                        Span::void(),
                                        ExprKind::TupleLiteral(TupleExpr {
                                            elements: vec![
                                                Expr::new(
                                                    Span::void(),
                                                    ExprKind::Ident(IdentExpr {
                                                        symbol: SymbolKind::named(String::from(
                                                            "x"
                                                        ))
                                                    }),
                                                ),
                                                Expr::new(
                                                    Span::void(),
                                                    ExprKind::Ident(IdentExpr {
                                                        symbol: SymbolKind::named(String::from(
                                                            "b"
                                                        ))
                                                    }),
                                                ),
                                                Expr::new(
                                                    Span::void(),
                                                    ExprKind::Ident(IdentExpr {
                                                        symbol: SymbolKind::named(String::from(
                                                            "c"
                                                        ))
                                                    }),
                                                ),
                                            ]
                                        }),
                                    )),
                                )
                            ]
                        }),
                    ),
                }),
            )],
        )
    );
}
