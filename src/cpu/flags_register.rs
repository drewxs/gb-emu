const ZERO_FLAG_BYTE_POSITION: u8 = 7;
const SUBTRACT_FLAG_BYTE_POSITION: u8 = 6;
const HALF_CARRY_FLAG_BYTE_POSITION: u8 = 5;
const CARRY_FLAG_BYTE_POSITION: u8 = 4;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FlagsRegister {
    pub zero: bool,
    pub subtract: bool,
    pub half_carry: bool,
    pub carry: bool,
}

impl FlagsRegister {
    pub fn new() -> Self {
        FlagsRegister {
            zero: false,
            subtract: false,
            half_carry: false,
            carry: false,
        }
    }
}

impl std::convert::From<FlagsRegister> for u8 {
    fn from(flags: FlagsRegister) -> Self {
        (if flags.zero { 1 } else { 0 }) << ZERO_FLAG_BYTE_POSITION
            | (if flags.subtract { 1 } else { 0 }) << SUBTRACT_FLAG_BYTE_POSITION
            | (if flags.half_carry { 1 } else { 0 }) << HALF_CARRY_FLAG_BYTE_POSITION
            | (if flags.carry { 1 } else { 0 }) << CARRY_FLAG_BYTE_POSITION
    }
}

impl std::convert::From<u8> for FlagsRegister {
    fn from(byte: u8) -> Self {
        FlagsRegister {
            zero: ((byte >> ZERO_FLAG_BYTE_POSITION) & 0b1) != 0,
            subtract: ((byte >> SUBTRACT_FLAG_BYTE_POSITION) & 0b1) != 0,
            half_carry: ((byte >> HALF_CARRY_FLAG_BYTE_POSITION) & 0b1) != 0,
            carry: ((byte >> CARRY_FLAG_BYTE_POSITION) & 0b1) != 0,
        }
    }
}
