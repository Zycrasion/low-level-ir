use crate::*;

pub fn variable_declaration(
    ty: &OperandType,
    name: &str,
    value: &Value,
    compiler: &mut Compiler,
) {
    let (variable_information, ty) = compiler
        .scope_manager
        .get_variable_manager()
        .allocate(name, ty)
        .expect("Unable to allocate variable");

    m_set_variable(&ty.size(), &variable_information.as_gen(&ty.size()), value, compiler);
}

pub fn set_value(dst: &Value, value: &Value, compiler: &mut Compiler) {
    let size = dst.size(compiler);
    let loc = dst.codegen_lhs(compiler);


    m_set_variable(&size, &loc, value, compiler);
}

/// Helper function
fn m_set_variable(
    ty: &Size,
    variable_information: &ValueCodegen,
    value: &Value,
    compiler: &mut Compiler,
) {
    let value = value.codegen(compiler);

    if variable_information.is_stack() && value.is_stack() {
        // Can't set stack offset to another stack offset
        compiler.new_instruction(Instruction::Move(Register::AX.as_gen(ty), value));
        compiler.new_instruction(Instruction::Move(
            variable_information.clone(),
            Register::AX.as_gen(ty),
        ));
        return;
    }

    compiler.new_instruction(Instruction::Move(
        variable_information.clone(),
        value,
    ))
}
