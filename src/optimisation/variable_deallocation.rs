use std::collections::HashMap;

use crate::*;

/// Deallocates and also will delete variables that are NEVER used
pub fn deallocation_pass(ir_module : &mut IRModule)
{
    let mut statements = Vec::with_capacity(ir_module.statements.capacity());
    let mut variable_last_usage = HashMap::new();

    let mut i = 0usize;
    for statement in &ir_module.statements
    {
        let values = statement.operand.get_values();

        for v in values
        {
            match v
            {
                Value::Variable(_, name) | Value::VariableReference(name) => {variable_last_usage.insert(name.clone(), i);},
                // Value::VariableReference(name) => variable_last_usage.insert(name, i),
                _ => {}
            }
        }
       
        i += 1;
    }

    let values = variable_last_usage.iter().map(|(k, v)| (k.clone(), *v)).collect::<Vec<(String, usize)>>();

    i = 0;
    for statement in &ir_module.statements
    {
        statements.push(statement.clone());
        'value_loop:
        for value in &values
        {
            if value.1 == i
            {
                // If the variable was last used on the line it was assigned,
                // Remove the statement that assigns it and dont append the drop.
                if let Operand::Move(lhs, _) = statement.operand.clone()
                {   
                    if let Value::Variable(_, name) = lhs
                    {
                        if name == value.0
                        {
                            statements.pop();
                            continue 'value_loop;
                        }
                    }
                }
                statements.push(Operand::DropVariable(value.0.clone()).ir(OperandType::Undefined))
            }
        }

        i += 1;
    }

    ir_module.statements = statements;
}