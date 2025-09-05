#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
pub enum ASMBinaryInstruction {
    Mov,
    Add,
    AddS,
    Sub,
    SubS,
    Mul,
    MulS,
    Div,
    DivS,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
pub enum ASMUnaryInstruction {
    Inc,
    IncS,
    Dec,
    DecS,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
pub enum ASMJumpInstruction {
    Jmp,
    Jz,
    Jnz,
    Jc,
    Jnc,
    Js,
    Jns,
    Jg,
    Jl,
    Jge,
    Jle,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
pub enum ASMInstruction {
    Nop,
    Unary(ASMUnaryInstruction),
    Binary(ASMBinaryInstruction),
    Jump(ASMJumpInstruction),
}

impl TryFrom<&str> for ASMInstruction {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let inst = match value {
            "NOP" => Self::Nop,
            "MOV" => Self::Binary(ASMBinaryInstruction::Mov),
            "ADD" => Self::Binary(ASMBinaryInstruction::Add),
            "ADDS" => Self::Binary(ASMBinaryInstruction::AddS),
            "SUB" => Self::Binary(ASMBinaryInstruction::Sub),
            "SUBS" => Self::Binary(ASMBinaryInstruction::SubS),
            "MUL" => Self::Binary(ASMBinaryInstruction::Mul),
            "MULS" => Self::Binary(ASMBinaryInstruction::MulS),
            "DIV" => Self::Binary(ASMBinaryInstruction::Div),
            "DIVS" => Self::Binary(ASMBinaryInstruction::DivS),
            "INC" => Self::Unary(ASMUnaryInstruction::Inc),
            "INCS" => Self::Unary(ASMUnaryInstruction::IncS),
            "DEC" => Self::Unary(ASMUnaryInstruction::Dec),
            "DECS" => Self::Unary(ASMUnaryInstruction::DecS),
            "JMP" => Self::Jump(ASMJumpInstruction::Jmp),
            "JZ" => Self::Jump(ASMJumpInstruction::Jz),
            "JNZ" => Self::Jump(ASMJumpInstruction::Jnz),
            "JC" => Self::Jump(ASMJumpInstruction::Jc),
            "JNC" => Self::Jump(ASMJumpInstruction::Jnc),
            "JS" => Self::Jump(ASMJumpInstruction::Js),
            "JNS" => Self::Jump(ASMJumpInstruction::Jns),
            "JG" => Self::Jump(ASMJumpInstruction::Jg),
            "JL" => Self::Jump(ASMJumpInstruction::Jl),
            "JGE" => Self::Jump(ASMJumpInstruction::Jge),
            "JLE" => Self::Jump(ASMJumpInstruction::Jle),
            _ => return Err(()),
        };

        Ok(inst)
    }
}
