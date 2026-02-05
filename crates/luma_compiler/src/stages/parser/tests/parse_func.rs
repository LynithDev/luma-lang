use crate::{Operator, Type, TypeKind, VisibilityKind, ast::*};
use luma_core::{MaybeSpanned, Span, Spanned};
use pretty_assertions::assert_eq;

use crate::{ParserStage, create_tokens};

#[test]
fn var_with_type_and_value() {
    /*

    func test(a: u32, b: f32, c: bool = false): (u32, f32, bool) {
        var x = a + 10;
        (x, b, c)
    };

     */

    let (ast, _) = ParserStage::parse(&create_tokens![
        // func test(
        Func,
        Ident => "test",
        LeftParen,

        // a: u32,
        Ident => "a",
        Colon,
        Ident => "u32",
        Comma,

        // b: f32,
        Ident => "b",
        Colon,
        Ident => "f32",
        Comma,

        // c: bool = false
        Ident => "c",
        Colon,
        Ident => "bool",
        Equal,
        BoolLiteral => "false",

        // )
        RightParen,

        // : (u32, f32, bool)
        Colon,
        LeftParen,
        Ident => "u32",
        Comma,
        Ident => "f32",
        Comma,
        Ident => "bool",
        RightParen,

        // {
        LeftBrace,

        // var x = a + 10;
        Var,
        Ident => "x",
        Equal,
        Ident => "a",
        Plus,
        IntLiteral => "10",
        Semicolon,

        // (x, b, c)
        LeftParen,
        Ident => "x",
        Comma,
        Ident => "b",
        Comma,
        Ident => "c",
        RightParen,

        // };
        RightBrace,
        Semicolon,
    ]);

    assert_eq!(
        ast,
        Ast::new(
            Span::default(),
            vec![Stmt::spanned(
                Span::default(),
                StmtKind::Func(FuncDeclStmt {
                    visibility: MaybeSpanned::unspanned(VisibilityKind::Private),
                    symbol: Symbol::spanned(
                        Span::default(),
                        SymbolKind::named(String::from("test")),
                    ),

                    // (a: u32, b: f32, c: bool = false)
                    parameters: vec![
                        Spanned::spanned(
                            Span::default(),
                            FuncParam {
                                symbol: Symbol::spanned(
                                    Span::default(),
                                    SymbolKind::named(String::from("a")),
                                ),
                                ty: Type::new(Span::default(), TypeKind::UInt32),
                                default_value: None,
                            },
                        ),
                        Spanned::spanned(
                            Span::default(),
                            FuncParam {
                                symbol: Symbol::spanned(
                                    Span::default(),
                                    SymbolKind::named(String::from("b")),
                                ),
                                ty: Type::new(Span::default(), TypeKind::Float32),
                                default_value: None,
                            },
                        ),
                        Spanned::spanned(
                            Span::default(),
                            FuncParam {
                                symbol: Symbol::spanned(
                                    Span::default(),
                                    SymbolKind::named(String::from("c")),
                                ),
                                ty: Type::new(Span::default(), TypeKind::Bool),
                                default_value: Some(Expr::spanned(
                                    Span::default(),
                                    ExprKind::Literal(LiteralExpr::Bool(false)),
                                )),
                            },
                        ),
                    ],
                    return_type: Some(Type::new(
                        // (u32, f32, bool)
                        Span::default(),
                        TypeKind::Tuple(vec![
                            Type::new(Span::default(), TypeKind::UInt32),
                            Type::new(Span::default(), TypeKind::Float32),
                            Type::new(Span::default(), TypeKind::Bool),
                        ]),
                    )),
                    body: Expr::spanned(
                        Span::default(),
                        ExprKind::Block(BlockExpr {
                            statements: vec![
                                // var x = a + 10;
                                Stmt::spanned(
                                    Span::default(),
                                    StmtKind::Var(VarDeclStmt {
                                        symbol: Symbol::spanned(
                                            Span::default(),
                                            SymbolKind::named(String::from("x")),
                                        ),
                                        ty: None,
                                        initializer: Expr::spanned(
                                            Span::default(),
                                            ExprKind::Binary(BinaryExpr {
                                                left: Box::new(Expr::spanned(
                                                    Span::default(),
                                                    ExprKind::Ident(IdentExpr {
                                                        symbol: SymbolKind::named(String::from(
                                                            "a"
                                                        ))
                                                    }),
                                                )),
                                                operator: Spanned::spanned(
                                                    Span::default(),
                                                    Operator::Add,
                                                ),
                                                right: Box::new(Expr::spanned(
                                                    Span::default(),
                                                    ExprKind::Literal(LiteralExpr::Int(10)),
                                                )),
                                            }),
                                        ),
                                        visibility: MaybeSpanned::unspanned(
                                            VisibilityKind::default()
                                        ),
                                    }),
                                ),
                                // (x, b, c)
                                Stmt::spanned(
                                    Span::default(),
                                    StmtKind::Expr(Expr::spanned(
                                        Span::default(),
                                        ExprKind::TupleLiteral(TupleExpr {
                                            elements: vec![
                                                Expr::spanned(
                                                    Span::default(),
                                                    ExprKind::Ident(IdentExpr {
                                                        symbol: SymbolKind::named(String::from(
                                                            "x"
                                                        ))
                                                    }),
                                                ),
                                                Expr::spanned(
                                                    Span::default(),
                                                    ExprKind::Ident(IdentExpr {
                                                        symbol: SymbolKind::named(String::from(
                                                            "b"
                                                        ))
                                                    }),
                                                ),
                                                Expr::spanned(
                                                    Span::default(),
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
