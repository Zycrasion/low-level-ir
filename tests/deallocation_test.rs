use low_level_ir::*;

#[test]
fn deallocation_test()
{
    let mut module = IRModule::new();
    module.operands = vec![
        Operand::FunctionDecl(OperandType::Int(Size::DoubleWord), String::from("_start"), vec![
            Operand::DeclareVariable(OperandType::Int(Size::DoubleWord), "a".to_string(), Value::Variable("c".to_string())),
            Operand::DeclareVariable(OperandType::Int(Size::DoubleWord), "b".to_string(), Value::Variable("a".to_string())),
            Operand::Return(Value::Variable("b".to_string())),
        ], vec![("c".to_string(), OperandType::Int(Size::DoubleWord))]),
    ];

    module.optimise();
    println!("{:#?}", module);

    println!("{}", module.compile());
}