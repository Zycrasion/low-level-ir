use crate::{Register, ValueCodegen};

#[derive(Clone)]
pub enum Instruction
{
    AsmLiteral(String),
    Label(String),
    Move(ValueCodegen, ValueCodegen),
    IntMultiply(ValueCodegen, ValueCodegen),
    Multiply(ValueCodegen, ValueCodegen),
    Return,
    Push(ValueCodegen),
    Pop(ValueCodegen),
    Add(ValueCodegen, ValueCodegen),
    Sub(ValueCodegen, ValueCodegen),
    LoadAddress(ValueCodegen, ValueCodegen),
    Call(String)
}

impl Instruction
{
    pub fn codegen_x86(&self) -> String
    {
        match self
        {
            Instruction::Label(name)            => format!("{name}:"),
            Instruction::Move(dst, src)         => format!("mov {dst}, {src}"),
            Instruction::IntMultiply(dst, src)  => format!("imul {dst}, {src}"),
            Instruction::Multiply(dst, src)     => format!("mul {dst}, {src}"),
            Instruction::Return                 => format!("ret"),
            Instruction::Push(src)              => format!("push {src}"),
            Instruction::Pop(dst)               => format!("pop {dst}"),
            Instruction::Add(dst, src)          => format!("add {dst}, {src}"),
            Instruction::Sub(dst, src)          => format!("sub {dst}, {src}"),
            Instruction::LoadAddress(dst, src)  => format!("lea {dst}, {src}"),
            Instruction::Call(name)             => format!("call {name}"),
            Instruction::AsmLiteral(literal)    => literal.clone()
        }
    }
}