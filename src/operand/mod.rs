mod functions;
pub use functions::*;

mod variables;
pub use variables::*;

use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum OperandType {
    Undefined,
    Int(Size),
    UInt(Size),
    Char,
    Pointer(Box<OperandType>),
}

impl OperandType {
    pub fn size(&self) -> Size {
        match self {
            OperandType::Undefined | OperandType::Char => Size::Byte,
            OperandType::Int(size) | OperandType::UInt(size) => *size,
            OperandType::Pointer(_) => Size::QuadWord,
        }
    }

    pub fn deref_size(&self) -> Option<Size> {
        match self {
            OperandType::Pointer(inner) => Some(inner.size()),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Operand {
    // Type-Explicit
    DeclareVariable(OperandType, String, Value),
    FunctionDecl(
        OperandType,
        String,
        Vec<Operand>,
        Vec<(String, OperandType)>,
    ),
    Multiply(OperandType, Value, Value),
    Add(OperandType, Value, Value),
    Subtract(OperandType, Value, Value),
    Divide(OperandType, Value, Value),

    // Type-Implicit
    SetValue(Value, Value),
    DropVariable(String),
    FunctionCall(String, Vec<Value>),
    Return(Value),
    InlineAssembly(String),
}

impl Operand {
    pub fn codegen(&self, compiler: &mut Compiler) {
        match self {
            Operand::DeclareVariable(ty, name, value) => {
                variable_declaration(ty, name, value, compiler);
            }
            Operand::SetValue(lhs, value) => {
                set_value(lhs, value, compiler);
            }
            Operand::InlineAssembly(asm) => {
                compiler.new_instruction(Instruction::AsmLiteral(asm.clone()));
            }
            Operand::FunctionCall(name, parameters) => {
                function_call(name, parameters, compiler);
            }
            Operand::FunctionDecl(return_type, name, operands, parameters) => {
                function_decl(return_type, name, operands, parameters, compiler);
            }
            Operand::Return(_) => {
                eprintln!("Return not paired with function.");
                panic!();
            }
            Operand::DropVariable(name) => {
                // This variable is no longer used anywhere
                compiler
                    .scope_manager
                    .get_variable_manager()
                    .deallocate(name);
            }
            Operand::Add(_, _, _) | Operand::Subtract(_, _, _) => {}
            Operand::Divide(_, _, _) => todo!(),
            Operand::Multiply(_, _, _) => todo!(),
        }
    }
}
