#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum OpCode {
    Nop = 0,
    Dup,
    Pop,
    Jump,

    Ret,
    PrepareFuncCall,

    Add,
    Sub,
    Mul,
    Div,

    Le,
    Lt,
    Ge,
    Gt,
    Eq,
    Ne,

    UnaryNot,

    LoadConst,

    StoreName,
    LoadName,

    PopJumpIfFalse,

    StopCode,
}

impl OpCode {
    #[allow(clippy::match_same_arms)]
    #[must_use]
    pub fn size_operand(self) -> usize {
        match self {
            Self::Nop | Self::Dup | Self::Pop => 0,
            Self::PrepareFuncCall | Self::Ret => 0,
            Self::Add | Self::Sub | Self::Mul | Self::Div | Self::UnaryNot => 0,
            Self::Le | Self::Lt | Self::Ge | Self::Gt | Self::Eq | Self::Ne => 0,

            Self::LoadConst | Self::StoreName | Self::LoadName => 4,
            Self::PopJumpIfFalse | Self::Jump => 4,
            Self::StopCode => 0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct InvalidOpCode;

impl TryFrom<u8> for OpCode {
    type Error = InvalidOpCode;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value >= Self::StopCode as u8 {
            return Err(InvalidOpCode);
        }
        // # Safety:
        // OpCode is repr(u8) and value is guaranteed to be < OpCode's last variant.
        unsafe { std::mem::transmute(value) }
    }
}
