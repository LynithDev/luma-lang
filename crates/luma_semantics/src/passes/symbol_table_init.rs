use luma_core::ast::*;
use luma_diagnostic::LumaResult;

use crate::AnalyzerContext;

pub struct SymbolTableBuildingPass<'a, 'ctx> {
    ctx: &'a mut AnalyzerContext<'ctx>,
}

impl<'a, 'ctx> SymbolTableBuildingPass<'a, 'ctx> {
    pub fn run(ctx: &'a mut AnalyzerContext<'ctx>) -> LumaResult<()> {
        let mut this = Self {
            ctx
        };

        this.iter()
    }

    fn iter(&mut self) -> LumaResult<()> {
        for statement in self.ctx.input.ast.statements.iter() {
            self.analyze_statement(statement)?;
        }

        Ok(())
    }

    fn analyze_statement(&mut self, statement: &luma_core::ast::Statement) -> LumaResult<()> {
        match &statement.kind {
            StatementKind::VarDecl(decl) => {
                let ty = match &decl.ty {
                    Some(t) => t.kind.clone(),
                    None => {
                        if let Some(init) = &decl.value {
                            self.get_type_info(init)
                        } else {
                            TypeKind::Void
                        }
                    },
                };

                self.ctx.symbol_table.declare(decl.name.clone(), ty);
            },
            StatementKind::FuncDecl(decl) => {
                // we enter scope for the function parameters as they are scoped to the func body
                self.ctx.symbol_table.enter_scope();

                // collect parameter types
                let mut param_types: Vec<TypeKind> = Vec::with_capacity(decl.parameters.len());

                for param in &decl.parameters {
                    self.ctx.symbol_table.declare(param.name.clone(), param.ty.kind.clone());
                    param_types.push(param.ty.kind.clone());
                }

                // collect or infer return type
                let return_type: TypeKind = match &decl.return_type {
                    Some(t) => t.kind.clone(),
                    None => {
                        if let Some(body) = &decl.body {
                            if let ExpressionKind::Scope(statements) = &body.kind {
                                // we don't want to enter a new scope here
                                self.get_scope_type_info(statements)
                            } else {
                                self.get_type_info(body)
                            }
                        } else {
                            TypeKind::Void
                        }
                    },
                };

                self.ctx.symbol_table.leave_scope();

                let fn_type = TypeKind::Function { 
                    param_types, 
                    return_type: Box::new(return_type),
                };

                self.ctx.symbol_table.declare(decl.name.clone(), fn_type);
            },
            _ => {},
        }

        Ok(())
    }

    #[allow(clippy::only_used_in_recursion)]
    fn get_type_info(&mut self, expression: &luma_core::ast::Expression) -> TypeKind {
        match &expression.kind {
            ExpressionKind::Literal { kind, .. } => {
                match kind {
                    LiteralKind::Integer => TypeKind::UInt32,
                    LiteralKind::Decimal => TypeKind::Float32,
                    LiteralKind::String => TypeKind::String,
                    LiteralKind::Boolean => TypeKind::Boolean,
                }
            },

            ExpressionKind::Unary(op, expr) => {
                let expr_type = self.get_type_info(expr);

                match op {
                    UnaryOperator::Negative => expr_type.as_signed(),
                    UnaryOperator::Not => TypeKind::Boolean,
                    _ => expr_type,
                }
            },

            ExpressionKind::Binary(left, _, right) => {
                let left_type = self.get_type_info(left);
                let right_type = self.get_type_info(right);

                let lhs_prec = left_type.precedence();
                let rhs_prec = right_type.precedence();

                // if the left is of lower precedence (higher priority), choose that one
                // else always choose right
                if lhs_prec < rhs_prec {
                    left_type
                } else {
                    right_type
                }
            },

            ExpressionKind::Comparison(_, _, _) => {
                TypeKind::Boolean
            },

            ExpressionKind::ArrayGet(array, _) => {
                let array_type = self.get_type_info(array);

                if array_type.is_array() && let TypeKind::Array(elem_ty) = array_type {
                    *elem_ty
                } else {
                    TypeKind::Void
                }
            },

            ExpressionKind::Get { object, .. } => {
                let object_type = self.get_type_info(object);

                if let TypeKind::Object(_) = object_type {
                    // if let Some(field) = fields.get(property) {
                    //     return field.clone();
                    // }
                    panic!("Object field lookup not implemented yet");
                }

                TypeKind::Void
            },

            ExpressionKind::Group(expr) => self.get_type_info(expr),

            ExpressionKind::If { .. } => {
                todo!("If expression type inference not implemented yet");
            },

            ExpressionKind::Invoke { callee, arguments } => {
                let callee_type = self.get_type_info(callee);

                if let TypeKind::Function { param_types: params, return_type } = callee_type {
                    if params.len() != arguments.len() {
                        dbg!("function call argument count mismatch");
                    }

                    for (arg, param) in arguments.iter().zip(params) {
                        let arg_type = self.get_type_info(arg);
                        if arg_type != param {
                            dbg!("function call argument type mismatch");
                        }
                    }

                    return *return_type;
                }

                TypeKind::Void
            },

            ExpressionKind::Logical(_, _, _) => {
                TypeKind::Boolean
            },

            ExpressionKind::Variable(name) => {
                if let Some(symbol) = self.ctx.symbol_table.lookup(name) {
                    return symbol.ty.clone();
                }

                TypeKind::Void
            },

            ExpressionKind::Scope(statements) => {
                self.ctx.symbol_table.enter_scope();
                let ty = self.get_scope_type_info(statements);
                self.ctx.symbol_table.leave_scope();
                
                ty
            },

            _ => {
                dbg!("handle {} kind", &expression.kind);
                TypeKind::Void
            }
        }
    }

    pub fn get_scope_type_info(&mut self, statements: &Vec<Statement>) -> TypeKind {
        let mut found_type = TypeKind::Void;

        for stmt in statements {
            match self.analyze_statement(stmt) {
                Ok(_) => {},
                Err(err) => {
                    self.ctx.reporter.report(err);
                }
            };
            
            let expr = match &stmt.kind {
                StatementKind::Return(Some(expr)) => expr,
                StatementKind::Expression(expr) => expr,
                _ => continue,
            };

            found_type = self.get_type_info(expr);
            break;
        }

        found_type
    }

}