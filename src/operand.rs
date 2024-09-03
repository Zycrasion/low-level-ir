use crate::{Compiler, Register, Size, Value};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum OperandType {
    Undefined,
    Int(Size),
}

#[derive(Debug, Clone, Copy)]
pub enum Operand {
    Move,
    Label,
    Multiply,
    IntMultiply,
    Add,
    Subtract,
    Divide,
    IntDivide,
    Return
}

impl Operand {
    pub fn codegen(
        &self,
        lhs: &Value,
        rhs: &Option<Value>,
        _ty: &OperandType,
        compiler: &mut Compiler,
    ) -> String {
        match self {
            Operand::Move => {
                let rhs = rhs.as_ref().unwrap().codegen(compiler);
                let lhs = lhs.codegen(compiler);

                if lhs.is_stack() && rhs.is_stack()
                {
                    return format!("mov {}, {}\nmov {}, {0}", Register::AX.as_size(&Size::DoubleWord), rhs.inner(), lhs.inner());
                }

                return format!("mov {}, {}", lhs.inner(), rhs.inner());
            }
            Operand::Label => return format!("{}:\npush rbp", lhs.codegen(compiler).inner()),
            Operand::Multiply | Operand::IntMultiply => {
                let rhs = rhs.as_ref().unwrap();
                match _ty {
                    OperandType::Undefined => todo!(),
                    OperandType::Int(size) => {
                        let lhs = lhs.codegen(compiler);
                        let rhs = rhs.codegen(compiler);

                        if lhs.is_stack() && rhs.is_stack()
                        {
                            return format!(
                                "mov eax, {}\n{} eax, {}\nmov {1}, eax",
                                lhs.inner(),
                                match self {Self::IntMultiply => "imul", _ => "mul"},
                                rhs.inner(),
                            );
                        }

                        return format!(
                            "{} {}, {}",
                            match self {Self::IntMultiply => "imul", _ => "mul"},
                            lhs.inner(),
                            rhs.inner(),
                        );
                    }
                }
            }
            Operand::Return => {
                if *_ty == OperandType::Undefined
                {
                    format!("pop rbp\nret")
                } else
                {
                    let size = if let OperandType::Int(size) = _ty {size} else {panic!()};
                    format!("mov {}, {}\npop rbp\nret", Register::AX.as_size(size), lhs.codegen(compiler).inner())
                }
            }
            Operand::Add => todo!(),
            Operand::Subtract => todo!(),
            Operand::Divide => todo!(),
            Operand::IntDivide => todo!(),
        }
    }
}