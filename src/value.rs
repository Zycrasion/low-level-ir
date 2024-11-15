use std::fmt::Display;

pub use crate::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Add(Box<Value>, Box<Value>),
    Sub(Box<Value>, Box<Value>),
    Reference(String),
    Dereference(String),
    Variable(String),
    Char(char),
    Int(String), // Store numerals as strings because we are directly compiling into Assembly
    StringLiteral(String),
    FunctionCall(String, Vec<Value>),
    Null,
}

impl Value {
    const DEFAULT_SIZE : Size = Size::DoubleWord;

    pub fn size(&self, compiler: &mut Compiler) -> Size
    {
        self.estimate_size(compiler).unwrap_or(Self::DEFAULT_SIZE)
    }

    /// If it has a defined size, return it, else return None, prefer lhs as a size definer, otherwise use rhs
    fn estimate_size(&self, compiler: &mut Compiler) -> Option<Size>
    {
        match self
        {
            Value::Add(lhs, rhs) |
            Value::Sub(lhs, rhs) => lhs.estimate_size(compiler).or(rhs.estimate_size(compiler)),
            Value::Reference(var) |
            Value::Dereference(var) |
            Value::Variable(var) => compiler.scope_manager.get_variable_manager().get(&var).map(|v| v.1.size()),
            Value::FunctionCall(name, _) => compiler.scope_manager.get_function(name).map(|v| v.0.size()),
            Value::Null |
            Value::Char(_) |
            Value::Int(_) |
            Value::StringLiteral(_) => None,
        }
    }

    pub fn codegen_size(&self, compiler: &mut Compiler, size : &Size) -> ValueCodegen
    {
        self.m_codegen(compiler, Some(size))
    }

    pub fn codegen(&self, compiler: &mut Compiler) -> ValueCodegen
    {
        self.m_codegen(compiler, None)
    }

    pub fn codegen_lhs(&self, compiler: &mut Compiler) -> ValueCodegen
    {
        match self
        {
            Value::Variable(ref name) => {
                let variable = compiler
                    .scope_manager
                    .get_variable_manager()
                    .get(name)
                    .expect("Variable {name} does not exist.");
                variable.0.as_gen(&variable.1.size())
            },
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

                Register::AX.as_deref(&deref_size)
            },
            _ => {
                eprintln!("Can't be lhs operand");
                panic!()
            }
        }
    }

    fn m_codegen(&self, compiler: &mut Compiler, size : Option<&Size>) -> ValueCodegen {
        match self {
            Value::Char(c) => {
                ValueCodegen::StringLikeValue(format!("'{c}'"))
            }
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
                ValueCodegen::Register(Register::AX.as_size(&function_call(name, parameters, compiler)))
            }
            Value::Add(lhs, rhs) => {
                let size = size.cloned().unwrap_or(self.size(compiler));
                let lhs = lhs.m_codegen(compiler, Some(&size));
                let rhs = rhs.m_codegen(compiler, Some(&size));
                let dst = Register::AX.as_gen(&size);
                compiler.new_instruction(Instruction::Move(dst.clone(), lhs.clone()));
                compiler.new_instruction(Instruction::Add(dst.clone(), rhs));
                dst
            }
            Value::Sub(lhs, rhs) => {
                let size = size.cloned().unwrap_or(self.size(compiler));
                let lhs = lhs.m_codegen(compiler, Some(&size));
                let rhs = rhs.m_codegen(compiler, Some(&size));
                let dst = Register::AX.as_gen(&size);
                compiler.new_instruction(Instruction::Move(dst.clone(), lhs.clone()));
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
    StringLikeValue(String),
    StringLiteral(String),
}

impl Display for ValueCodegen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.inner())
    }
}

impl ValueCodegen {
    pub fn is_immediate(&self) -> bool
    {
        matches!(self, ValueCodegen::Number(_))
    }

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
            | ValueCodegen::StringLikeValue(s)
            | ValueCodegen::StringLiteral(s) => s.clone(),
        }
    }
}
