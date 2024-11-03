use crate::*;

pub fn function_call(name: &String, parameters: &Vec<Value>, compiler: &mut Compiler) {
    let function = compiler
        .scope_manager
        .get_function(name)
        .expect("No Function Exists");

    for (i, value) in parameters.iter().enumerate() {
        let value = value.codegen(compiler, &function.1[i]);
        compiler.new_instruction(Instruction::Push(PARAMETER_REGISTERS[i].as_gen(&Size::QuadWord)));
        compiler.new_instruction(Instruction::Move(
            PARAMETER_REGISTERS[i].as_gen(&function.1[i].size()),
            value,
        ));
    }
    compiler.new_instruction(Instruction::Call(name.clone()));
    let len = parameters.len();
    for i in 1..=len {
        let i = len - i;
        compiler.new_instruction(Instruction::Pop(PARAMETER_REGISTERS[i].as_gen(&Size::QuadWord)));
    }
}

pub fn function_decl(
    return_type: &OperandType,
    name: &String,
    operands: &Vec<Operand>,
    parameters: &Vec<(String, OperandType)>,
    compiler: &mut Compiler,
) {
    compiler.scope_manager.enter_scope();
    compiler.new_instruction(Instruction::Label(name.clone()));
    compiler.new_instruction(Instruction::Push(Register::BP.as_gen(&Size::QuadWord)));
    compiler.new_instruction(Instruction::Move(
        Register::BP.as_gen(&Size::QuadWord),
        Register::SP.as_gen(&Size::QuadWord),
    ));
    compiler.new_instruction(Instruction::Label("[PLACEHOLDER]".to_string()));
    let placeholder_index = compiler.compiled.len() - 1;

    for (i, param) in parameters.iter().enumerate() {
        compiler
            .scope_manager
            .get_variable_manager()
            .allocate_parameter(&param.0, &param.1, i);
    }

    if !operands.iter().any(|v| matches!(v, Operand::Return(_)))
    {
        eprintln!("No return statement in function {name}!");
        panic!()
    }

    for op in operands {
        if let Operand::Return(value) = op {
            if *value != Value::Null {
                let value = value.codegen(compiler, return_type);

                // Edge case where the return value is a maths expression
                // Since all Maths Expressions are calculated using the AX register there is no need to move it...
                if value.inner() != Register::AX.as_size(&return_type.size()) {
                    compiler.new_instruction(Instruction::Move(
                        Register::AX.as_gen(&return_type.size()),
                        value,
                    ));
                }
            }

            let stack = compiler.scope_manager.get_variable_manager().used_stack();
            if stack == 0 {
                compiler.compiled.remove(placeholder_index);
            } else {
                compiler.compiled[placeholder_index] = Instruction::Sub(
                    Register::SP.as_gen(&Size::QuadWord),
                    ValueCodegen::Number(stack.to_string()),
                );
            }
            compiler.new_instruction(Instruction::Move(
                Register::SP.as_gen(&Size::QuadWord),
                Register::BP.as_gen(&Size::QuadWord),
            ));
            compiler.new_instruction(Instruction::Pop(Register::BP.as_gen(&Size::QuadWord)));
            compiler.new_instruction(Instruction::Return);
            return;
        } else {
            op.codegen(compiler);
        }
    }
    compiler.scope_manager.leave_scope();
}
