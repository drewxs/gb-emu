pub mod flags_register;
pub mod instruction;
pub mod memory_bus;
pub mod registers;

use self::instruction::{ADDHLTarget, ArithmeticTarget, IncDecTarget, Instruction, JumpTest};
use self::memory_bus::MemoryBus;
use self::registers::Registers;

#[derive(Copy, Clone, Debug, PartialEq)]
struct CPU {
    pub registers: Registers,
    pc: u16, // program counter
    sp: u16, // stack pointer
    bus: MemoryBus,
}

impl CPU {
    pub fn step(&mut self) {
        let mut instruction_byte = self.bus.read_byte(self.pc);
        let prefixed = instruction_byte == 0xCB;
        if prefixed {
            instruction_byte = self.bus.read_byte(self.pc + 1);
        }

        if let Some(instruction) = Instruction::from_byte(instruction_byte) {
            let next_pc = self.execute(instruction);
            self.pc = next_pc;
        }
    }

    pub fn execute(&mut self, instruction: Instruction) -> u16 {
        match instruction {
            Instruction::INC(register) => match register {
                IncDecTarget::A => {
                    self.registers.a = self.registers.a.wrapping_add(1);
                    self.pc
                }
                IncDecTarget::B => {
                    self.registers.b = self.registers.b.wrapping_add(1);
                    self.pc
                }
                IncDecTarget::C => {
                    self.registers.c = self.registers.c.wrapping_add(1);
                    self.pc
                }
                IncDecTarget::D => {
                    self.registers.d = self.registers.d.wrapping_add(1);
                    self.pc
                }
                IncDecTarget::E => {
                    self.registers.e = self.registers.e.wrapping_add(1);
                    self.pc
                }
                IncDecTarget::H => {
                    self.registers.h = self.registers.h.wrapping_add(1);
                    self.pc
                }
                IncDecTarget::L => {
                    self.registers.l = self.registers.l.wrapping_add(1);
                    self.pc
                }
                IncDecTarget::BC => {
                    let value = self.registers.get_bc();
                    let new_value = value.wrapping_add(1);
                    self.registers.set_bc(new_value);
                    self.pc
                }
                IncDecTarget::DE => {
                    let value = self.registers.get_de();
                    let new_value = value.wrapping_add(1);
                    self.registers.set_de(new_value);
                    self.pc
                }
                IncDecTarget::HL => {
                    let value = self.registers.get_hl();
                    let new_value = value.wrapping_add(1);
                    self.registers.set_hl(new_value);
                    self.pc
                }
                IncDecTarget::SP => {
                    self.sp = self.sp.wrapping_add(1);
                    self.pc
                }
            },
            Instruction::DEC(register) => match register {
                IncDecTarget::A => {
                    self.registers.a = self.registers.a.wrapping_sub(1);
                    self.pc
                }
                IncDecTarget::B => {
                    self.registers.b = self.registers.b.wrapping_sub(1);
                    self.pc
                }
                IncDecTarget::C => {
                    self.registers.c = self.registers.c.wrapping_sub(1);
                    self.pc
                }
                IncDecTarget::D => {
                    self.registers.d = self.registers.d.wrapping_sub(1);
                    self.pc
                }
                IncDecTarget::E => {
                    self.registers.e = self.registers.e.wrapping_sub(1);
                    self.pc
                }
                IncDecTarget::H => {
                    self.registers.h = self.registers.h.wrapping_sub(1);
                    self.pc
                }
                IncDecTarget::L => {
                    self.registers.l = self.registers.l.wrapping_sub(1);
                    self.pc
                }
                IncDecTarget::BC => {
                    let value = self.registers.get_bc();
                    let new_value = value.wrapping_sub(1);
                    self.registers.set_bc(new_value);
                    self.pc
                }
                IncDecTarget::DE => {
                    let value = self.registers.get_de();
                    let new_value = value.wrapping_sub(1);
                    self.registers.set_de(new_value);
                    self.pc
                }
                IncDecTarget::HL => {
                    let value = self.registers.get_hl();
                    let new_value = value.wrapping_sub(1);
                    self.registers.set_hl(new_value);
                    self.pc
                }
                IncDecTarget::SP => {
                    self.sp = self.sp.wrapping_sub(1);
                    self.pc
                }
            },
            Instruction::ADD(register) => match register {
                ArithmeticTarget::C => {
                    self.registers.a = self.add(self.registers.c);
                    self.pc
                }
                _ => self.pc,
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
                self.pc
            }
            Instruction::JP(test) => {
                let jump_condition = match test {
                    JumpTest::NotZero => !self.registers.f.zero,
                    JumpTest::NotCarry => !self.registers.f.carry,
                    JumpTest::Zero => self.registers.f.zero,
                    JumpTest::Carry => self.registers.f.carry,
                    JumpTest::Always => true,
                };
                self.jump(jump_condition)
            }
            _ => self.pc,
        }
    }

    pub fn jump(&self, should_jump: bool) -> u16 {
        if should_jump {
            let least_significant_byte = self.bus.read_byte(self.pc + 1) as u16;
            let most_significant_byte = self.bus.read_byte(self.pc + 2) as u16;
            (most_significant_byte << 8) | least_significant_byte
        } else {
            self.pc.wrapping_add(3)
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
