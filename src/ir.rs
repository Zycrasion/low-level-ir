use crate::{Compiler, Operand, OperandType, Size};

#[derive(Debug, Clone)]
pub enum Value {
    Variable(Size, String),
    Int(String), // Store numerals as strings because we are directly compiling into AMD64
    StringLiteral(String),
    Null,
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum ValueCodegen
{
    Register(String),
    StackOffset(String),
    Number(String),
    StringLiteral(String)
}

impl ValueCodegen
{
    pub fn is_register(&self) -> bool
    {
        if let ValueCodegen::Register(_) = self {true} else {false}
    }

    pub fn is_stack(&self) -> bool
    {
        if let ValueCodegen::StackOffset(_) = self {true} else {false}
    }

    pub fn inner(&self) -> String
    {
        match self
        {
            ValueCodegen::Register(s) |
            ValueCodegen::StackOffset(s) |
            ValueCodegen::Number(s) |
            ValueCodegen::StringLiteral(s) => s.clone(),
        }
    }
}

impl Value {
    pub fn codegen(&self, compiler: &mut Compiler) -> ValueCodegen {
        match self {
            Value::Variable(size, ref name) => compiler.get_or_allocate_variable(name, size),
            Value::Int(num) => ValueCodegen::Number(num.clone()),
            Value::StringLiteral(literal) => ValueCodegen::StringLiteral(literal.clone()),
            _ => panic!()
        }
    }

    pub fn is_variable(&self) -> bool {
        match self {
            Value::Variable(_, _) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct IRStatement {
    pub op_type: OperandType,
    pub operand: Operand,
    pub lhs: Value,
    pub rhs: Option<Value>,
}

impl IRStatement {
    pub fn codegen(&self, compiler: &mut Compiler) -> String {
        self.operand
            .codegen(&self.lhs, &self.rhs, &self.op_type, compiler)
    }
}

#[derive(Debug, Clone)]
pub struct IRModule {
    pub statements: Vec<IRStatement>,
}

impl IRModule {
    pub fn new() -> Self
    {
        Self
        {
            statements : vec![]
        }
    }
    
    pub fn compile(&self) -> String {
        let mut compiler = Compiler::new();

        let mut buffer = String::new();
        for statement in &self.statements {
            buffer.push_str(format!("{}\n", statement.codegen(&mut compiler)).as_str());
        }

        buffer
    }
}