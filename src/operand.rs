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

#[derive(Debug, Clone)]
pub enum Operand {
    Move(Value, Value),
    DropVariable(String),
    FunctionDecl(String),
    FunctionCall(String),
    Multiply(Value, Value),
    Add(Value, Value),
    Subtract(Value, Value),
    Divide(Value, Value),
    Return(Value),
    InlineAssembly(String),
}

impl Operand {
    pub fn get_values(&self) -> Vec<Value>
    {
        match self
        {
            Self::Move(a, b) | Self::Multiply(a, b) | Self::Add(a, b) | Self::Subtract(a, b) | Self::Divide(a, b) => vec![a.clone(), b.clone()],
            Self::Return(a) => vec![a.clone()],
            Self::DropVariable(_) | Self::FunctionDecl(_)  | Self::InlineAssembly(_) | Self::FunctionCall(_) => vec![]
        }
    }
    
    pub fn ir(&self, ty : OperandType) -> IRStatement
    {
        IRStatement
        {
            op_type : ty,
            operand : self.clone(),
        }
    }

    pub fn codegen(
        &self,
        _ty: &OperandType,
        compiler: &mut Compiler,
    ) {
        let size = _ty.size();

        match self {
            Operand::Move(lhs, rhs) => {
                let lhs = lhs.codegen(compiler);
                let rhs = rhs.codegen(compiler);
                
                if lhs.is_stack() && rhs.is_stack()
                {
                    compiler.new_instruction(Instruction::Move(Register::AX.as_gen(&size), rhs));
                    compiler.new_instruction(Instruction::Move(lhs, Register::AX.as_gen(&size)));
                    return;
                }
                
                compiler.new_instruction(Instruction::Move(lhs, rhs))
            }
            Operand::InlineAssembly(asm) =>
            {
                compiler.new_instruction(Instruction::AsmLiteral(asm.clone()));
            }
            Operand::FunctionCall(name) => {
                compiler.new_instruction(Instruction::Call(name.clone()));
            }
            Operand::FunctionDecl(name) => {
                compiler.new_instruction(Instruction::Label(name.clone()));
                compiler.new_instruction(Instruction::Push(Register::BP.as_gen(&Size::QuadWord)));
                compiler.new_stack_frame();
            },
            Operand::Multiply(lhs, rhs) => {
                let lhs = lhs.codegen(compiler);

                match _ty {
                    OperandType::Undefined => todo!(),
                    OperandType::Int(size) | OperandType::UInt(size) => {
                        let lhs = lhs;
                        let rhs = rhs.codegen(compiler);    

                        if lhs.is_stack() && rhs.is_stack()
                        {
                            compiler.new_instruction(Instruction::Move(Register::AX.as_gen(size), lhs.clone()));
                            if let OperandType::Int(size) = _ty
                            {
                                compiler.new_instruction(Instruction::IntMultiply(Register::AX.as_gen(size), rhs));
                            } else
                            {
                                compiler.new_instruction(Instruction::Multiply(Register::AX.as_gen(size), rhs));
                            }
                            compiler.new_instruction(Instruction::Move(lhs, Register::AX.as_gen(size)));
                            return;
                        }

                        if let OperandType::Int(size) = _ty
                        {
                            compiler.new_instruction(Instruction::IntMultiply(Register::AX.as_gen(size), rhs));
                        } else
                        {
                            compiler.new_instruction(Instruction::Multiply(Register::AX.as_gen(size), rhs));
                        }
                    }
                }
            }
            Operand::Return(value) => {
                if let OperandType::Int(size) = _ty.clone()
                {
                    let value = value.codegen(compiler);
                    compiler.new_instruction(Instruction::Move(ValueCodegen::Register(Register::AX.as_size(&size)), value));
                }

                compiler.new_instruction(Instruction::Pop(Register::BP.as_gen(&Size::QuadWord)));
                compiler.new_instruction(Instruction::Return);
                compiler.pop_stack_frame()
            }
            Operand::DropVariable(name) =>
            {
                // This variable is no longer used anywhere
                compiler.dealloc_variable(name);
            }
            Operand::Add(lhs, rhs) => {
                let lhs = lhs.codegen(compiler);
                let rhs = rhs.codegen(compiler);
                compiler.new_instruction(Instruction::Add(lhs, rhs))
            }
            Operand::Subtract(lhs, rhs) => {
                let lhs = lhs.codegen(compiler);
                let rhs = rhs.codegen(compiler);
                compiler.new_instruction(Instruction::Sub(lhs, rhs))
            },
            Operand::Divide(_, _) => todo!(),
        }
    }
}