#[derive(PartialEq, Eq)]
pub enum Opcode {
    /// Stop program execution.
    Halt = 0,
    /// Push an integer to the stack.
    IConst = 1,
    /// Make a syscall.
    Syscall = 3,
    /// Jump if the top stack element is zero.
    JumpIfZero = 4,
    /// Add two integers.
    IAdd = 0x10,
    /// Multiply two integers.
    IMul = 0x11,
    /// Subtract an integer from another.
    ISub = 0x12,
    /// Divide an integer by another.
    IDiv = 0x13,
    /// Duplicate the top item.
    Dup = 0x20,
}

impl TryFrom<u32> for Opcode {
    type Error = String;

        fn try_from(code: u32) -> Result<Self, Self::Error> {
        Ok(match code {
            0 => Self::Halt,
            1 => Self::IConst,
            3 => Self::Syscall,
            4 => Self::JumpIfZero,
            0x10 => Self::IAdd,
            0x11 => Self::IMul,
            0x12 => Self::ISub,
            0x13 => Self::IDiv,
            0x20 => Self::Dup,
            _ => Err(format!("Unknown opcode {:x?}.", code))?
        })
    }
}

impl From<Opcode> for u32 {
    fn from(opcode: Opcode) -> u32 {
        match opcode {
            Opcode::Halt => 0,
            Opcode::IConst => 1,
            Opcode::Syscall => 3,
            Opcode::JumpIfZero => 4,
            Opcode::IAdd => 0x10,
            Opcode::IMul => 0x11,
            Opcode::ISub => 0x12,
            Opcode::IDiv => 0x13,
            Opcode::Dup => 0x20,
        }
    }
}
