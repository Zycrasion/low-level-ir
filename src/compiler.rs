use crate::*;

pub struct Compiler {
    pub(crate) compiled: Vec<Instruction>,
    pub(crate) scope_manager: ScopeManager,
    pub operands: Vec<Operand>,
}

impl Compiler {
    pub fn new() -> Self {
        Compiler {
            scope_manager: ScopeManager::new(),
            compiled: vec![],
            operands: vec![],
        }
    }

    pub fn new_instruction(&mut self, instr: Instruction) {
        self.compiled.push(instr)
    }

    pub fn compile(&mut self) -> String {
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

        buffer
    }
}

impl Default for Compiler {
    fn default() -> Self {
        Self::new()
    }
}
