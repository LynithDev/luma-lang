use luma_diagnostic::CompilerResult;

use crate::{
    OperatorKind,
    aast::*,
    bytecode::*,
    stages::codegen::{
        chunk::{ChunkBuilderEnv, CodeChunk},
        module::ModuleContext,
    },
};

pub struct ChunkBuilder;

impl ChunkBuilder {
    pub fn build(
        self,
        ctx: &ModuleContext,
        statements: &mut Vec<AnnotStmt>,
    ) -> CompilerResult<CodeChunk> {
        let mut env = ChunkBuilderEnv::new();

        println!("Building chunk for statements: {:#?}", statements);

        self.traverse(
            &mut ChunkAstVisitorCtx {
                module: ctx,
                env: &mut env,
            },
            statements,
        )?;

        Ok(env.chunk)
    }
}

pub struct ChunkAstVisitorCtx<'a> {
    pub module: &'a ModuleContext,
    pub env: &'a mut ChunkBuilderEnv,
}

impl<'a> AnnotAstVisitor<'a> for ChunkBuilder {
    type Ctx = ChunkAstVisitorCtx<'a>;

    fn try_leave_stmt(&self, ctx: &mut Self::Ctx, stmt: &mut AnnotStmt) -> CompilerResult<()> {
        #[allow(unused)]
        match &stmt.item {
            AnnotStmtKind::Expr(expr) => {
                ctx.env.emit(Opcode::Pop);
            }
            AnnotStmtKind::Func(func_decl) => {
                // let chunk = Self.build(ctx, &mut func_decl.body)?;
            }
            AnnotStmtKind::Return(ret_stmt) => todo!(),
            AnnotStmtKind::Struct(struct_decl) => todo!(),
            AnnotStmtKind::Var(var_decl) => {
                let slot = ctx.env.declare_local(var_decl.symbol.id)?;

                ctx.env.emit(Opcode::SetLocal(slot));
            }
        }

        Ok(())
    }

    fn try_leave_expr(&self, ctx: &mut Self::Ctx, expr: &mut AnnotExpr) -> CompilerResult<()> {
        #[allow(unused)]
        match &mut expr.item {
            AnnotExprKind::Assign(assign_expr) => todo!(),
            AnnotExprKind::Binary(binary_expr) => {
                ctx.env.emit(match binary_expr.operator.kind {
                    OperatorKind::Add => Opcode::Add,
                    _ => todo!(),
                });
            }
            AnnotExprKind::Block(block_expr) => {}
            AnnotExprKind::Call(call_expr) => todo!(),
            AnnotExprKind::Get(get_expr) => todo!(),
            AnnotExprKind::Group(expr) => todo!(),
            AnnotExprKind::Ident(ident_expr) => {
                let slot = ctx.env.resolve_local_slot(&ident_expr.symbol.id)?;
                ctx.env.emit(Opcode::GetLocal(slot));
            }
            AnnotExprKind::If(if_expr) => todo!(),
            AnnotExprKind::Literal(literal_expr) => {
                let bytecode_value = build_literal_expr(literal_expr.clone());
                let const_index = ctx.env.add_constant(bytecode_value)?;

                ctx.env.emit(Opcode::LoadConst(const_index));
            }
            AnnotExprKind::Struct(struct_expr) => todo!(),
            AnnotExprKind::TupleLiteral(tuple_expr) => todo!(),
            AnnotExprKind::Unary(unary_expr) => todo!(),
        }

        Ok(())
    }
}

fn build_literal_expr(lit: LiteralAnnotExpr) -> BytecodeValue {
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
        LiteralAnnotExpr::Unit => unreachable!("unit literals should not be ever be emitted"),
    }
}
