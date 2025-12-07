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
    pub locals: HashMap<SymbolId, usize>,
    pub upvalues: HashMap<SymbolId, usize>,
    pub upvalue_descriptors: Vec<UpvalueDescriptor>,
    pub next_local_index: usize,
}

enum SymbolResolution {
    Local(usize),
    Upvalue(usize),
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
    constants_lookup: HashMap<BytecodeValue, usize>,

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
            constants_lookup: HashMap::new(),
            functions_chunk,
            env: ChunkBuilderEnvironment::new(),
            parent_env: outer_env,
            curr_cursor: Cursor::default(),
        }
    }

    // MARK: Add Const
    /// adds a constant to the chunk and returns its index
    pub fn add_const(&mut self, value: BytecodeValue) -> usize {
        if let Some(&index) = self.constants_lookup.get(&value) {
            return index;
        }

        self.chunk.constants.push(value.clone());
        let index = self.chunk.constants.len() - 1;
        self.constants_lookup.insert(value, index);
        index
    }

    // MARK: Add Local
    /// adds a local variable to the chunk's environment and returns its index
    pub fn add_local(&mut self, symbol_id: SymbolId) -> usize {
        let local_index = self.env.next_local_index;

        self.env
            .locals
            .insert(symbol_id, local_index);
        self.env.next_local_index += 1;
        self.chunk.local_count = self.env.next_local_index;

        local_index
    }

    // MARK: Add Upvalue
    /// adds an upvalue to the chunk's environment and returns its index
    pub fn add_upvalue(
        &mut self,
        symbol_id: SymbolId,
        is_local: bool,
        index: usize,
    ) -> usize {
        let upvalue_index = self.env.upvalues.len();
        self.env
            .upvalues
            .insert(symbol_id, upvalue_index);

        self.env
            .upvalue_descriptors
            .push(UpvalueDescriptor { is_local, index });

        upvalue_index
    }

    // MARK: Resolve Symbol
    /// resolves a symbol id to either a local or upvalue index. if the symbol is not found in the current environment,
    /// it will attempt to capture it from the parent environment (if provided)
    fn resolve_symbol(
        &mut self,
        symbol_id: SymbolId,
        parent_env: Option<&ChunkBuilderEnvironment>,
    ) -> CodegenResult<Option<SymbolResolution>> {
        Ok(
            if let Some(&local_index) = self.env.locals.get(&symbol_id) {
                Some(SymbolResolution::Local(local_index))
            } else if let Some(&upvalue_index) = self.env.upvalues.get(&symbol_id) {
                Some(SymbolResolution::Upvalue(upvalue_index))
            } else if let Some(parent_env) = parent_env {
                let parent_resolution = self.capture_upvalue(symbol_id, parent_env)?;
                Some(SymbolResolution::Upvalue(parent_resolution))
            } else {
                None
            },
        )
    }

    // MARK: Capture Upvalue
    /// captures an upvalue from the parent environment and adds it to the current environment
    fn capture_upvalue(
        &mut self,
        symbol_id: SymbolId,
        parent_env: &ChunkBuilderEnvironment,
    ) -> CodegenResult<usize> {
        if let Some(&local_index) = parent_env.locals.get(&symbol_id) {
            return Ok(self.add_upvalue(symbol_id, true, local_index));
        }

        if let Some(&upvalue_index) = parent_env.upvalues.get(&symbol_id) {
            return Ok(self.add_upvalue(symbol_id, false, upvalue_index));
        }

        Err(CodegenDiagnostic::UnableToCaptureUpvalue(symbol_id))
    }

    // MARK: Emit Opcode
    /// emits an opcode to the chunk and returns its index
    pub fn emit_opcode(&mut self, opcode: OpCode) -> usize {
        let instruction = Instruction::new(opcode, self.curr_cursor);
        let idx = self.chunk.instructions.len();
        self.chunk.instructions.push(instruction);
        idx
    }

    // MARK: Patch Opcode
    /// patches an opcode at the given index
    pub fn patch_opcode(&mut self, index: usize, opcode: OpCode) {
        if let Some(instruction) = self.chunk.instructions.get_mut(index) {
            instruction.opcode = opcode;
        }
    }

    // MARK: -- Statement --
    pub fn gen_statement(&mut self, statement: &HirStatement) -> CodegenResult<()> {
        self.curr_cursor = statement.cursor;

        match &statement.kind {
            HirStatementKind::VarDecl(decl) => self.gen_var_decl(decl),
            HirStatementKind::Expression { inner } => self.gen_expr_stmt(inner),
            HirStatementKind::FuncDecl(decl) => self.gen_func_decl(decl),
            HirStatementKind::Return { value } => self.gen_return_stmt(value),

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
        let mut builder =
            ChunkBuilder::with_outer(&mut chunk, self.functions_chunk, Some(&self.env));

        if let Some(body) = &decl.body {
            for param in &decl.parameters {
                builder.add_local(param.symbol_id);
            }

            builder.gen_expression(body)?;
        } else {
            todo!("impl interface / abstract function");
        }

        let func_chunk = FunctionChunk {
            name: None,
            arity: decl.parameters.len() as Arity,
            kind: FunctionKind::Function, // todo
            upvalues: builder.env.upvalue_descriptors,
            chunk,
        };

        // push function chunk
        let func_index = self.functions_chunk.len();
        self.functions_chunk.push(func_chunk);

        // push function as constant (for lookup)
        let const_index = self.add_const(BytecodeValue::Function(func_index));
        self.emit_opcode(OpCode::Closure(const_index, Some(local_index)));

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

    // MARK: Return Stmt
    pub fn gen_return_stmt(&mut self, value: &Option<Box<HirExpression>>) -> CodegenResult<()> {
        if let Some(value) = value {
            self.gen_expression(value)?;
            self.emit_opcode(OpCode::Return);
        } else {
            self.emit_opcode(OpCode::ReturnUnit);
        }


        Ok(())
    }

    // MARK: Expr Stmt
    pub fn gen_expr_stmt(&mut self, expr: &HirExpression) -> CodegenResult<()> {
        self.gen_expression(expr)?;

        // Pop the result of the expression off the stack
        if expr.ty != TypeKind::Unit {
            self.emit_opcode(OpCode::Pop);
        }

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

            HirExpressionKind::If {
                main_expr,
                branches,
                else_expr,
            } => self.gen_if_expression(main_expr, branches, else_expr),

            _ => todo!(
                "expression kind '{}' not implemented",
                &expression.kind.to_string()
            ),
        }
    }

    // MARK: Literal
    pub fn gen_literal(&mut self, literal: &HirLiteralKind) -> CodegenResult<()> {
        let value = literal_to_value(literal);
        let const_index = self.add_const(value);

        let opcode = OpCode::Const(const_index);
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

        self.emit_opcode(OpCode::Call(arguments.len() as Arity));

        Ok(())
    }

    // MARK: If
    pub fn gen_if_expression(
        &mut self,
        main_expr: &HirConditionalBranch,
        branches: &[HirConditionalBranch],
        else_expr: &Option<Box<HirExpression>>,
    ) -> CodegenResult<()> {
        let mut jump_instructions: Vec<usize> = Vec::new();
        let mut branch_positions: Vec<usize> = Vec::new();

        // main condition
        self.gen_expression(&main_expr.condition)?;
        jump_instructions.push(self.emit_opcode(OpCode::JumpIfFalse(0)));
        
        self.gen_expression(&main_expr.body)?;

        // branch conditions

        macro_rules! branch_start {
            ($self:ident) => {
                let emit_jump = self.chunk.instructions.last().is_some_and(|instr| {
                    !instr.opcode.is_return()
                });

                if emit_jump {
                    let instr_index = self.emit_opcode(OpCode::Jump(0));
                    jump_instructions.push(instr_index);
                }

                branch_positions.push($self.chunk.instructions.len());
            };
        }

        // else branch
        if let Some(else_expr) = else_expr {
            branch_start!(self);

            self.gen_expression(else_expr)?;
        }

        // patch jumps
        let len = self.chunk.instructions.len();
        for (index, jump_target) in jump_instructions.iter().enumerate() {
            let instr = self.chunk.instructions.get_mut(*jump_target).unwrap();
            
            match instr.opcode {
                OpCode::JumpIfFalse(_) => {
                    let target_pos = branch_positions.get(index).cloned().unwrap_or(len);
                    instr.opcode = OpCode::JumpIfFalse(target_pos);
                },
                OpCode::Jump(_) => {
                    let target_pos = len - 1;
                    instr.opcode = OpCode::Jump(target_pos);
                },
                _ => unreachable!(),
            }
        }

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
    pub fn gen_assign(&mut self, symbol_id: &SymbolId, value: &HirExpression) -> CodegenResult<()> {
        self.gen_expression(value)?;
        
        let set_opcode = match self.resolve_symbol(*symbol_id, self.parent_env)? {
            Some(SymbolResolution::Local(index_ref)) => {
                OpCode::SetLocal(index_ref)
            }
            Some(SymbolResolution::Upvalue(index_ref)) => {
                OpCode::SetUpvalue(index_ref)
            }
            None => {
                return Err(CodegenDiagnostic::UnableToCaptureUpvalue(*symbol_id));
            }
        };

        self.emit_opcode(OpCode::Dup);
        self.emit_opcode(set_opcode);
        
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

        if locals > 0 {
            self.emit_opcode(OpCode::PopMul(locals));
        }

        Ok(())
    }
}
