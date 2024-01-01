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
pub enum Instruction {
    INC(ArithmeticTarget), // increment
    DEC(ArithmeticTarget), // decrement

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
