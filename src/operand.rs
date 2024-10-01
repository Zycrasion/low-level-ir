use crate::{Compiler, IRStatement, Instruction, Register, Size, Value, ValueCodegen};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum OperandType {
    Undefined,
    Int(Size),
    UInt(Size)
}

impl OperandType
{
    pub fn size(&self) -> Size
    {
        match self
        {
            OperandType::Undefined => Size::Byte,
            OperandType::Int(size) |
            OperandType::UInt(size) => *size,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Operand {
    Move,
    DropVariable,
    FunctionDecl,
    Multiply,
    IntMultiply,
    Add,
    Subtract,
    Divide,
    IntDivide,
    Return,
    InlineAssembly
}

impl Operand {
    pub fn ir(&self, ty : OperandType, lhs : Value, rhs : Option<Value>) -> IRStatement
    {
        IRStatement
        {
            op_type : ty,
            operand : *self,
            lhs,
            rhs
        }
    }

    pub fn codegen(
        &self,
        lhs: &Value,
        rhs: &Option<Value>,
        _ty: &OperandType,
        compiler: &mut Compiler,
    ) {
        let lhs_original = lhs.clone();
        let lhs = lhs.codegen(compiler);
        let size = _ty.size();

        match self {
            Operand::InlineAssembly =>
            {
                compiler.new_instruction(Instruction::AsmLiteral(lhs.inner()));
            }
            Operand::Move => {
                let rhs = rhs.as_ref().unwrap().codegen(compiler);

                if lhs.is_stack() && rhs.is_stack()
                {
                    compiler.new_instruction(Instruction::Move(Register::AX.as_gen(&size), rhs));
                    compiler.new_instruction(Instruction::Move(lhs, Register::AX.as_gen(&size)));
                    return;
                }

                compiler.new_instruction(Instruction::Move(lhs, rhs))
            }
            Operand::FunctionDecl => {
                compiler.new_instruction(Instruction::Label(lhs));
                compiler.new_instruction(Instruction::Push(Register::BP.as_gen(&Size::QuadWord)));
                compiler.new_stack_frame();
            },
            Operand::Multiply | Operand::IntMultiply => {
                let rhs = rhs.as_ref().unwrap();
                match _ty {
                    OperandType::Undefined => todo!(),
                    OperandType::Int(size) | OperandType::UInt(size) => {
                        let lhs = lhs;
                        let rhs = rhs.codegen(compiler);    

                        if lhs.is_stack() && rhs.is_stack()
                        {
                            compiler.new_instruction(Instruction::Move(Register::AX.as_gen(size), lhs.clone()));
                            compiler.new_instruction(match self {Self::IntMultiply => Instruction::IntMultiply(Register::AX.as_gen(size), rhs), _ => Instruction::Multiply(Register::AX.as_gen(size), rhs)});
                            compiler.new_instruction(Instruction::Move(lhs, Register::AX.as_gen(size)));
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

                compiler.new_instruction(Instruction::Pop(Register::BP.as_gen(&Size::QuadWord)));
                compiler.new_instruction(Instruction::Return);
                compiler.pop_stack_frame()
            }
            Operand::DropVariable =>
            {
                // This variable is no longer used anywhere
                if let Value::VariableReference(name) = lhs_original
                {
                    compiler.dealloc_variable(&name)
                } else
                {
                    panic!()
                }
            }
            Operand::Add => {
                let rhs = rhs.clone().unwrap().codegen(compiler);
                compiler.new_instruction(Instruction::Add(lhs, rhs))
            }
            Operand::Subtract => {
                let rhs = rhs.clone().unwrap().codegen(compiler);
                compiler.new_instruction(Instruction::Sub(lhs, rhs))
            },
            Operand::Divide => todo!(),
            Operand::IntDivide => todo!(),
        }
    }
}