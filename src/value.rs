use std::fmt::Display;

pub use crate::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Add(Box<Value>, Box<Value>),
    Sub(Box<Value>, Box<Value>),
    Reference(String),
    Dereference(String),
    Variable(String),
    Int(String), // Store numerals as strings because we are directly compiling into AMD64
    StringLiteral(String),
    FunctionCall(String, Vec<Value>),
    Null,
}

impl Value {
    pub fn codegen(&self, compiler: &mut Compiler, ty: &OperandType) -> ValueCodegen {
        match self {
            Value::Reference(ref name) => {
                let variable = compiler
                    .scope_manager
                    .get_variable_manager()
                    .get(name)
                    .expect("Variable {name} does not exist.");
                compiler.new_instruction(Instruction::LoadAddress(
                    Register::AX.as_gen(&Size::QuadWord),
                    variable.0.as_gen(&variable.1.size()),
                ));
                Register::AX.as_gen(&Size::QuadWord)
            }
            Value::Dereference(ref name) => {
                let variable = compiler
                    .scope_manager
                    .get_variable_manager()
                    .get(name)
                    .expect("Variable {name} does not exist.");
                compiler.new_instruction(Instruction::Move(
                    Register::AX.as_gen(&Size::QuadWord),
                    variable.0.as_ptr(),
                ));
                let deref_size = variable.1.deref_size().expect("Not a pointer");
                compiler.new_instruction(Instruction::Move(
                    Register::AX.as_gen(&deref_size),
                    Register::AX.as_deref(&deref_size),
                ));
                Register::AX.as_gen(&deref_size)
            }
            Value::Variable(ref name) => {
                let variable = compiler
                    .scope_manager
                    .get_variable_manager()
                    .get(name)
                    .expect("Variable {name} does not exist.");
                variable.0.as_gen(&variable.1.size())
            }
            Value::Int(num) => ValueCodegen::Number(num.clone()),
            Value::StringLiteral(literal) => ValueCodegen::StringLiteral(literal.clone()),
            Value::FunctionCall(name, parameters) => {
                function_call(name, parameters, compiler);

                ValueCodegen::Register(Register::AX.as_dword())
            }
            Value::Add(lhs, rhs) => {
                // TODO: Make Dynamic Sizing
                let lhs = lhs.codegen(compiler, ty);
                let rhs = rhs.codegen(compiler, ty);
                let dst = Register::AX.as_gen(&ty.size());
                compiler.new_instruction(Instruction::Move(dst.clone(), lhs));
                compiler.new_instruction(Instruction::Add(dst.clone(), rhs));
                dst
            }
            Value::Sub(lhs, rhs) => {
                // TODO: Make Dynamic Sizing
                let lhs = lhs.codegen(compiler, ty);
                let rhs = rhs.codegen(compiler, ty);
                let dst = Register::AX.as_gen(&ty.size());
                compiler.new_instruction(Instruction::Move(dst.clone(), lhs));
                compiler.new_instruction(Instruction::Sub(dst.clone(), rhs));
                dst
            }
            Value::Null => panic!(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum ValueCodegen {
    Register(String),
    StackOffset(String),
    Pointer(String),
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
        matches!(self, ValueCodegen::Register(_))
    }

    pub fn is_stack(&self) -> bool {
        matches!(self, ValueCodegen::StackOffset(_))
    }

    pub fn inner(&self) -> String {
        match self {
            ValueCodegen::Register(s)
            | ValueCodegen::StackOffset(s)
            | ValueCodegen::Pointer(s)
            | ValueCodegen::Number(s)
            | ValueCodegen::StringLiteral(s) => s.clone(),
        }
    }
}
