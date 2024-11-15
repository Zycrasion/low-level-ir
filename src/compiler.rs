use crate::*;

pub struct Compiler {
    pub(crate) compiled: Vec<Instruction>,
    pub(crate) scope_manager: ScopeManager,
    pub operands: Vec<Operand>,
    pub string_defines: Vec<(String, String)>,
    pub id : usize,
}

impl Compiler {
    pub fn new() -> Self {
        Compiler {
            scope_manager: ScopeManager::new(),
            compiled: vec![],
            operands: vec![],
            string_defines : vec![],
            id : 0
        }
    }

    pub fn fetch_id(&mut self, prefix : &str) -> String
    {
        self.id += 1;
        format!("{prefix}{}", self.id)
    }

    pub fn new_instruction(&mut self, instr: Instruction) {
        self.compiled.push(instr)
    }

    pub fn compile(mut self) -> String {
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
                    &parameters // I hate collecting the vector then referencing into a slice too, there is no way around it however
                        .iter()
                        .cloned()
                        .map(|v| v.1)
                        .collect::<Vec<OperandType>>(),
                );
            }
        }

        let mut buffer = String::new();
        // take ownership of operands
        let operands = std::mem::take(&mut self.operands);
        for operand in &operands {
            operand.codegen(&mut self);
        }

        for asm in self.compiled {
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
