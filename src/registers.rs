
use crate::{Size, ValueCodegen};


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
    pub fn as_ptr(&self) -> ValueCodegen
    {
        ValueCodegen::Pointer(format!("QWORD [{}]", self.as_qword()))
    }

    pub fn as_deref(&self, size : &Size) -> ValueCodegen
    {
        ValueCodegen::Pointer(format!("{} [{}]", size.name(), self.as_qword()))
    }

    pub fn as_index(&self) -> usize {
        unsafe { std::mem::transmute(self) }
    }

    pub fn as_gen(&self, size : &Size) -> ValueCodegen
    {
        ValueCodegen::Register(self.as_size(size))
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
            _ => format!("R{}", self.as_word()),
        }
    }

    pub fn as_size(&self, size: &Size) -> String {
        match size {
            Size::Word => self.as_word(),
            Size::DoubleWord => self.as_dword(),
            Size::QuadWord => self.as_qword(),
            _ => todo!(),
        }
    }
}
