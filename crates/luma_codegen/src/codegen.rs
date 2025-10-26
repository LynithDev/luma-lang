use luma_core::{Cursor, SymbolId, bytecode::prelude::*};
use luma_diagnostic::DiagnosticResult;
use luma_semantics::hir::prelude::*;

// MARK: Literal to Value
pub fn literal_to_value(kind: &HirLiteralKind) -> BytecodeValue {
    match kind {
        HirLiteralKind::Boolean(value) => BytecodeValue::Boolean(*value),
        HirLiteralKind::String(value) => BytecodeValue::String(value.clone()),
        HirLiteralKind::Integer(HirLiteralIntegerKind::Int8(value)) => BytecodeValue::Int8(*value),
        HirLiteralKind::Integer(HirLiteralIntegerKind::Int16(value)) => {
            BytecodeValue::Int16(*value)
        }
        HirLiteralKind::Integer(HirLiteralIntegerKind::Int32(value)) => {
            BytecodeValue::Int32(*value)
        }
        HirLiteralKind::Integer(HirLiteralIntegerKind::Int64(value)) => {
            BytecodeValue::Int64(*value)
        }
        HirLiteralKind::Integer(HirLiteralIntegerKind::UInt8(value)) => {
            BytecodeValue::UInt8(*value)
        }
        HirLiteralKind::Integer(HirLiteralIntegerKind::UInt16(value)) => {
            BytecodeValue::UInt16(*value)
        }
        HirLiteralKind::Integer(HirLiteralIntegerKind::UInt32(value)) => {
            BytecodeValue::UInt32(*value)
        }
        HirLiteralKind::Integer(HirLiteralIntegerKind::UInt64(value)) => {
            BytecodeValue::UInt64(*value)
        }
        HirLiteralKind::Float(HirLiteralFloatKind::Float32(value)) => {
            BytecodeValue::Float32(Float32(*value))
        }
        HirLiteralKind::Float(HirLiteralFloatKind::Float64(value)) => {
            BytecodeValue::Float64(Float64(*value))
        }
    }
}

// MARK: Chunk Builder
pub struct ChunkBuilder<'a> {
    chunk: &'a mut Chunk,
    cursor: Cursor,
}

impl<'a> ChunkBuilder<'a> {
    pub fn new(chunk: &'a mut Chunk) -> Self {
        Self {
            chunk,
            cursor: Cursor::default(),
        }
    }

    fn emit_opcode(&mut self, opcode: OpCode) {
        let instruction = Instruction::new(opcode, self.cursor);
        self.chunk.emit_instr(instruction);
    }

    // MARK: -- Statement --
    pub fn gen_statement(&mut self, statement: &HirStatement) -> DiagnosticResult<()> {
        self.cursor = statement.cursor;

        match &statement.kind {
            HirStatementKind::VarDecl(decl) => self.gen_var_decl(decl),
            HirStatementKind::Expression { inner } => self.gen_expr_stmt(inner),
            _ => todo!(
                "statement kind '{}' not implemented",
                &statement.kind.to_string()
            ),
        }
    }

    // MARK: Var Decl
    pub fn gen_var_decl(&mut self, decl: &HirVarDecl) -> DiagnosticResult<()> {
        if let Some(value) = &decl.value {
            self.gen_expression(value)?;
        }

        self.emit_opcode(OpCode::SetLocal(decl.symbol_id));

        Ok(())
    }

    // MARK: Expr Stmt
    pub fn gen_expr_stmt(&mut self, expr: &HirExpression) -> DiagnosticResult<()> {
        self.gen_expression(expr)?;

        // Pop the result of the expression off the stack
        self.emit_opcode(OpCode::Pop);

        Ok(())
    }

    // MARK: -- Expression --
    pub fn gen_expression(&mut self, expression: &HirExpression) -> DiagnosticResult<()> {
        self.cursor = expression.cursor;

        match &expression.kind {
            HirExpressionKind::Literal { kind } => self.gen_literal(kind),
            HirExpressionKind::Group { inner } => self.gen_expression(inner),
            HirExpressionKind::Binary {
                left,
                right,
                operator,
            } => self.gen_binary(left, right, operator),

            HirExpressionKind::Variable { symbol_id } => self.gen_variable(*symbol_id),
            HirExpressionKind::Assign { symbol_id, value } => self.gen_assign(symbol_id, value),

            _ => todo!(
                "expression kind '{}' not implemented",
                &expression.kind.to_string()
            ),
        }
    }

    // MARK: Literal
    pub fn gen_literal(&mut self, literal: &HirLiteralKind) -> DiagnosticResult<()> {
        let value = literal_to_value(literal);
        let const_index = self.chunk.add_const(value);

        let opcode = OpCode::Const(const_index);
        self.emit_opcode(opcode);

        Ok(())
    }

    // MARK: Binary
    pub fn gen_binary(
        &mut self,
        left: &HirExpression,
        right: &HirExpression,
        operator: &BinaryOperator,
    ) -> DiagnosticResult<()> {
        self.gen_expression(left)?;
        self.gen_expression(right)?;

        let opcode = match operator {
            BinaryOperator::Add => OpCode::Add,
            BinaryOperator::Subtract => OpCode::Sub,
            BinaryOperator::Multiply => OpCode::Mul,
            BinaryOperator::Divide => OpCode::Div,
            _ => unreachable!(),
        };

        self.emit_opcode(opcode);

        Ok(())
    }

    pub fn gen_assign(
        &mut self,
        symbol_id: &SymbolId,
        value: &HirExpression,
    ) -> DiagnosticResult<()> {
        self.gen_expression(value)?;

        self.emit_opcode(OpCode::SetLocal(*symbol_id));

        Ok(())
    }

    // MARK: Variable
    pub fn gen_variable(&mut self, symbol_id: SymbolId) -> DiagnosticResult<()> {
        self.emit_opcode(OpCode::GetLocal(symbol_id));
        Ok(())
    }
}
