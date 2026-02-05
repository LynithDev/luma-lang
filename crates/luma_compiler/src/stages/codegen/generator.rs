use luma_diagnostic::CompilerResult;

use crate::{aast::*, bytecode::*, stages::codegen::ctx::BytecodeGenCtx};

pub struct BytecodeGen;

impl BytecodeGen {
    pub fn generate(ast: AnnotatedAst) -> CompilerResult<Bytecode> {
        let mut ctx = BytecodeGenCtx::default();
        let mut ast = ast;

        Self.traverse(&mut ctx, &mut ast);

        Ok(Bytecode {
            instructions: ctx.instructions,
        })
    }
}

impl AnnotAstVisitor<'_> for BytecodeGen {
    type Ctx = BytecodeGenCtx;

    fn visit_stmt(&self, ctx: &mut Self::Ctx, stmt: &mut AnnotStmt) {
        match &stmt.item {
            AnnotStmtKind::Var(var_decl) => {
                let local_id = ctx.declare_local(var_decl.symbol.id);
                ctx.emit(Opcode::SetLocal(local_id));
            }
            _ => {}
        }
    }

    fn visit_expr(&self, ctx: &mut Self::Ctx, expr: &mut AnnotExpr) {
        match &expr.item {
            AnnotExprKind::Assign(assign_expr) => todo!(),
            AnnotExprKind::Binary(binary_expr) => todo!(),
            AnnotExprKind::Block(block_expr) => todo!(),
            AnnotExprKind::Call(call_expr) => todo!(),
            AnnotExprKind::Get(get_expr) => todo!(),
            AnnotExprKind::Group(expr) => todo!(),
            AnnotExprKind::Ident(ident_expr) => todo!(),
            AnnotExprKind::If(if_expr) => todo!(),
            AnnotExprKind::Literal(lit) => {
                let value = match lit.clone() {
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
                    LiteralAnnotExpr::Bool(value) => BytecodeValue::Bool(value),
                    LiteralAnnotExpr::Float(value) => match value {
                        FloatLiteralAnnotExpr::Float32(value) => BytecodeValue::Float32(value),
                        FloatLiteralAnnotExpr::Float64(value) => BytecodeValue::Float64(value),
                    },
                    LiteralAnnotExpr::String(value) => BytecodeValue::String(value.clone()),
                    LiteralAnnotExpr::Char(value) => BytecodeValue::Char(value),
                    LiteralAnnotExpr::Unit => {
                        return;
                    }
                };

                let const_id = ctx.declare_constant(value);

                ctx.emit(Opcode::LoadConst(const_id));
            }
            AnnotExprKind::Struct(struct_expr) => todo!(),
            AnnotExprKind::TupleLiteral(tuple_expr) => todo!(),
            AnnotExprKind::Unary(unary_expr) => todo!(),
        }
    }
}
