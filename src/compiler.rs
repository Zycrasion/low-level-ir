use crate::*;

pub struct Compiler {

    pub compiled : Vec<Instruction>,
    pub scope_manager : ScopeManager,
}

impl Compiler {
    pub fn new() -> Self
    {
        Compiler {
            scope_manager : ScopeManager::new(),
            compiled : vec![]
        }
    }
    
    pub fn new_instruction(&mut self, instr : Instruction)
    {
        self.compiled.push(instr)
    }
}

impl Default for Compiler
{
    fn default() -> Self {
        Self::new()
    }
}