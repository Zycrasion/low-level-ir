use low_level_ir::*;

#[test]
fn deallocation_test()
{
    let mut module = IRModule::new();
    module.statements = vec![
        Operand::Label.ir(OperandType::Undefined, Value::StringLiteral(String::from("_start")), None),
        Operand::Move.ir(OperandType::Int(Size::DoubleWord), Value::Variable(Size::DoubleWord, "a".to_string()), Some(Value::Int("20".to_string()))),
        Operand::Move.ir(OperandType::Int(Size::DoubleWord), Value::Variable(Size::DoubleWord, "b".to_string()), Some(Value::VariableReference("a".to_string()))),
        Operand::Move.ir(OperandType::Int(Size::DoubleWord), Value::Variable(Size::DoubleWord, "c".to_string()), Some(Value::Int("20".to_string()))),
        Operand::Return.ir(OperandType::Int(Size::DoubleWord), Value::VariableReference("b".to_string()), None),
    ];

    module.optimise();

    println!("{:#?}", module.statements);
    println!("{}", module.compile());
}