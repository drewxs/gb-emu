pub mod flags_register;
pub mod instruction;
pub mod registers;

use self::instruction::{ADDHLTarget, ArithmeticTarget, Instruction};
use self::registers::Registers;

#[derive(Copy, Clone, Debug, PartialEq)]
struct CPU {
    pub registers: Registers,
    sp: u16,
}

impl CPU {
    pub fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::ADD(register) => match register {
                ArithmeticTarget::C => {
                    let value = self.registers.c;
                    let new_value = self.add(value);
                    self.registers.a = new_value;
                }
                _ => {}
            },
            Instruction::ADDHL(register) => {
                let value = match register {
                    ADDHLTarget::BC => self.registers.get_bc(),
                    ADDHLTarget::DE => self.registers.get_de(),
                    ADDHLTarget::HL => self.registers.get_hl(),
                    ADDHLTarget::SP => self.sp,
                };
                let res = self.add_hl(value);
                self.registers.set_hl(res);
            }
            _ => {}
        }
    }

    pub fn add(&mut self, value: u8) -> u8 {
        let (res, carry) = self.registers.a.overflowing_add(value);

        self.registers.f.zero = res == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = carry;
        self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;

        res
    }

    pub fn add_hl(&mut self, value: u16) -> u16 {
        let hl = self.registers.get_hl();
        let (res, carry) = hl.overflowing_add(value);

        self.registers.f.subtract = false;
        self.registers.f.carry = carry;
        let mask = 0b111_1111_1111;
        self.registers.f.half_carry = (value & mask) + (hl & mask) > mask;

        res
    }
}
