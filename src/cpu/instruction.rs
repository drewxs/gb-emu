#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ArithmeticTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ADDHLTarget {
    BC,
    DE,
    HL,
    SP,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum IncDecTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    BC,
    DE,
    HL,
    SP,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Instruction {
    INC(IncDecTarget), // increment
    DEC(IncDecTarget), // decrement

    ADD(ArithmeticTarget), // add
    ADDHL(ADDHLTarget),    // add to HL
    ADC(ArithmeticTarget), // add with carry
    SUB(ArithmeticTarget), // subtract
    SBC(ArithmeticTarget), // subtract with carry

    AND(ArithmeticTarget), // logical AND
    OR(ArithmeticTarget),  // logical OR
    XOR(ArithmeticTarget), // logical XOR
    CP(ArithmeticTarget),  // compare

    CCF(ArithmeticTarget), // complement carry flag
    SCF(ArithmeticTarget), // set carry flag

    RRA(ArithmeticTarget),  // rotate right A register
    RLA(ArithmeticTarget),  // rotate left A register
    RRCA(ArithmeticTarget), // rotate right A register (no carry)
    RRLA(ArithmeticTarget), // rotate left A register (no carry)
    CPL(ArithmeticTarget),  // complement

    BIT(ArithmeticTarget),   // bit test
    RESET(ArithmeticTarget), // bit reset
    SET(ArithmeticTarget),   // bit set

    SRL(ArithmeticTarget), // shift right logical
    SLL(ArithmeticTarget), // shift left logical
    SRA(ArithmeticTarget), // shift right arithmetic
    SLA(ArithmeticTarget), // shift left arithmetic

    RR(ArithmeticTarget),  // rotate right
    RL(ArithmeticTarget),  // rotate left
    RRC(ArithmeticTarget), // rotate right (no carry)
    RLC(ArithmeticTarget), // rotate left (no carry)

    SWAP(ArithmeticTarget), // swap nibbles
}

impl Instruction {
    pub fn from_byte(byte: u8) -> Option<Self> {
        match byte {
            0x3c => Some(Instruction::INC(IncDecTarget::A)),
            0x04 => Some(Instruction::INC(IncDecTarget::B)),
            0x14 => Some(Instruction::INC(IncDecTarget::D)),
            0x24 => Some(Instruction::INC(IncDecTarget::H)),
            0x0c => Some(Instruction::INC(IncDecTarget::C)),
            0x1c => Some(Instruction::INC(IncDecTarget::E)),
            0x2c => Some(Instruction::INC(IncDecTarget::L)),
            0x03 => Some(Instruction::INC(IncDecTarget::BC)),
            0x13 => Some(Instruction::INC(IncDecTarget::DE)),
            0x23 => Some(Instruction::INC(IncDecTarget::HL)),
            0x33 => Some(Instruction::INC(IncDecTarget::SP)),

            0x3d => Some(Instruction::DEC(IncDecTarget::A)),
            0x05 => Some(Instruction::DEC(IncDecTarget::B)),
            0x0d => Some(Instruction::DEC(IncDecTarget::C)),
            0x15 => Some(Instruction::DEC(IncDecTarget::D)),
            0x1d => Some(Instruction::DEC(IncDecTarget::E)),
            0x25 => Some(Instruction::DEC(IncDecTarget::H)),
            0x2d => Some(Instruction::DEC(IncDecTarget::L)),
            0x0b => Some(Instruction::DEC(IncDecTarget::BC)),
            0x1b => Some(Instruction::DEC(IncDecTarget::DE)),
            0x2b => Some(Instruction::DEC(IncDecTarget::HL)),
            0x3b => Some(Instruction::DEC(IncDecTarget::SP)),

            0x87 => Some(Instruction::ADD(ArithmeticTarget::A)),
            0x80 => Some(Instruction::ADD(ArithmeticTarget::B)),
            0x81 => Some(Instruction::ADD(ArithmeticTarget::C)),
            0x82 => Some(Instruction::ADD(ArithmeticTarget::D)),
            0x83 => Some(Instruction::ADD(ArithmeticTarget::E)),
            0x84 => Some(Instruction::ADD(ArithmeticTarget::H)),
            0x85 => Some(Instruction::ADD(ArithmeticTarget::L)),

            0x09 => Some(Instruction::ADDHL(ADDHLTarget::BC)),
            0x19 => Some(Instruction::ADDHL(ADDHLTarget::DE)),
            0x29 => Some(Instruction::ADDHL(ADDHLTarget::HL)),
            0x39 => Some(Instruction::ADDHL(ADDHLTarget::SP)),

            _ => None,
        }
    }
}
