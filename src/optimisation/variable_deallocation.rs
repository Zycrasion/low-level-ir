use std::collections::HashMap;

use crate::*;

/// Deallocates and also will delete variables that are NEVER used
pub fn deallocation_pass(ir_module : &mut IRModule)
{
    let mut operands = Vec::with_capacity(ir_module.operands.capacity());
    let mut variable_last_usage = HashMap::new();

    let mut i = 0usize;
    for operand in &ir_module.operands
    {
        let values = operand.get_values();

        for v in values
        {
            match v
            {
                Value::Variable(name)  => {variable_last_usage.insert(name.clone(), i);},
                _ => {}
            }
        }
       
        i += 1;
    }

    let values = variable_last_usage.iter().map(|(k, v)| (k.clone(), *v)).collect::<Vec<(String, usize)>>();

    i = 0;
    for operand in &ir_module.operands
    {
        operands.push(operand.clone());
        'value_loop:
        for value in &values
        {
            if value.1 == i
            {
                // If the variable was last used on the line it was assigned,
                // Remove the statement that assigns it and dont append the drop.
                if let Operand::DeclareVariable(_, name, _) = operand.clone()
                { 
                    if name == value.0
                    {
                        operands.pop();
                        continue 'value_loop;
                    }
                }
                operands.push(Operand::DropVariable(value.0.clone()))
            }
        }

        i += 1;
    }

    ir_module.operands = operands;
}