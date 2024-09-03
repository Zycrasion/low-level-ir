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