use luma_diagnostic::CompilerResult;

use crate::{OperatorKind, aast::*, bytecode::*, stages::codegen::chunk::{ChunkBuilderEnv, CodeChunk}};

pub struct ChunkBuilder;

impl ChunkBuilder {
    pub fn build(self, ast: &mut AnnotatedAst) -> CompilerResult<CodeChunk> {
        let mut ctx = ChunkBuilderEnv::default();

        dbg!(&ast);

        self.traverse(&mut ctx, ast)?;

        Ok(ctx.chunk)
    }
}

impl AnnotAstVisitor<'_> for ChunkBuilder {
    type Ctx = ChunkBuilderEnv;

    fn try_visit_stmt(&self, ctx: &mut Self::Ctx, stmt: &mut AnnotStmt) -> CompilerResult<()> {
        #[allow(unused)]
        match &stmt.item {
            AnnotStmtKind::Expr(expr) => {},
            AnnotStmtKind::Func(func_decl) => todo!(),
            AnnotStmtKind::Return(ret_stmt) => todo!(),
            AnnotStmtKind::Struct(struct_decl) => todo!(),
            AnnotStmtKind::Var(var_decl) => {
                let slot = ctx.declare_local(var_decl.symbol.id)?;

                ctx.emit(Opcode::SetLocal(slot));
            },
        }

        Ok(())
    }

    fn try_visit_expr(&self, ctx: &mut Self::Ctx, expr: &mut AnnotExpr) -> CompilerResult<()> {
        #[allow(unused)]
        match &mut expr.item {
            AnnotExprKind::Assign(assign_expr) => todo!(),
            AnnotExprKind::Binary(binary_expr) => {
                ctx.emit(match binary_expr.operator.kind {
                    OperatorKind::Add => Opcode::Add,
                    _ => todo!()
                });
            },
            AnnotExprKind::Block(block_expr) => todo!(),
            AnnotExprKind::Call(call_expr) => todo!(),
            AnnotExprKind::Get(get_expr) => todo!(),
            AnnotExprKind::Group(expr) => todo!(),
            AnnotExprKind::Ident(ident_expr) => {
                let slot = ctx.resolve_local_slot(&ident_expr.symbol.id)?;
                ctx.emit(Opcode::GetLocal(slot));
            },
            AnnotExprKind::If(if_expr) => todo!(),
            AnnotExprKind::Literal(literal_expr) => {
                let bytecode_value = build_literal_expr(literal_expr.clone());
                let const_index = ctx.add_constant(bytecode_value)?;

                ctx.emit(Opcode::LoadConst(const_index));
            },
            AnnotExprKind::Struct(struct_expr) => todo!(),
            AnnotExprKind::TupleLiteral(tuple_expr) => todo!(),
            AnnotExprKind::Unary(unary_expr) => todo!(),
        }

        Ok(())
    }

}

pub fn build_literal_expr(lit: LiteralAnnotExpr) -> BytecodeValue {
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


