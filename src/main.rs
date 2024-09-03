use low_level_ir::*;

pub fn value_from_str<S>(size: Size, s: S) -> Value
where
    S: AsRef<str>,
{
    let s = s.as_ref();
    if let Ok(_) = s.parse::<i32>() {
        return Value::Int(s.to_string());
    }

    return Value::Variable(size, s.to_string());
}

fn main() {
    let mut module = IRModule { statements: vec![] };

    let contents = include_str!("../EXAMPLE.ir").to_string();

    let lines = contents.split("\n");

    for line in lines {
        let line = line.strip_suffix(";").unwrap_or(line);
        let line = line.split(" ").collect::<Vec<&str>>();

        match line[0].trim() {
            "label" => module.statements.push(IRStatement {
                op_type: OperandType::Undefined,
                operand: Operand::Label,
                lhs: Value::StringLiteral(line[1].to_string()),
                rhs: None,
            }),
            "void" => {
                if line[1] == "return"
                {
                    module.statements.push(IRStatement
                    {
                        op_type: OperandType::Undefined,
                        operand: Operand::Return,
                        lhs: Value::Null,
                        rhs: None,
                    })
                }
            }
            "i32" => {
                if line[1] == "*" {
                    module.statements.push(IRStatement {
                        op_type: OperandType::Int(Size::DoubleWord),
                        operand: Operand::IntMultiply,
                        lhs: value_from_str(Size::DoubleWord, line[2]),
                        rhs: Some(value_from_str(Size::DoubleWord, line[3])),
                    })
                } else if line[1] == "return"{
                    module.statements.push(IRStatement
                        {
                            op_type: OperandType::Int(Size::DoubleWord),
                            operand: Operand::Return,
                            lhs: value_from_str(Size::DoubleWord, line[2]),
                            rhs: None,
                        })
                } else {
                    module.statements.push(IRStatement {
                        op_type: OperandType::Int(Size::DoubleWord),
                        operand: Operand::Move,
                        lhs: Value::Variable(Size::DoubleWord, line[1].to_string()),
                        rhs: Some(value_from_str(Size::DoubleWord, line[3])),
                    });
                }
            },
            _ => {}
        }
    }

    println!("{:#?}", module);
    let compiled = module.compile();
    println!("{}", compiled);
}
