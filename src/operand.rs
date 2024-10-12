use crate::{Compiler, Instruction, Register, Size, Value, ValueCodegen};

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
    // Type-Explicit
    DeclareVariable(OperandType, String, Value),
    FunctionDecl(OperandType, String, Vec<Operand>),
    Multiply(OperandType, Value, Value),
    Add(OperandType, Value, Value),
    Subtract(OperandType, Value, Value),
    Divide(OperandType, Value, Value),

    // Type-Implicit
    SetVariable(String, Value),
    DropVariable(String),
    FunctionCall(String),
    Return(Value),
    InlineAssembly(String),
}

impl Operand {
    pub fn get_values(&self) -> Vec<Value>
    {
        match self
        {
            Self::Multiply(_, a, b) | Self::Add(_, a, b) | Self::Subtract(_, a, b) | Self::Divide(_, a, b) => vec![a.clone(), b.clone()],
            Self::Return(a) | Self::DeclareVariable(_, _, a) | Self::SetVariable(_, a) => vec![a.clone()],
            Self::DropVariable(_) |  Self::InlineAssembly(_) | Self::FunctionCall(_) => vec![],
            Self::FunctionDecl(_, _, a) => a.iter().flat_map(|v| v.get_values()).collect()
        }
    }
    
    pub fn codegen(
        &self,
        compiler: &mut Compiler,
    ) {
        match self {
            Operand::DeclareVariable(ty, name, value) => {
                let value = value.codegen(compiler);
                let variable_location = compiler.variables.allocate(name, &ty).expect("Unable to allocate variable");

                // It's impossible for the VariableManager to allocate on the Stack at the moment
                // if variable_location.is_stack() && value.is_stack()
                // {
                //     // Can't set stack offset to another stack offset
                //     compiler.new_instruction(Instruction::Move(Register::AX.as_gen(&ty.size()), value));
                //     compiler.new_instruction(Instruction::Move(variable_location, Register::AX.as_gen(&ty.size())));
                //     return;
                // }
                
                compiler.new_instruction(Instruction::Move(variable_location.0.as_gen(&ty.size()), value))
            }
            Operand::SetVariable(name, value) => {
                let value = value.codegen(compiler);
                let variable_location = compiler.variables.get(name).expect("Unable to allocate variable");

                compiler.new_instruction(Instruction::Move(variable_location.0.as_gen(&variable_location.1.size()), value));
            }
            Operand::InlineAssembly(asm) =>
            {
                compiler.new_instruction(Instruction::AsmLiteral(asm.clone()));
            }
            Operand::FunctionCall(name) => {
                compiler.new_instruction(Instruction::Call(name.clone()));
            }
            Operand::FunctionDecl(_type, name, operands) => {
                compiler.new_instruction(Instruction::Label(name.clone()));
                compiler.new_instruction(Instruction::Push(Register::BP.as_gen(&Size::QuadWord)));
                compiler.functions.declare_function(name, _type).expect("Function {name} is already defined");

                for op in operands
                {
                    if let Operand::Return(value) = op
                    {
                        if *value != Value::Null
                        {
                            let value = value.codegen(compiler);
                            compiler.new_instruction(Instruction::Move(Register::AX.as_gen(&_type.size()), value));
                        }

                        compiler.new_instruction(Instruction::Pop(Register::BP.as_gen(&Size::QuadWord)));
                        compiler.new_instruction(Instruction::Return);
                    } else
                    {
                        op.codegen(compiler);
                    }
                }

            },
            Operand::Return(_) => {
                eprintln!("Return not paired with function.");
                panic!();
            },
            Operand::Multiply(_ty, lhs, rhs) => {
                let lhs = lhs.codegen(compiler);
                let rhs = rhs.codegen(compiler);    
                let size = &_ty.size();

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
            Operand::DropVariable(name) =>
            {
                // This variable is no longer used anywhere
                compiler.variables.deallocate(name);
            }
            Operand::Add(ty, lhs, rhs) => {
                let lhs = lhs.codegen(compiler);
                let rhs = rhs.codegen(compiler);
                compiler.new_instruction(Instruction::Add(lhs, rhs))
            }
            Operand::Subtract(ty, lhs, rhs) => {
                let lhs = lhs.codegen(compiler);
                let rhs = rhs.codegen(compiler);
                compiler.new_instruction(Instruction::Sub(lhs, rhs))
            },
            Operand::Divide(_, _, _) => todo!(),
        }
    }
}