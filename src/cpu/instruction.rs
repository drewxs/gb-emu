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
pub enum Instruction {
    ADD(ArithmeticTarget),   // add
    ADDHL(ArithmeticTarget), // add to HL
    ADC(ArithmeticTarget),   // add with carry
    SUB(ArithmeticTarget),   // subtract
    SBC(ArithmeticTarget),   // subtract with carry
    AND(ArithmeticTarget),   // logical AND
    OR(ArithmeticTarget),    // logical OR
    XOR(ArithmeticTarget),   // logical XOR
    CP(ArithmeticTarget),    // compare
    INC(ArithmeticTarget),   // increment
    DEC(ArithmeticTarget),   // decrement
    CCF(ArithmeticTarget),   // complement carry flag
    SCF(ArithmeticTarget),   // set carry flag
    RRA(ArithmeticTarget),   // rotate right A register
    RLA(ArithmeticTarget),   // rotate left A register
    RRCA(ArithmeticTarget),  // rotate right A register (no carry)
    RRLA(ArithmeticTarget),  // rotate left A register (no carry)
    CPL(ArithmeticTarget),   // complement
    BIT(ArithmeticTarget),   // bit test
    RESET(ArithmeticTarget), // bit reset
    SET(ArithmeticTarget),   // bit set
    SRL(ArithmeticTarget),   // shift right logical
    RR(ArithmeticTarget),    // rotate right
    RL(ArithmeticTarget),    // rotate left
    RRC(ArithmeticTarget),   // rotate right (no carry)
    RLC(ArithmeticTarget),   // rotate left (no carry)
    SRA(ArithmeticTarget),   // shift right arithmetic
    SLA(ArithmeticTarget),   // shift left arithmetic
    SWAP(ArithmeticTarget),  // swap nibbles
}
