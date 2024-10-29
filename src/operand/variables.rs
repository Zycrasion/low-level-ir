use crate::*;

pub fn variable_declaration(
    ty: &OperandType,
    name: &String,
    value: &Value,
    compiler: &mut Compiler,
) {
    let (variable_information, ty) = compiler
        .scope_manager
        .get_variable_manager()
        .allocate(name, ty)
        .expect("Unable to allocate variable");

    m_set_variable(&ty, &variable_information, value, compiler);
}

pub fn set_variable(name: &String, value: &Value, compiler: &mut Compiler) {
    let (variable_information, ty) = compiler
        .scope_manager
        .get_variable_manager()
        .get(name)
        .expect("Unable to allocate variable");

    m_set_variable(&ty, &variable_information, value, compiler);
}

/// Helper function
fn m_set_variable(
    ty: &OperandType,
    variable_information: &VariableLocation,
    value: &Value,
    compiler: &mut Compiler,
) {
    let value = value.codegen(compiler, ty);

    if variable_information.is_stack() && value.is_stack() {
        // Can't set stack offset to another stack offset
        compiler.new_instruction(Instruction::Move(Register::AX.as_gen(&ty.size()), value));
        compiler.new_instruction(Instruction::Move(
            variable_information.as_gen(&ty.size()),
            Register::AX.as_gen(&ty.size()),
        ));
        return;
    }

    compiler.new_instruction(Instruction::Move(
        variable_information.as_gen(&ty.size()),
        value,
    ))
}
