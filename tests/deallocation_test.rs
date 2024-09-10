use low_level_ir::*;

#[test]
fn deallocation_test()
{
    let mut module = IRModule::new();
    module.statements = vec![
        Operand::Label("_start".to_string()).ir(OperandType::Undefined),
        Operand::Move(Value::Variable(Size::DoubleWord, "a".to_string()), Value::Int("20".to_string())).ir(OperandType::Int(Size::DoubleWord)),
        Operand::Move(Value::Variable(Size::DoubleWord, "b".to_string()), Value::VariableReference("a".to_string())).ir(OperandType::Int(Size::DoubleWord)),
        Operand::Move(Value::Variable(Size::DoubleWord, "c".to_string()), Value::Int("20".to_string())).ir(OperandType::Int(Size::DoubleWord)),
        Operand::Return(Value::VariableReference("b".to_string())).ir(OperandType::Int(Size::DoubleWord)),
    ];

    module.optimise();

    println!("{:#?}", module.statements);
    println!("{}", module.compile());
}