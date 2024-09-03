use crate::{Compiler, Instruction, Register, Size, Value, ValueCodegen};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum OperandType {
    Undefined,
    Int(Size),
}

#[derive(Debug, Clone, Copy)]
pub enum Operand {
    Move,
    Label,
    Multiply,
    IntMultiply,
    Add,
    Subtract,
    Divide,
    IntDivide,
    Return
}

impl Operand {
    pub fn codegen(
        &self,
        lhs: &Value,
        rhs: &Option<Value>,
        _ty: &OperandType,
        compiler: &mut Compiler,
    ) {
        let lhs = lhs.codegen(compiler);
        match self {
            Operand::Move => {
                let rhs = rhs.as_ref().unwrap().codegen(compiler);

                if lhs.is_stack() && rhs.is_stack()
                {
                    compiler.new_instruction(Instruction::Move(ValueCodegen::Register(Register::AX.as_dword()), rhs));
                    compiler.new_instruction(Instruction::Move(lhs, ValueCodegen::Register(Register::AX.as_dword())));
                    return;
                }

                compiler.new_instruction(Instruction::Move(lhs, rhs))
            }
            Operand::Label => {
                compiler.new_instruction(Instruction::Label(lhs));
                compiler.new_instruction(Instruction::Push(ValueCodegen::Register(Register::BP.as_qword())));
            },
            Operand::Multiply | Operand::IntMultiply => {
                let rhs = rhs.as_ref().unwrap();
                match _ty {
                    OperandType::Undefined => todo!(),
                    OperandType::Int(size) => {
                        let lhs = lhs;
                        let rhs = rhs.codegen(compiler);    

                        if lhs.is_stack() && rhs.is_stack()
                        {
                            compiler.new_instruction(Instruction::Move(ValueCodegen::Register(Register::AX.as_dword()), lhs.clone()));
                            compiler.new_instruction(match self {Self::IntMultiply => Instruction::IntMultiply(ValueCodegen::Register(Register::AX.as_dword()), rhs), _ => Instruction::Multiply(ValueCodegen::Register(Register::AX.as_dword()), rhs)});
                            compiler.new_instruction(Instruction::Move(lhs, ValueCodegen::Register(Register::AX.as_dword())));
                            return;
                        }

                        compiler.new_instruction(match self {Self::IntMultiply => Instruction::IntMultiply(lhs, rhs), _ => Instruction::Multiply(lhs, rhs)});
                    }
                }
            }
            Operand::Return => {
                if let OperandType::Int(size) = _ty.clone()
                {
                    compiler.new_instruction(Instruction::Move(ValueCodegen::Register(Register::AX.as_size(&size)), lhs));
                }

                compiler.new_instruction(Instruction::Pop(ValueCodegen::Register(Register::BP.as_qword())));
                compiler.new_instruction(Instruction::Return);
            }
            Operand::Add => todo!(),
            Operand::Subtract => todo!(),
            Operand::Divide => todo!(),
            Operand::IntDivide => todo!(),
        }
    }
}