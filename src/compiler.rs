use std::collections::HashMap;
use crate::{RegisterAllocator, Size, ValueCodegen};

pub struct Compiler {
    pub variables: HashMap<String, (u32, Size)>,
    pub registers: RegisterAllocator,
    pub current_offset: Vec<u32>,
}

impl Compiler {
    pub fn new() -> Self
    {
        Compiler {
            variables: HashMap::new(),
            current_offset: vec![],
            registers: RegisterAllocator::new(),
        }
    }

    pub fn new_stack_frame(&mut self) {
        self.current_offset.push(0)
    }

    pub fn offset(&mut self) -> &mut u32 {
        if self.current_offset.len() == 0 {
            self.current_offset.push(0);
        }

        self.current_offset.last_mut().unwrap()
    }

    pub fn allocate_variable(&mut self, name: &String, size: &Size) -> ValueCodegen {
        if self.registers.allocate(name).is_ok()
        {
            return ValueCodegen::Register(self.registers.get(name).unwrap().as_size(size))
        }

        let off = *self.offset();

        self.variables.insert(name.clone(), (off, *size));

        let offset = self.offset();
        *offset += size.get_bytes() as u32;
        ValueCodegen::StackOffset(format!("{} [rbp-{}]", size.name(), offset))
    }

    pub fn get_or_allocate_variable(&mut self, name: &String, size: &Size) -> ValueCodegen {
        if let Some(reg) = self.registers.get_or_allocate(name)
        {
            return ValueCodegen::Register(reg.as_size(size));
        }

        if let Some((offset, other_size)) = self.variables.get(name) {
            if *other_size != *size {
                eprintln!("Mismatched Sizes for variable");
                panic!();
            }

            ValueCodegen::StackOffset(format!("{} [rbp-{}]", size.name(), offset))
        } else {
            self.allocate_variable(name, size)
        }
    }
}