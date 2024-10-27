use std::collections::HashMap;

use crate::*;

#[derive(Debug)]
pub struct FunctionManager
{
    functions : HashMap<String, (OperandType, Vec<OperandType>)>,
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

    pub fn get_function_type(&self, name : &String) -> Option<(OperandType, Vec<OperandType>)>
    {
        self.functions.get(name).cloned()
    }

    pub fn declare_function(&mut self, name : &String, _type : &OperandType, params : &[OperandType]) -> Result<(), ()>
    {
        if self.functions.contains_key(name)
        {
            return Err(())
        }

        self.functions.insert(name.clone(), (_type.clone(), params.to_vec()));
        
        Ok(())
    }
}