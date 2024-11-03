use crate::*;

pub struct Compiler {
    pub(crate) compiled: Vec<Instruction>,
    pub(crate) scope_manager: ScopeManager,
    pub operands: Vec<Operand>,
    pub string_defines: Vec<(String, String)>,
}

impl Compiler {
    pub fn new() -> Self {
        Compiler {
            scope_manager: ScopeManager::new(),
            compiled: vec![],
            operands: vec![],
            string_defines : vec![],
        }
    }

    pub fn new_instruction(&mut self, instr: Instruction) {
        self.compiled.push(instr)
    }

    pub fn compile(&mut self) -> String {
        let mut defines = String::new();
        for (name, value) in &self.string_defines
        {
            let value = value.replace("\\n", "\", 10, \"");
            defines.push_str(&format!("{name}:\n\tdb \"{value}\", 0\n"));
        }

        for operand in &self.operands {
            if let Operand::FunctionDecl(_type, name, _, parameters) = operand {
                // If its a function, add it to the function  declaration.
                self.scope_manager.declare_function_global(
                    name,
                    _type,
                    &parameters
                        .iter()
                        .cloned()
                        .map(|v| v.1)
                        .collect::<Vec<OperandType>>(),
                );
            }
        }

        let mut buffer = String::new();
        // if we iter over a non-cloned operands then the for loop retains a borrow and we cant call operands.codegen
        let cloned_operand = self.operands.clone();
        for operands in &cloned_operand {
            operands.codegen(self);
        }

        for asm in &self.compiled {
            buffer.push_str(&asm.codegen_x86());
            buffer.push('\n')
        }

        format!("section .rodata\n{defines}\nsection .text\n{buffer}")
    }
}

impl Default for Compiler {
    fn default() -> Self {
        Self::new()
    }
}
