use crate::ValueCodegen;

pub enum Instruction
{
    Label(ValueCodegen),
    Move(ValueCodegen, ValueCodegen),
    IntMultiply(ValueCodegen, ValueCodegen),
    Multiply(ValueCodegen, ValueCodegen),
    Return,
    Push(ValueCodegen),
    Pop(ValueCodegen)
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
        }
    }
}