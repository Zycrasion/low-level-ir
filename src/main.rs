use std::{collections::HashMap, hash::Hash};

pub struct RegisterAllocator {
    registers: HashMap<Register, Option<String>>,
    variables: HashMap<String, Register>
}

impl RegisterAllocator {
    pub fn new() -> Self {
        let registers = vec![
            Register::R8,
            Register::R9,
            Register::R10,
            Register::R11,
            Register::R12,
            Register::R13,
            Register::R14,
            Register::R15,
        ];

        let mut map = HashMap::new();

        for register in &registers {
            map.insert(*register, None);
        }

        Self { registers: map,variables : HashMap::new() }
    }

    pub fn allocate(&mut self, var : &String) -> Result<(), ()>
    {
        if self.variables.contains_key(var)
        {
            return Ok(());
        }

        let keys = self.registers.keys().cloned();
        for key in keys
        {
            if self.registers[&key].is_none()
            {
                self.registers.insert(key, Some(var.clone()));
                self.variables.insert(var.clone(), key);
                return Ok(());
            }
        }

        Err(())
    }

    pub fn get(&self, var : &String) -> Option<Register>
    {
        if !self.variables.contains_key(var)
        {
            return None;
        }

        Some(self.variables[var])
    }

    pub fn get_or_allocate(&mut self, var : &String) -> Option<Register>
    {
        let _ = self.allocate(var);
        let get = self.get(var);

        if get.is_some()
        {
            return get;
        }
        None
    }
}

#[repr(usize)]
#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub enum Register {
    AX,
    BX,
    CX,
    DX,
    SI,
    DI,
    SP,
    BP,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
}

impl Register {
    pub fn as_index(&self) -> usize {
        unsafe { std::mem::transmute(self) }
    }

    pub fn as_word(&self) -> String {
        match self {
            Register::AX => "AX".to_string(),
            Register::BX => "BX".to_string(),
            Register::CX => "CX".to_string(),
            Register::DX => "DX".to_string(),
            Register::SI => "SI".to_string(),
            Register::DI => "DI".to_string(),
            Register::SP => "SP".to_string(),
            Register::BP => "BP".to_string(),
            _ => format!("{}W", self.as_qword()),
        }
    }

    pub fn as_dword(&self) -> String {
        match self {
            Register::R8 => "R8D".to_string(),
            Register::R9 => "R9D".to_string(),
            Register::R10 => "R10D".to_string(),
            Register::R11 => "R11D".to_string(),
            Register::R12 => "R12D".to_string(),
            Register::R13 => "R13D".to_string(),
            Register::R14 => "R14D".to_string(),
            Register::R15 => "R15D".to_string(),
            _ => format!("E{}", self.as_word()),
        }
    }

    pub fn as_qword(&self) -> String {
        match self {
            Register::R8 => "R8".to_string(),
            Register::R9 => "R9".to_string(),
            Register::R10 => "R10".to_string(),
            Register::R11 => "R11".to_string(),
            Register::R12 => "R12".to_string(),
            Register::R13 => "R13".to_string(),
            Register::R14 => "R14".to_string(),
            Register::R15 => "R15".to_string(),
            _ => format!("E{}", self.as_word()),
        }
    }

    pub fn as_size(&self, size: &Size) -> String
    {
        match size
        {
            Size::Word => self.as_word(),
            Size::DoubleWord => self.as_dword(),
            Size::QuadWord =>self.as_qword(),
            _ => todo!()
        }
    }
}

pub struct Compiler {
    variables: HashMap<String, (u32, Size)>,
    registers: RegisterAllocator,
    current_offset: Vec<u32>,
}

impl Compiler {
    pub fn new_stack_frame(&mut self) {
        self.current_offset.push(0)
    }

    pub fn offset(&mut self) -> &mut u32 {
        if self.current_offset.len() == 0 {
            self.current_offset.push(0);
        }

        self.current_offset.last_mut().unwrap()
    }

    pub fn allocate_variable(&mut self, name: &String, size: &Size) -> ValueCodegen {
        if self.registers.allocate(name).is_ok()
        {
            return ValueCodegen::Register(self.registers.get(name).unwrap().as_size(size))
        }

        let off = *self.offset();

        self.variables.insert(name.clone(), (off, *size));

        let offset = self.offset();
        *offset += size.get_bytes() as u32;
        ValueCodegen::StackOffset(format!("{} [rbp-{}]", size.name(), offset))
    }

    pub fn get_or_allocate_variable(&mut self, name: &String, size: &Size) -> ValueCodegen {
        if let Some(reg) = self.registers.get_or_allocate(name)
        {
            return ValueCodegen::Register(reg.as_size(size));
        }

        if let Some((offset, other_size)) = self.variables.get(name) {
            if *other_size != *size {
                eprintln!("Mismatched Sizes for variable");
                panic!();
            }

            ValueCodegen::StackOffset(format!("{} [rbp-{}]", size.name(), offset))
        } else {
            self.allocate_variable(name, size)
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Size {
    Byte = 1,       // 8
    Word = 2,       // 16
    DoubleWord = 4, // 32
    QuadWord = 8,   // 64
}

impl Size {
    pub fn name(&self) -> String {
        match self {
            Size::Byte => "BYTE",
            Size::Word => "WORD",
            Size::DoubleWord => "DWORD",
            Size::QuadWord => "QWORD",
        }
        .to_string()
    }

    pub fn get_bytes(&self) -> u8 {
        match self {
            Size::Byte => 1,
            Size::Word => 2,
            Size::DoubleWord => 4,
            Size::QuadWord => 8,
        }
    }
}
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

#[derive(Debug, Clone)]
pub enum Value {
    Variable(Size, String),
    Int(String), // Store numerals as strings because we are directly compiling into AMD64
    StringLiteral(String),
    Null,
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum ValueCodegen
{
    Register(String),
    StackOffset(String),
    Number(String),
    StringLiteral(String)
}

impl ValueCodegen
{
    pub fn is_register(&self) -> bool
    {
        if let ValueCodegen::Register(_) = self {true} else {false}
    }

    pub fn is_stack(&self) -> bool
    {
        if let ValueCodegen::StackOffset(_) = self {true} else {false}
    }

    pub fn inner(&self) -> String
    {
        match self
        {
            ValueCodegen::Register(s) |
            ValueCodegen::StackOffset(s) |
            ValueCodegen::Number(s) |
            ValueCodegen::StringLiteral(s) => s.clone(),
        }
    }
}

impl Value {
    pub fn codegen(&self, compiler: &mut Compiler) -> ValueCodegen {
        match self {
            Value::Variable(size, ref name) => compiler.get_or_allocate_variable(name, size),
            Value::Int(num) => ValueCodegen::Number(num.clone()),
            Value::StringLiteral(literal) => ValueCodegen::StringLiteral(literal.clone()),
            _ => panic!()
        }
    }

    pub fn is_variable(&self) -> bool {
        match self {
            Value::Variable(_, _) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct IRStatement {
    op_type: OperandType,
    operand: Operand,
    lhs: Value,
    rhs: Option<Value>,
}

impl IRStatement {
    pub fn codegen(&self, compiler: &mut Compiler) -> String {
        self.operand
            .codegen(&self.lhs, &self.rhs, &self.op_type, compiler)
    }
}

#[derive(Debug, Clone)]
pub struct IRModule {
    statements: Vec<IRStatement>,
}

impl IRModule {
    pub fn compile(&self) -> String {
        let mut compiler = Compiler {
            variables: HashMap::new(),
            current_offset: vec![],
            registers: RegisterAllocator::new(),
        };
        let mut buffer = String::new();
        for statement in &self.statements {
            buffer.push_str(format!("{}\n", statement.codegen(&mut compiler)).as_str());
        }

        buffer
    }
}

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
