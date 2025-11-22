use std::{collections::HashMap, rc::Rc};

use luma_core::{bytecode::prelude::*, Cursor, SymbolId};
use luma_semantics::hir::prelude::*;

use crate::diagnostics::CodegenDiagnostic;

// MARK: Literal to Value
pub fn literal_to_value(kind: &HirLiteralKind) -> BytecodeValue {
    match kind {
        HirLiteralKind::Boolean(value) => BytecodeValue::Boolean(*value),
        HirLiteralKind::String(value) => BytecodeValue::String(Rc::new(value.clone())),
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

type CodegenResult<T> = Result<T, CodegenDiagnostic>;

// MARK: Environment
#[derive(Debug, Clone)]
struct ChunkBuilderEnvironment {
    pub locals: HashMap<SymbolId, IndexRef>,
    pub upvalues: HashMap<SymbolId, IndexRef>,
    pub upvalue_descriptors: Vec<UpvalueDescriptor>,
    pub next_local_index: usize,
}

enum SymbolResolution {
    Local(IndexRef),
    Upvalue(IndexRef),
}

impl ChunkBuilderEnvironment {
    pub fn new() -> Self {
        Self {
            locals: HashMap::new(),
            upvalues: HashMap::new(),
            upvalue_descriptors: Vec::new(),
            next_local_index: 0,
        }
    }

}

// MARK: Chunk Builder
pub struct ChunkBuilder<'a> {
    functions_chunk: &'a mut Vec<FunctionChunk>,
    chunk: &'a mut Chunk,
    
    env: ChunkBuilderEnvironment,
    
    parent_env: Option<&'a ChunkBuilderEnvironment>,
    
    curr_cursor: Cursor,
}

impl<'a> ChunkBuilder<'a> {
    pub fn new(chunk: &'a mut Chunk, functions_chunk: &'a mut Vec<FunctionChunk>) -> Self {
        Self::with_outer(chunk, functions_chunk, None)
    }
    
    fn with_outer(
        chunk: &'a mut Chunk,
        functions_chunk: &'a mut Vec<FunctionChunk>,
        outer_env: Option<&'a ChunkBuilderEnvironment>,
    ) -> Self {
        Self {
            chunk,
            functions_chunk,
            env: ChunkBuilderEnvironment::new(),
            parent_env: outer_env,
            curr_cursor: Cursor::default(),
        }
    }
    
    pub fn add_local(&mut self, symbol_id: SymbolId) -> IndexRef {
        let local_index = self.env.next_local_index;
        
        self.env.locals.insert(symbol_id, IndexRef::new(local_index));
        self.env.next_local_index += 1;
        self.chunk.local_count = self.env.next_local_index;
        
        IndexRef::new(local_index)
    }
    
    pub fn add_upvalue(&mut self, symbol_id: SymbolId, is_local: bool, index: IndexRef) -> IndexRef {
        let upvalue_index = self.env.upvalues.len();
        self.env.upvalues.insert(symbol_id, IndexRef::new(upvalue_index));

        self.env.upvalue_descriptors.push(UpvalueDescriptor {
            is_local,
            index,
        });

        IndexRef::new(upvalue_index)
    }
    
    fn resolve_symbol(&mut self, symbol_id: SymbolId, parent_env: Option<&ChunkBuilderEnvironment>) -> CodegenResult<Option<SymbolResolution>> {
        Ok(if let Some(&local_index) = self.env.locals.get(&symbol_id) {
            Some(SymbolResolution::Local(local_index))
        } else if let Some(&upvalue_index) = self.env.upvalues.get(&symbol_id) {
            Some(SymbolResolution::Upvalue(upvalue_index))
        } else if let Some(parent_env) = parent_env {
            let parent_resolution = self.capture_upvalue(symbol_id, parent_env)?;
            Some(SymbolResolution::Upvalue(parent_resolution))
        } else {
            None
        })
    }
    
    fn capture_upvalue(
        &mut self,
        symbol_id: SymbolId,
        parent_env: &ChunkBuilderEnvironment
    ) -> CodegenResult<IndexRef> {

        if let Some(&local_index) = parent_env.locals.get(&symbol_id) {
            return Ok(self.add_upvalue(symbol_id, true, local_index));
        }

        if let Some(&upvalue_index) = parent_env.upvalues.get(&symbol_id) {
            return Ok(self.add_upvalue(symbol_id, false, upvalue_index));
        }

        Err(CodegenDiagnostic::UnableToCaptureUpvalue(symbol_id))
    }

    fn emit_opcode(&mut self, opcode: OpCode) {
        let instruction = Instruction::new(opcode, self.curr_cursor);
        self.chunk.emit_instr(instruction);
    }

    // MARK: -- Statement --
    pub fn gen_statement(&mut self, statement: &HirStatement) -> CodegenResult<()> {
        self.curr_cursor = statement.cursor;

        match &statement.kind {
            HirStatementKind::VarDecl(decl) => self.gen_var_decl(decl),
            HirStatementKind::Expression { inner } => self.gen_expr_stmt(inner),
            HirStatementKind::FuncDecl(decl) => self.gen_func_decl(decl),
            _ => todo!(
                "statement kind '{}' not implemented",
                &statement.kind.to_string()
            ),
        }
    }

    // MARK: Function Decl
    pub fn gen_func_decl(&mut self, decl: &HirFuncDecl) -> CodegenResult<()> {
        // reserve local slot for function (this allows recursion)
        let local_index = self.add_local(decl.symbol_id);

        let mut chunk = Chunk::new();
        let mut builder = ChunkBuilder::with_outer(&mut chunk, self.functions_chunk, Some(&self.env));
        
        if let Some(body) = &decl.body {
            builder.gen_expression(body)?;
            builder.emit_opcode(OpCode::Return);
        } else {
            todo!("impl interface / abstract function");
        }
        
        let func_chunk = FunctionChunk {
            name: None,
            arity: ArityRef::new(decl.parameters.len() as u8),
            kind: FunctionKind::Function, // todo
            upvalues: builder.env.upvalue_descriptors,
            chunk,
        };

        // push function chunk
        let func_index = self.functions_chunk.len();
        self.functions_chunk.push(func_chunk);
        
        // push function as constant (for lookup)
        let const_index = self.chunk.add_const(BytecodeValue::Function(IndexRef::new(func_index)));
        self.emit_opcode(OpCode::Const(IndexRef::new(const_index)));
        
        // store the function in the reserved local slot
        self.emit_opcode(OpCode::SetLocal(local_index));

        Ok(())
    }

    // MARK: Var Decl
    pub fn gen_var_decl(&mut self, decl: &HirVarDecl) -> CodegenResult<()> {
        if let Some(value) = &decl.value {
            self.gen_expression(value)?;
        }

        let local_index = self.add_local(decl.symbol_id);
        self.emit_opcode(OpCode::SetLocal(local_index));

        Ok(())
    }

    // MARK: Expr Stmt
    pub fn gen_expr_stmt(&mut self, expr: &HirExpression) -> CodegenResult<()> {
        self.gen_expression(expr)?;

        // Pop the result of the expression off the stack
        self.emit_opcode(OpCode::Pop);

        Ok(())
    }

    // MARK: -- Expression --
    pub fn gen_expression(&mut self, expression: &HirExpression) -> CodegenResult<()> {
        self.curr_cursor = expression.cursor;

        match &expression.kind {
            // syntax
            HirExpressionKind::Literal { kind } => self.gen_literal(kind),
            HirExpressionKind::Group { inner } => self.gen_expression(inner),

            // other
            HirExpressionKind::Variable { symbol_id } => self.gen_variable(*symbol_id),

            // operators
            HirExpressionKind::Assign { symbol_id, value } => self.gen_assign(symbol_id, value),
            HirExpressionKind::Unary { operator, value } => self.gen_unary(value, operator),
            HirExpressionKind::Binary {
                left,
                right,
                operator,
            } => self.gen_binary(left, right, operator),
            HirExpressionKind::Comparison {
                left,
                right,
                operator,
            } => self.gen_comparison(left, right, operator),
            HirExpressionKind::Logical {
                left,
                right,
                operator,
            } => self.gen_logical(left, right, operator),

            HirExpressionKind::Scope { statements, value } => self.gen_scope(statements, value),
            HirExpressionKind::Invoke { callee, arguments } => self.gen_invoke(callee, arguments),
            _ => todo!(
                "expression kind '{}' not implemented",
                &expression.kind.to_string()
            ),
        }
    }

    // MARK: Literal
    pub fn gen_literal(&mut self, literal: &HirLiteralKind) -> CodegenResult<()> {
        let value = literal_to_value(literal);
        let const_index = self.chunk.add_const(value);

        let opcode = OpCode::Const(IndexRef::new(const_index));
        self.emit_opcode(opcode);

        Ok(())
    }

    // MARK: Invoke
    pub fn gen_invoke(
        &mut self,
        callee: &HirExpression,
        arguments: &Vec<HirExpression>,
    ) -> CodegenResult<()> {
        // evaluate arguments first
        for argument in arguments {
            self.gen_expression(argument)?;
        }

        // then evaluate the callee
        self.gen_expression(callee)?;

        self.emit_opcode(OpCode::Call(ArityRef::new(arguments.len() as u8)));

        Ok(())
    }

    // MARK: Binary
    pub fn gen_binary(
        &mut self,
        left: &HirExpression,
        right: &HirExpression,
        operator: &BinaryOperator,
    ) -> CodegenResult<()> {
        self.gen_expression(left)?;
        self.gen_expression(right)?;

        let opcode = match operator {
            BinaryOperator::Add => OpCode::Add,
            BinaryOperator::Subtract => OpCode::Sub,
            BinaryOperator::Multiply => OpCode::Mul,
            BinaryOperator::Divide => OpCode::Div,
            BinaryOperator::Modulo => OpCode::Mod,
            BinaryOperator::BitwiseAnd => OpCode::BitAnd,
            BinaryOperator::BitwiseOr => OpCode::BitOr,
            BinaryOperator::BitwiseXor => OpCode::BitXor,
            BinaryOperator::ShiftLeft => OpCode::ShiftLeft,
            BinaryOperator::ShiftRight => OpCode::ShiftRight,
        };

        self.emit_opcode(opcode);

        Ok(())
    }

    // MARK: Comparison
    pub fn gen_comparison(
        &mut self,
        left: &HirExpression,
        right: &HirExpression,
        operator: &ComparisonOperator,
    ) -> CodegenResult<()> {
        self.gen_expression(left)?;
        self.gen_expression(right)?;

        let opcode = match operator {
            ComparisonOperator::Equals => OpCode::Equal,
            ComparisonOperator::GreaterThan => OpCode::GreaterThan,
            ComparisonOperator::LesserThan => OpCode::LesserThan,
            ComparisonOperator::GreaterThanEqual => OpCode::GreaterThanEqual,
            ComparisonOperator::LesserThanEqual => OpCode::LesserThanEqual,
            ComparisonOperator::NotEquals => OpCode::NotEqual,
        };

        self.emit_opcode(opcode);

        Ok(())
    }

    // MARK: Logical
    pub fn gen_logical(
        &mut self,
        left: &HirExpression,
        right: &HirExpression,
        operator: &LogicalOperator,
    ) -> CodegenResult<()> {
        self.gen_expression(left)?;
        self.gen_expression(right)?;

        let opcode = match operator {
            LogicalOperator::And => OpCode::And,
            LogicalOperator::Or => OpCode::Or,
        };

        self.emit_opcode(opcode);

        Ok(())
    }

    // MARK: Unary
    pub fn gen_unary(
        &mut self,
        value: &HirExpression,
        operator: &UnaryOperator,
    ) -> CodegenResult<()> {
        self.gen_expression(value)?;

        let opcode = match operator {
            UnaryOperator::Negate => OpCode::Negate,
            UnaryOperator::Not => OpCode::Not,
            UnaryOperator::BitwiseNot => OpCode::BitNot,
        };

        self.emit_opcode(opcode);

        Ok(())
    }

    // MARK: Assign
    pub fn gen_assign(
        &mut self,
        symbol_id: &SymbolId,
        value: &HirExpression,
    ) -> CodegenResult<()> {
        self.gen_expression(value)?;
        match self.resolve_symbol(*symbol_id, self.parent_env)? {
            Some(SymbolResolution::Local(index_ref)) => {
                self.emit_opcode(OpCode::SetLocal(index_ref));
            }
            Some(SymbolResolution::Upvalue(index_ref)) => {
                self.emit_opcode(OpCode::SetUpvalue(index_ref));
            }
            None => {
                return Err(CodegenDiagnostic::UnableToCaptureUpvalue(*symbol_id));
            }
        }

        Ok(())
    }

    // MARK: Variable
    pub fn gen_variable(&mut self, symbol_id: SymbolId) -> CodegenResult<()> {
        match self.resolve_symbol(symbol_id, self.parent_env)? {
            Some(SymbolResolution::Local(index_ref)) => {
                self.emit_opcode(OpCode::GetLocal(index_ref));
            }
            Some(SymbolResolution::Upvalue(index_ref)) => {
                self.emit_opcode(OpCode::GetUpvalue(index_ref));
            }
            None => {
                return Err(CodegenDiagnostic::UnableToCaptureUpvalue(symbol_id));
            }
        }

        Ok(())
    }

    // MARK: Scope
    pub fn gen_scope(
        &mut self,
        statements: &Vec<HirStatement>,
        value: &Option<Box<HirExpression>>,
    ) -> CodegenResult<()> {
        let mut locals: usize = 0;

        for statement in statements {
            self.gen_statement(statement)?;

            if let HirStatementKind::VarDecl(_) = &statement.kind {
                locals += 1;
            }
        }

        if let Some(value) = value {
            self.gen_expression(value)?;
        }

        self.emit_opcode(OpCode::PopLocals(locals));
        Ok(())
    }
}
