use std::collections::HashMap;


use crate::*;

#[derive(Debug)]
pub struct VariableManager {
    registers: HashMap<Register, Option<String>>,
    variables: HashMap<String, (Register, OperandType)>,
}

pub const SCRATCH_REGISTERS : &[Register] = &[
    Register::SI, // System V-Abi scratch registers
    Register::DX,
    Register::CX,
    Register::R8,
    Register::R9,
    Register::R10,
    Register::R11,
];


impl VariableManager {
    pub fn used_registers(&self) -> Vec<Register>
    {
        self.variables.values().map(|v| v.0).collect()
    }
    
    pub fn new() -> Self {
        let mut map = HashMap::new();

        for register in SCRATCH_REGISTERS.iter() {
            map.insert(*register, None);
        }

        Self {
            registers: map,
            variables: HashMap::new(),
        }
    }

    pub fn deallocate(&mut self, var: &String) -> bool
    {
        if !self.variables.contains_key(var)
        {
            return false;
        }

        *self.registers.get_mut(&self.variables[var].0).unwrap() = None;
        self.variables.remove(var);
        true
    }

    pub fn allocate(&mut self, var: &String, _type : &OperandType) -> Result<(Register, OperandType), ()> {
        for reg in SCRATCH_REGISTERS.iter() {
            if self.registers[reg].is_none() {
                self.registers.insert(*reg, Some(var.clone()));
                self.variables.insert(var.clone(), (*reg, *_type));
                return Ok((*reg, *_type));
            }
        }

        Err(())
    }

    pub fn get(&self, var: &String) -> Option<(Register, OperandType)> {
        if !self.variables.contains_key(var) {
            return None;
        }

        Some(self.variables[var])
    }

    pub fn get_or_allocate(&mut self, var: &String, _type : &OperandType) -> Option<(Register, OperandType)> {
        let _ = self.allocate(var, _type);
        let get = self.get(var);

        if get.is_some() {
            return get;
        }
        None
    }
}