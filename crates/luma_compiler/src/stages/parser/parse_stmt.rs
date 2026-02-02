use crate::{Visibility, ast::*};
use luma_core::Spanned;
use luma_diagnostic::{CompilerResult, LumaError};

use crate::stages::{
    lexer::TokenKind,
    parser::{parse::TokenParser, error::ParserErrorKind},
};

impl TokenParser<'_> {
    pub fn parse_statement(&mut self, semi: Option<bool>) -> CompilerResult<Stmt> {
        let stmt = self.stmt_declaration()?;

        if semi.unwrap_or(true) {
            self.consume(TokenKind::Semicolon)?;
        }

        Ok(stmt)
    }

    // MARK: Declaration
    /// Parses a declaration statement
    pub(super) fn stmt_declaration(&mut self) -> CompilerResult<Stmt> {
        let visibility = self.parse_visibility()?;
        let current = self.current();

        match current.kind {
            TokenKind::Var => self.stmt_var_decl(visibility),
            TokenKind::Func => self.stmt_func_decl(visibility),
            TokenKind::Struct => self.stmt_struct_decl(visibility),

            _ => self.statement(),
        }
    }

    pub(super) fn stmt_var_decl(&mut self, visibility: Visibility) -> CompilerResult<Stmt> {
        // this is our 'var' token
        let var_token = self.consume(TokenKind::Var)?;
        let mut span = var_token.span;

        // our variable's identifier
        let ident_token = self.consume(TokenKind::Ident)?;
        span.merge(&ident_token.span);

        // check for explicit type annotation
        let ty: Option<Type> = if self.consume(TokenKind::Colon).is_ok() {
            let ty = self.parse_type()?;
            span.maybe_merge(&ty.span);

            Some(ty)
        } else {
            None
        };

        // parse initializer
        self.consume(TokenKind::Equal)?;

        let initializer = self.parse_expression()?;
        span.merge(&initializer.span);

        Ok(Stmt::spanned(
            span,
            StmtKind::Var(VarDeclStmt {
                visibility,
                symbol: ident_token.as_symbol(),
                ty,
                initializer,
            }),
        ))
    }

    // MARK: Function
    /// Parses a function declaration statement
    ///
    /// ```ignore
    /// pub func my_function(param1: u32, param2: f64) -> u32 {
    ///     // function body
    /// }
    /// ```
    ///
    /// `visibility` - The visibility of the function
    pub(super) fn stmt_func_decl(&mut self, visibility: Visibility) -> CompilerResult<Stmt> {
        let func_token = self.consume(TokenKind::Func)?;
        let mut span = func_token.span;

        // function name
        let ident_token = self.consume(TokenKind::Ident)?;
        span.merge(&ident_token.span);

        // parameters
        self.consume(TokenKind::LeftParen)?;
        let mut parameters = Vec::new();

        while self.assert(TokenKind::RightParen).is_err() {
            let mut param_span = self.current().span;

            let param_ident = self.consume(TokenKind::Ident)?;
            param_span.merge(&param_ident.span);

            // param type
            self.consume(TokenKind::Colon)?;

            let ty = self.parse_type()?;
            param_span.maybe_merge(&ty.span);

            // param value initializer
            let default_value = if self.consume(TokenKind::Equal).is_ok() {
                let expr = self.parse_expression()?;
                param_span.merge(&expr.span);
                Some(expr)
            } else {
                None
            };

            // register param
            parameters.push(Spanned::spanned(
                param_span,
                FuncParam {
                    symbol: param_ident.as_symbol(),
                    ty,
                    default_value,
                },
            ));

            // check for comma separation
            if !self.check(TokenKind::RightParen) {
                self.consume(TokenKind::Comma)?;
            }
        }

        self.consume(TokenKind::RightParen)?;

        // return type
        let return_type = if self.consume(TokenKind::Colon).is_ok() {
            let ty = self.parse_type()?;
            span.maybe_merge(&ty.span);

            Some(ty)
        } else {
            None
        };

        // function body
        let current = self.current();
        let body = match &current.kind {
            TokenKind::LeftBrace => {
                // we don't consume the left brace here because it will be consumed in expr_block

                let block = self.expr_block()?;
                span.merge(&block.span);
                block
            }
            TokenKind::Equal => {
                // we consume the '=' token because otherwise it will be parsed as an assignment
                self.consume(TokenKind::Equal)?;

                let expr = self.parse_expression()?;
                span.merge(&expr.span);
                expr
            }
            _ => {
                return Err(LumaError::new(
                    ParserErrorKind::MissingFunctionBody,
                    current.span,
                ));
            }
        };

        Ok(Stmt::spanned(
            span,
            StmtKind::Func(FuncDeclStmt {
                visibility,
                symbol: ident_token.as_symbol(),
                parameters,
                return_type,
                body,
            }),
        ))
    }

    // MARK: Struct
    /// Parses a struct declaration statement
    ///
    /// ```ignore
    /// struct MyStruct {
    ///    pub field1: u32;
    ///    field2: f64;
    /// }
    /// ```
    ///
    /// `visibility` - The visibility of the struct
    pub(super) fn stmt_struct_decl(&mut self, visibility: Visibility) -> CompilerResult<Stmt> {
        let struct_token = self.consume(TokenKind::Struct)?;
        let mut span = struct_token.span;

        // struct name
        let ident_token = self.consume(TokenKind::Ident)?;
        span.merge(&ident_token.span);

        // struct body
        self.consume(TokenKind::LeftBrace)?;
        let mut fields = Vec::new();

        while !self.check(TokenKind::RightBrace) {
            let field_visibility = self.parse_visibility()?;

            let field_ident = self.consume(TokenKind::Ident)?;
            let mut field_span = field_ident.span;

            self.consume(TokenKind::Colon)?;

            let field_type = self.parse_type()?;
            field_span.maybe_merge(&field_type.span);

            fields.push(Spanned::spanned(
                field_span,
                StructFieldDecl {
                    visibility: field_visibility,
                    symbol: field_ident.as_symbol(),
                    ty: field_type,
                },
            ));
            
            span.merge(&field_span);

            // comma
            if self.consume(TokenKind::Comma).is_err() {
                break;
            }
        }

        self.consume(TokenKind::RightBrace)?;

        Ok(Stmt::spanned(
            span,
            StmtKind::Struct(StructDeclStmt {
                visibility,
                symbol: ident_token.as_symbol(),
                fields,
            }),
        ))
    }

    // MARK: Statement
    /// Parses a statement (non-declaration)
    pub(super) fn statement(&mut self) -> CompilerResult<Stmt> {
        let current = self.current();

        match &current.kind {
            TokenKind::Return => self.stmt_return(),
            _ => self.stmt_expr(),
        }
    }

    // MARK: Return
    /// Parses a return statement
    pub(super) fn stmt_return(&mut self) -> CompilerResult<Stmt> {
        let return_token = self.consume(TokenKind::Return)?;
        let mut span = return_token.span;

        let value = if !self.check_next(TokenKind::Semicolon) {
            let expr = self.parse_expression()?;
            span.merge(&expr.span);
            Some(expr)
        } else {
            None
        };

        Ok(Stmt::spanned(
            span,
            StmtKind::Return(ReturnStmt { value }), 
        ))
    }

    // MARK: Expression
    /// Parses an expression statement
    pub(super) fn stmt_expr(&mut self) -> CompilerResult<Stmt> {
        let expr = self.parse_expression()?;

        Ok(Stmt::spanned(
            expr.span,
            StmtKind::Expr(expr), 
        ))
    }
}
