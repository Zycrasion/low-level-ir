use std::{env::var, fmt::Display};

use crate::{deallocation_pass, operand, Compiler, Instruction, Operand, OperandType, Register, Size};

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Variable(String),
    Int(String), // Store numerals as strings because we are directly compiling into AMD64
    StringLiteral(String),
    FunctionCall(String),
    Null,
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum ValueCodegen {
    Register(String),
    StackOffset(String),
    Number(String),
    StringLiteral(String),
}

impl Display for ValueCodegen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.inner())
    }
}

impl ValueCodegen {
    pub fn is_register(&self) -> bool {
        if let ValueCodegen::Register(_) = self {
            true
        } else {
            false
        }
    }

    pub fn is_stack(&self) -> bool {
        if let ValueCodegen::StackOffset(_) = self {
            true
        } else {
            false
        }
    }

    pub fn inner(&self) -> String {
        match self {
            ValueCodegen::Register(s)
            | ValueCodegen::StackOffset(s)
            | ValueCodegen::Number(s)
            | ValueCodegen::StringLiteral(s) => s.clone(),
        }
    }
}

impl Value {
    pub fn codegen(&self, compiler: &mut Compiler) -> ValueCodegen {
        match self {
            Value::Variable(ref name) => {
                let variable = compiler.scope_manager.get_variable_manager().get(name).expect("Variable {name} does not exist.");
                variable.0.as_gen(&variable.1.size())
            },
            Value::Int(num) => ValueCodegen::Number(num.clone()),
            Value::StringLiteral(literal) => ValueCodegen::StringLiteral(literal.clone()),
            Value::FunctionCall(name) => {
                compiler.new_instruction(Instruction::Call(name.clone()));
                ValueCodegen::Register(Register::AX.as_dword())
            }
            Value::Null => panic!(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct IRModule {
    pub operands: Vec<Operand>,
}

impl IRModule {
    pub fn new() -> Self {
        Self { operands: vec![] }
    }

    pub fn optimise(&mut self)
    {
        deallocation_pass(self);
    }

    pub fn compile(&self) -> String {
        let mut compiler = Compiler::new();

        // first we will scan through the code for functions,

        for operand in &self.operands {
            if let Operand::FunctionDecl(_type, name, _, parameters) = operand
            {
                // If its a function, add it to the function  declaration.
                compiler.scope_manager.declare_function_global(name, _type, &parameters.iter().cloned().map(|v| v.1).collect()).expect("Function {name} is already defined");
            }
        }


        let mut buffer = String::new();
        for operands in &self.operands {
            operands.codegen(&mut compiler);
        }

        for asm in &compiler.compiled
        {
            buffer.push_str(&asm.codegen_x86());
            buffer.push('\n')
        }

        buffer
    }
}
