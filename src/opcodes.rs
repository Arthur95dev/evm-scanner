#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Opcode {
    // Арифметика
    STOP = 0x00,
    ADD = 0x01,
    MUL = 0x02,

    // Опасные / интересные для безопасности
    CREATE = 0xf0,
    CALL = 0xf1,
    CALLCODE = 0xf2,
    RETURN = 0xf3,
    DELEGATECALL = 0xf4,
    CREATE2 = 0xf5,
    STATICCALL = 0xfa,
    REVERT = 0xfd,
    SELFDESTRUCT = 0xff,
}

impl Opcode {
    pub fn from_byte(b: u8) -> Option<Self> {
        match b {
            0x00 => Some(Self::STOP),
            0x01 => Some(Self::ADD),
            0x02 => Some(Self::MUL),
            0xf0 => Some(Self::CREATE),
            0xf1 => Some(Self::CALL),
            0xf2 => Some(Self::CALLCODE),
            0xf3 => Some(Self::RETURN),
            0xf4 => Some(Self::DELEGATECALL),
            0xf5 => Some(Self::CREATE2),
            0xfa => Some(Self::STATICCALL),
            0xfd => Some(Self::REVERT),
            0xff => Some(Self::SELFDESTRUCT),
            _ => None,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Self::STOP => "STOP",
            Self::ADD => "ADD",
            Self::MUL => "MUL",
            Self::CREATE => "CREATE",
            Self::CALL => "CALL",
            Self::CALLCODE => "CALLCODE",
            Self::RETURN => "RETURN",
            Self::DELEGATECALL => "DELEGATECALL",
            Self::CREATE2 => "CREATE2",
            Self::STATICCALL => "STATICCALL",
            Self::REVERT => "REVERT",
            Self::SELFDESTRUCT => "SELFDESTRUCT",
        }
    }

    pub fn is_dangerous(self) -> bool {
        matches!(
            self,
            Self::CALL | Self::DELEGATECALL | Self::CALLCODE | Self::SELFDESTRUCT
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    Op(Opcode),
    Push(u8),
}

impl Instruction {
    /// Распарсить следующий опкод из байта в байткоде.
    pub fn from_byte(b: u8) -> Self {
        if (0x60..=0x7f).contains(&b) {
            Instruction::Push(b - 0x60 + 1) // 1..32 байт данных
        } else {
            // Если нераспознанный опкод – считаем его пустой Instruction без имени
            Instruction::Op(Opcode::from_byte(b).unwrap_or(Opcode::STOP))
        }
    }

    /// Полный размер инструкции в байтах (опкод + данные для PUSH)
    pub fn size(self) -> usize {
        match self {
            Instruction::Op(_) => 1,
            Instruction::Push(n) => 1 + n as usize,
        }
    }
}
