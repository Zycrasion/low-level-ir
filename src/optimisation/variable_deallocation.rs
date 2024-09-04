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
        if let Value::VariableReference(name) = statement.lhs.clone()
        {
            variable_last_usage.insert(name, i);
        }
        if let Value::Variable(_, name) = statement.lhs.clone()
        {
            variable_last_usage.insert(name, i);
        }
        if let Some(rhs) = statement.rhs.clone()
        {
            if let Value::VariableReference(name) = rhs
            {
                variable_last_usage.insert(name, i);
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
                if let Value::Variable(_, name) = statement.lhs.clone()
                {   
                    if name == value.0
                    {
                        statements.pop();
                    }
                    continue 'value_loop;
                }
                statements.push(Operand::DropVariable.ir(OperandType::Undefined, Value::VariableReference(value.0.clone()), None))
            }
        }

        i += 1;
    }

    ir_module.statements = statements;
}