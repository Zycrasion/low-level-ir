use std::collections::HashMap;
use crate::{FunctionManager, Instruction, OperandType, Size, ValueCodegen, VariableManager};

pub struct Compiler {
    pub variables: VariableManager,
    pub functions: FunctionManager,
    pub compiled : Vec<Instruction>,
}

impl Compiler {
    pub fn new() -> Self
    {
        Compiler {
            variables: VariableManager::new(),
            functions : FunctionManager::new(),
            compiled : vec![]
        }
    }
    
    pub fn new_instruction(&mut self, instr : Instruction)
    {
        self.compiled.push(instr)
    }
}