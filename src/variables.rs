use std::collections::HashMap;

use crate::*;

#[derive(Debug, Clone, Copy)]
pub enum VariableLocation {
    Register(Register),
    StackOffset(u32),
}

impl VariableLocation {
    pub fn as_reg(&self) -> Option<Register> {
        match self {
            VariableLocation::Register(reg) => Some(*reg),
            _ => None,
        }
    }

    pub fn is_stack(&self) -> bool {
        matches!(self, Self::StackOffset(_))
    }

    pub fn as_gen(&self, size: &Size) -> ValueCodegen {
        match self {
            VariableLocation::Register(register) => register.as_gen(size),
            VariableLocation::StackOffset(stack) => {
                ValueCodegen::StackOffset(format!("{} [rbp-{}]", size.name(), stack))
            }
        }
    }

    pub fn as_ptr(&self) -> ValueCodegen {
        match self {
            VariableLocation::Register(register) => register.as_ptr(),
            VariableLocation::StackOffset(stack) => {
                ValueCodegen::Pointer(format!("QWORD [rbp-{}]", stack))
            }
        }
    }
}

#[derive(Debug)]
pub struct VariableManager {
    variables: HashMap<String, (VariableLocation, OperandType)>,
    stack_location: u32,
}

pub const PARAMETER_REGISTERS: &[Register] = &[
    Register::DI,
    Register::SI,
    Register::DX,
    Register::CX,
    Register::R8,
    Register::R9,
];

impl Default for VariableManager {
    fn default() -> Self {
        Self::new()
    }
}

impl VariableManager {
    pub fn used_stack(&self) -> u32 {
        self.stack_location
    }

    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            stack_location: 0,
        }
    }

    pub fn deallocate(&mut self, var: &str) -> bool {
        if !self.variables.contains_key(var) {
            return false;
        }

        true
    }

    pub fn allocate(
        &mut self,
        var: &str,
        _type: &OperandType,
    ) -> Option<(VariableLocation, OperandType)> {
        let size = _type.size().get_bytes();

        self.stack_location += size as u32;
        let variable = (
            VariableLocation::StackOffset(self.stack_location),
            _type.clone(),
        );

        self.variables.insert(var.to_string(), variable.clone());

        Some(variable)
    }

    pub fn allocate_parameter(&mut self, var: &str, _type: &OperandType, i: usize) {
        self.variables.insert(
            var.to_string(),
            (
                VariableLocation::Register(PARAMETER_REGISTERS[i]),
                _type.clone(),
            ),
        );
    }

    pub fn get(&self, var: &str) -> Option<(VariableLocation, OperandType)> {
        if !self.variables.contains_key(var) {
            return None;
        }

        Some(self.variables[var].clone())
    }

    pub fn get_or_allocate(
        &mut self,
        var: &str,
        _type: &OperandType,
    ) -> Option<(VariableLocation, OperandType)> {
        let _ = self.allocate(var, _type);
        let get = self.get(var);

        if get.is_some() {
            return get;
        }
        None
    }
}
