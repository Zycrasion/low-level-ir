use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompareOperation
{
    GT,
    GTE,
    LT,
    LTE,
    EQ,
    NEQ,
}

impl CompareOperation
{
    pub fn as_suffix(&self) -> String
    {
        match self
        {
            CompareOperation::GT => "g",
            CompareOperation::GTE => "ge",
            CompareOperation::LT => "l",
            CompareOperation::LTE => "le",
            CompareOperation::EQ => "e",
            CompareOperation::NEQ => "ne",
        }.to_string()
    }

    pub fn get_opposite(&self) -> Self
    {
        match self
        {
            CompareOperation::GT => CompareOperation::LTE,
            CompareOperation::GTE => CompareOperation::LT,
            CompareOperation::LT => CompareOperation::GTE,
            CompareOperation::LTE => CompareOperation::GT,
            CompareOperation::EQ => CompareOperation::NEQ,
            CompareOperation::NEQ => CompareOperation::EQ
        }
    }
}

#[derive(Debug, Clone)]
pub struct ComparePredicate
{
    pub operation : CompareOperation,
    pub lhs : Value,   
    pub rhs : Value,
}

pub fn if_statement(predicate : &ComparePredicate, main_body : &[Operand], compiler : &mut Compiler)
{
    let ComparePredicate { operation, lhs, rhs } = predicate;
    let _rhs= rhs;
    let rhs = lhs;
    let lhs = _rhs;

    let op_size = lhs.size(compiler);

    let mut lhs_gen = rhs.codegen(compiler);
    let rhs_gen = lhs.codegen_size(compiler, &op_size);

    if lhs_gen.is_stack() && rhs_gen.is_stack() || lhs_gen.is_immediate()
    {
        let new_location = Register::AX.as_gen(&op_size);
        compiler.new_instruction(Instruction::Move(new_location.clone(), lhs_gen));
        lhs_gen = new_location;
    }

    compiler.new_instruction(Instruction::Compare(lhs_gen, rhs_gen));

    let id = compiler.fetch_id(".IF");

    let jump_instr = Instruction::JumpConditional { label_destination: id.clone(), conditional: operation.get_opposite() };

    compiler.new_instruction(jump_instr);

    main_body.iter().for_each(|v|
    {
        v.codegen(compiler);
    });

    compiler.new_instruction(Instruction::Label(id));
}