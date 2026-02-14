use luma_diagnostic::CompilerResult;

use crate::{
    OperatorKind,
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
    pub fn build(
        self,
        module: &mut ModuleContext,
        statements: &mut Vec<AnnotStmt>,
    ) -> CompilerResult<CodeChunk> {
        let mut env = ChunkBuilderEnv::new();

        for stmt in statements {
            self.compile_stmt(module, &mut env, stmt)?;
        }

        Ok(env.chunk)
    }

    fn build_function(
        &self,
        module: &mut ModuleContext,
        func_decl: &mut FuncDeclAnnotStmt,
    ) -> CompilerResult<FunctionChunk> {
        let mut env: ChunkBuilderEnv = ChunkBuilderEnv::new();

        self.compile_expr(module, &mut env, &mut func_decl.body)?;

        let has_return = env
            .chunk
            .instructions
            .last()
            .is_some_and(|instr| matches!(instr, Opcode::Return));

        // means its a void function, emit a Unit value to stack to replace the call stack slot, then return to indicate end of function
        if !has_return {
            let unit_const = module.constant_table.add_constant(BytecodeValue::Unit)?;

            env.emit(Opcode::LoadConst(unit_const));
            env.emit(Opcode::Return);
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
        stmt: &mut AnnotStmt,
    ) -> CompilerResult<()> {
        match &mut stmt.item {
            AnnotStmtKind::Expr(expr) => {
                self.compile_expr(module, env, expr)?;

                env.emit(Opcode::Pop);
            }
            AnnotStmtKind::Func(func_decl) => {
                let func_chunk = self.build_function(module, func_decl)?;

                let func_index = module.function_table.add_function(func_decl.symbol.id, func_chunk);

            }
            AnnotStmtKind::Return(ret_stmt) => todo!(),
            AnnotStmtKind::Struct(struct_decl) => todo!(),
            AnnotStmtKind::Var(var_decl) => {
                let slot = env.declare_local(var_decl.symbol.id)?;

                self.compile_expr(module, env, &mut var_decl.initializer)?;

                env.emit(Opcode::SetLocal(slot));
            }
        }

        Ok(())
    }

    fn compile_expr(
        &self,
        module: &mut ModuleContext,
        env: &mut ChunkBuilderEnv,
        expr: &mut AnnotExpr,
    ) -> CompilerResult<()> {
        match &mut expr.item {
            AnnotExprKind::Assign(assign_expr) => todo!(),
            AnnotExprKind::Binary(binary_expr) => {
                env.emit(match binary_expr.operator.kind {
                    OperatorKind::Add => Opcode::Add,
                    _ => todo!(),
                });
            }
            AnnotExprKind::Block(block_expr) => {
                for stmt in &mut block_expr.statements {
                    self.compile_stmt(module, env, stmt)?;
                }

                if let Some(expr) = &mut block_expr.tail_expr {
                    self.compile_expr(module, env, expr)?;
                }
            }
            AnnotExprKind::Call(call_expr) => todo!(),
            AnnotExprKind::Get(get_expr) => todo!(),
            AnnotExprKind::Group(expr) => {
                self.compile_expr(module, env, expr)?;
            }
            AnnotExprKind::Ident(ident_expr) => {
                let slot = env.resolve_local_slot(&ident_expr.symbol.id)?;

                env.emit(Opcode::GetLocal(slot));
            }
            AnnotExprKind::If(if_expr) => todo!(),
            AnnotExprKind::Literal(literal_expr) => {
                let bytecode_value = lit_to_value(literal_expr.clone());
                let const_index = module.constant_table.add_constant(bytecode_value)?;

                env.emit(Opcode::LoadConst(const_index));
            }
            AnnotExprKind::Struct(struct_expr) => todo!(),
            AnnotExprKind::TupleLiteral(tuple_expr) => todo!(),
            AnnotExprKind::Unary(unary_expr) => todo!(),
        }

        Ok(())
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
