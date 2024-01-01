pub mod flags_register;
pub mod instruction;
pub mod registers;

use instruction::{ArithmeticTarget, Instruction};
use registers::Registers;

#[derive(Copy, Clone, Debug, PartialEq)]
struct CPU {
    pub registers: Registers,
}

impl CPU {
    pub fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::ADD(target) => match target {
                ArithmeticTarget::C => {
                    let value = self.registers.c;
                    let new_value = self.add(value);
                    self.registers.a = new_value;
                }
                _ => {}
            },
            _ => {}
        }
    }

    pub fn add(&mut self, value: u8) -> u8 {
        let (new_value, overflow) = self.registers.a.overflowing_add(value);

        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = overflow;
        self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;

        new_value
    }
}
