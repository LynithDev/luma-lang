use luma_diagnostic::CompilerResult;

use crate::{
    aast::*,
    bytecode::*,
    stages::codegen::{
        chunk::{ChunkBuilderEnv, CodeChunk, FunctionChunk},
        module::ModuleContext,
    },
};

pub struct ChunkBuilder;

#[allow(unused)]
impl ChunkBuilder {
    pub fn build_top_level(
        self,
        module: &mut ModuleContext,
        statements: &mut Vec<AnnotStmt>,
    ) -> CompilerResult<CodeChunk> {
        let mut env = ChunkBuilderEnv::new();

        for stmt in statements {
            self.compile_stmt(module, &mut env, stmt)?;
        }

        self.emit_unit(module, &mut env)?;
        env.chunk.emit(Opcode::Return);

        Ok(env.chunk)
    }

    pub fn build_function(
        &self,
        module: &mut ModuleContext,
        func_decl: &FuncDeclAnnotStmt,
    ) -> CompilerResult<FunctionChunk> {
        let mut env: ChunkBuilderEnv = ChunkBuilderEnv::new();

        self.compile_expr(module, &mut env, &func_decl.body, true)?;

        let has_return = env
            .chunk
            .last()
            .is_some_and(|instr| matches!(instr, Opcode::Return));

        // means its a void function, emit a Unit value to stack to replace the call stack slot, then return to indicate end of function
        if !has_return {
            self.emit_unit(module, &mut env)?;
            env.chunk.emit(Opcode::Return);
        }

        Ok(FunctionChunk {
            code: env.chunk,
            arity: func_decl.parameters.len(),
        })
    }

    fn compile_stmt(
        &self,
        module: &mut ModuleContext,
        env: &mut ChunkBuilderEnv,
        stmt: &AnnotStmt,
    ) -> CompilerResult<()> {
        match &stmt.item {
            AnnotStmtKind::Expr(expr) => self.compile_expr(module, env, expr, false)?,
            AnnotStmtKind::Func(func_decl) => {
                let func_chunk = self.build_function(module, func_decl)?;

                let func_index = module
                    .function_table
                    .add_function(func_decl.symbol.id, func_chunk);
            }
            AnnotStmtKind::Return(ret_stmt) => {
                if let Some(expr) = &ret_stmt.value {
                    self.compile_expr(module, env, expr, true)?;
                } else {
                    self.emit_unit(module, env)?;
                }

                env.chunk.emit(Opcode::Return);
            },
            AnnotStmtKind::Struct(struct_decl) => todo!(),
            AnnotStmtKind::Var(var_decl) => {
                let slot = env.declare_local(var_decl.symbol.id)?;

                self.compile_expr(module, env, &var_decl.initializer, true)?;

                env.chunk.emit(Opcode::SetLocal(slot));
            }
        }

        Ok(())
    }

    fn compile_expr(
        &self,
        module: &mut ModuleContext,
        env: &mut ChunkBuilderEnv,
        expr: &AnnotExpr,
        value_used: bool,
    ) -> CompilerResult<()> {
        match &expr.item {
            AnnotExprKind::Assign(assign_expr) => {
                self.compile_expr(module, env, &assign_expr.value, true)?;
                if let Some(operator) = &assign_expr.operator {
                    // env.chunk.emit()
                    todo!()
                } else {
                    // simple assign (no special operator like +=, -=, etc.)
                    match &assign_expr.target.item {
                        AnnotExprKind::Ident(ident) => {
                            let slot = env.resolve_local_slot(&ident.symbol.id)?;

                            env.chunk.emit(Opcode::SetLocal(slot));

                            if value_used {
                                env.chunk.emit(Opcode::GetLocal(slot));
                            }
                        }
                        _ => todo!(),
                    };
                }
            }
            AnnotExprKind::Binary(binary_expr) => {
                self.compile_expr(module, env, &binary_expr.left, true)?;
                self.compile_expr(module, env, &binary_expr.right, true)?;

                let opcode = operator_to_opcode(binary_expr.operator.kind.clone());
                env.chunk.emit(opcode);

                if !value_used {
                    env.chunk.emit(Opcode::Pop);
                }
            }
            AnnotExprKind::Block(block_expr) => {
                for stmt in &block_expr.statements {
                    self.compile_stmt(module, env, stmt)?;
                }

                if let Some(expr) = &block_expr.tail_expr {
                    self.compile_expr(module, env, expr, value_used)?;
                } else if value_used {
                    // if there's no tail expression but the block's value is used, push a unit value to the stack
                    self.emit_unit(module, env)?;
                }
            }
            AnnotExprKind::Call(call_expr) => todo!(),
            AnnotExprKind::Get(get_expr) => todo!(),
            AnnotExprKind::Group(expr) => self.compile_expr(module, env, expr, value_used)?,
            AnnotExprKind::Ident(ident_expr) => {
                let slot = env.resolve_local_slot(&ident_expr.symbol.id)?;

                env.chunk.emit(Opcode::GetLocal(slot));

                if !value_used {
                    env.chunk.emit(Opcode::Pop);
                }
            }
            AnnotExprKind::If(if_expr) => {
                // condition
                self.compile_expr(module, env, &if_expr.condition, true)?;
                let jump_to_else = if if_expr.else_branch.is_some() {
                    Some(env.chunk.emit(Opcode::JumpIfFalse(0))?)
                } else {
                    None
                };

                // then branch
                let pre_max_locals = env.chunk.max_locals;
                let then_locals = {
                    self.compile_expr(module, env, &if_expr.then_branch, value_used)?;
                    env.chunk.max_locals - pre_max_locals
                };

                let jump_to_end = env.chunk.emit(Opcode::Jump(0))?;

                // else branch
                if let Some(else_branch) = &if_expr.else_branch {
                    let else_start = env.chunk.instr_len();
                    env.chunk
                        .patch(jump_to_else.unwrap(), Opcode::JumpIfFalse(else_start))?;

                    let else_locals = {
                        let pre_else_locals = env.chunk.max_locals;
                        self.compile_expr(module, env, else_branch, value_used)?;
                        env.chunk.max_locals - pre_else_locals
                    };

                    env.chunk.max_locals = pre_max_locals + then_locals.max(else_locals);
                } else {
                    env.chunk.max_locals = pre_max_locals + then_locals;
                }

                // end
                let end = env.chunk.instr_len();
                env.chunk.patch(jump_to_end, Opcode::Jump(end))?;
            }
            AnnotExprKind::Literal(literal_expr) => {
                let bytecode_value = lit_to_value(literal_expr.clone());

                if let BytecodeValue::Unit = bytecode_value {
                    env.chunk.emit(Opcode::PushUnit);
                } else {
                    let const_index = module.constant_table.add_constant(bytecode_value)?;
                    env.chunk.emit(Opcode::LoadConst(const_index));
                }


                if !value_used {
                    env.chunk.emit(Opcode::Pop);
                }
            }
            AnnotExprKind::Struct(struct_expr) => todo!(),
            AnnotExprKind::TupleLiteral(tuple_expr) => todo!(),
            AnnotExprKind::Unary(unary_expr) => {
                self.compile_expr(module, env, &unary_expr.value, true)?;

                let opcode = match unary_expr.operator.kind {
                    AnnotOperatorKind::Not => Opcode::Not,
                    AnnotOperatorKind::Subtract => Opcode::Negate,
                    _ => unreachable!(),
                };

                env.chunk.emit(opcode);

                if !value_used {
                    env.chunk.emit(Opcode::Pop);
                }
            }
        }

        Ok(())
    }

    fn emit_unit(
        &self,
        module: &mut ModuleContext,
        env: &mut ChunkBuilderEnv,
    ) -> CompilerResult<()> {
        env.chunk.emit(Opcode::PushUnit);

        Ok(())
    }
}

fn operator_to_opcode(op: AnnotOperatorKind) -> Opcode {
    match op {
        AnnotOperatorKind::Add => Opcode::Add,
        AnnotOperatorKind::Subtract => Opcode::Sub,
        AnnotOperatorKind::Multiply => Opcode::Mul,
        AnnotOperatorKind::Divide => Opcode::Div,
        AnnotOperatorKind::Modulo => Opcode::Mod,
        AnnotOperatorKind::BitwiseAnd => Opcode::BitAnd,
        AnnotOperatorKind::BitwiseOr => Opcode::BitOr,
        AnnotOperatorKind::BitwiseXor => Opcode::BitXor,
        AnnotOperatorKind::ShiftLeft => Opcode::ShiftLeft,
        AnnotOperatorKind::ShiftRight => Opcode::ShiftRight,
        AnnotOperatorKind::Equal => Opcode::Equal,
        AnnotOperatorKind::GreaterThan => Opcode::GreaterThan,
        AnnotOperatorKind::LessThan => Opcode::LesserThan,
        AnnotOperatorKind::GreaterThanOrEqual => Opcode::GreaterThanEqual,
        AnnotOperatorKind::LessThanOrEqual => Opcode::LesserThanEqual,
        AnnotOperatorKind::NotEqual => Opcode::NotEqual,
        AnnotOperatorKind::And => Opcode::And,
        AnnotOperatorKind::Or => Opcode::Or,
        AnnotOperatorKind::Not => Opcode::Not,
    }
}

fn lit_to_value(lit: LiteralAnnotExpr) -> BytecodeValue {
    match lit {
        LiteralAnnotExpr::Int(value) => match value {
            IntLiteralAnnotExpr::UInt8(value) => BytecodeValue::UInt8(value),
            IntLiteralAnnotExpr::UInt16(value) => BytecodeValue::UInt16(value),
            IntLiteralAnnotExpr::UInt32(value) => BytecodeValue::UInt32(value),
            IntLiteralAnnotExpr::UInt64(value) => BytecodeValue::UInt64(value),
            IntLiteralAnnotExpr::Int8(value) => BytecodeValue::Int8(value),
            IntLiteralAnnotExpr::Int16(value) => BytecodeValue::Int16(value),
            IntLiteralAnnotExpr::Int32(value) => BytecodeValue::Int32(value),
            IntLiteralAnnotExpr::Int64(value) => BytecodeValue::Int64(value),
        },
        LiteralAnnotExpr::Float(value) => match value {
            FloatLiteralAnnotExpr::Float32(value) => BytecodeValue::Float32(value),
            FloatLiteralAnnotExpr::Float64(value) => BytecodeValue::Float64(value),
        },
        LiteralAnnotExpr::Bool(value) => BytecodeValue::Bool(value),
        LiteralAnnotExpr::Char(value) => BytecodeValue::Char(value),
        LiteralAnnotExpr::String(value) => BytecodeValue::String(value),
        LiteralAnnotExpr::Unit => BytecodeValue::Unit,
    }
}
