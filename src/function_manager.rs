use std::collections::HashMap;

use crate::*;

pub struct FunctionManager
{
    functions : HashMap<String, OperandType>,
}

impl FunctionManager
{
    pub fn new() -> Self
    {
        Self
        {
            functions : HashMap::new(),
        }
    }

    pub fn get_function_type(&self, name : &String) -> Option<OperandType>
    {
        self.functions.get(name).cloned()
    }

    pub fn declare_function(&mut self, name : &String, _type : &OperandType) -> Result<(), ()>
    {
        if self.functions.contains_key(name)
        {
            return Err(())
        }

        self.functions.insert(name.clone(), *_type);
        
        Ok(())
    }
}