use std::{collections::HashMap, hash::Hash};

use crate::{scope, Compiler, Instruction, Register, Size, Value, ValueCodegen, PARAMETER_REGISTERS, SCRATCH_REGISTERS};

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
    FunctionDecl(OperandType, String, Vec<Operand>, Vec<(String, OperandType)>),
    Multiply(OperandType, Value, Value),
    Add(OperandType, Value, Value),
    Subtract(OperandType, Value, Value),
    Divide(OperandType, Value, Value),

    // Type-Implicit
    SetVariable(String, Value),
    DropVariable(String),
    FunctionCall(String, Vec<Value>),
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
            Self::FunctionCall(_, a) => a.clone(),
            Self::DropVariable(_) |  Self::InlineAssembly(_)  => vec![],
            Self::FunctionDecl(_, _, a, _) => a.iter().flat_map(|v| v.get_values()).collect()
        }
    }
    
    pub fn codegen(
        &self,
        compiler: &mut Compiler,
    ) {
        match self {
            Operand::DeclareVariable(ty, name, value) => {
                let value = value.codegen(compiler);
                let variable_location = compiler.scope_manager.get_variable_manager().allocate(name, &ty).expect("Unable to allocate variable");

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
                let variable_location = compiler.scope_manager.get_variable_manager().get(name).expect("Unable to allocate variable");

                compiler.new_instruction(Instruction::Move(variable_location.0.as_gen(&variable_location.1.size()), value));
            }
            Operand::InlineAssembly(asm) =>
            {
                compiler.new_instruction(Instruction::AsmLiteral(asm.clone()));
            }
            Operand::FunctionCall(name, parameters) => {
                let function = compiler.scope_manager.get_function(name).expect("No Function Exists");
                let mut saved_registers = vec![];
                for (i, value) in parameters.iter().enumerate()
                {
                    let value = value.codegen(compiler);
                    if compiler.scope_manager.get_variable_manager().used_registers().contains(&PARAMETER_REGISTERS[i])
                    {
                        compiler.new_instruction(Instruction::Push(PARAMETER_REGISTERS[i].as_gen(&Size::QuadWord)));
                        saved_registers.push(PARAMETER_REGISTERS[i]);
                    }
                    compiler.new_instruction(Instruction::Move(PARAMETER_REGISTERS[i].as_gen(&function.1[i].size()), value));
                }
                compiler.new_instruction(Instruction::Call(name.clone()));
                saved_registers.reverse();
                for reg in saved_registers
                {
                    compiler.new_instruction(Instruction::Pop(reg.as_gen(&Size::QuadWord)));
                }
            }
            Operand::FunctionDecl(_type, name, operands, parameters) => {
                compiler.scope_manager.enter_scope();
                compiler.new_instruction(Instruction::Label(name.clone()));
                compiler.new_instruction(Instruction::Push(Register::BP.as_gen(&Size::QuadWord)));
                for (i, param) in parameters.iter().enumerate()
                {
                    compiler.scope_manager.get_variable_manager().allocate_parameter(&param.0, &param.1, i);
                }

                let saved_asm = compiler.compiled.clone();
                compiler.compiled = vec![];

                for op in operands
                {
                    if let Operand::Return(value) = op
                    {
                        if *value != Value::Null
                        {
                            let value = value.codegen(compiler);
                            compiler.new_instruction(Instruction::Move(Register::AX.as_gen(&_type.size()), value));
                        }
                        let mut asm = compiler.compiled.clone();
                        asm.reverse();
                        compiler.compiled = saved_asm;
                        let used_registers = compiler.scope_manager.get_variable_manager().used_registers();

                        for reg in used_registers.clone()
                        {
                            asm.push(Instruction::Push(reg.as_gen(&Size::QuadWord)));
                        }

                        asm.reverse();
                        for reg in used_registers
                        {
                            asm.push(Instruction::Pop(reg.as_gen(&Size::QuadWord)));
                        }

                        compiler.compiled.append(&mut asm);
                        
                        compiler.new_instruction(Instruction::Pop(Register::BP.as_gen(&Size::QuadWord)));
                        compiler.new_instruction(Instruction::Return);
                        return;
                    } else
                    {
                        op.codegen(compiler);
                    }
                }
                compiler.scope_manager.leave_scope();
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
                compiler.scope_manager.get_variable_manager().deallocate(name);
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