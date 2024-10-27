use crate::*;

#[derive(Debug, Clone)]
pub struct IRModule {
    pub operands: Vec<Operand>,
}

impl Default for IRModule {
    fn default() -> Self {
        Self::new()
    }
}

impl IRModule {
    pub fn new() -> Self {
        Self { operands: vec![] }
    }

    pub fn optimise(&mut self)
    {
        println!("[TODO]: Fix variable deallocation")
    }

    pub fn compile(&self) -> String {
        let mut compiler = Compiler::new();

        // first we will scan through the code for functions,

        for operand in &self.operands {
            if let Operand::FunctionDecl(_type, name, _, parameters) = operand
            {
                // If its a function, add it to the function  declaration.
                compiler.scope_manager.declare_function_global(name, _type, &parameters.iter().cloned().map(|v| v.1).collect::<Vec<OperandType>>());
            }
        }


        let mut buffer = String::new();
        for operands in &self.operands {
            operands.codegen(&mut compiler);
        }

        for asm in &compiler.compiled
        {
            buffer.push_str(&asm.codegen_x86());
            buffer.push('\n')
        }

        buffer
    }
}
