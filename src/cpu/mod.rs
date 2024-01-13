pub mod flags_register;
pub mod instruction;
pub mod memory_bus;
pub mod registers;
pub mod targets;

use self::instruction::Instruction;
use self::memory_bus::MemoryBus;
use self::registers::Registers;
use self::targets::{
    ADDHLTarget, ArithmeticTarget, IncDecTarget, JumpTest, LoadByteSource, LoadByteTarget,
    LoadType, StackTarget,
};

#[derive(Copy, Clone, Debug, PartialEq)]
struct CPU {
    pub registers: Registers,
    pc: u16, // program counter
    sp: u16, // stack pointer
    bus: MemoryBus,
    is_halted: bool,
}

#[allow(dead_code)]
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
        if self.is_halted {
            return self.pc;
        }
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
                let jump_condition = self.should_jump(test);
                self.jump(jump_condition)
            }
            Instruction::LD(load_type) => match load_type {
                LoadType::Byte(target, source) => {
                    let source_value = match source {
                        LoadByteSource::A => self.registers.a,
                        LoadByteSource::D8 => self.read_next_byte(),
                        LoadByteSource::HLI => self.bus.read_byte(self.registers.get_hl()),
                        _ => panic!("TODO: implement other sources"),
                    };
                    match target {
                        LoadByteTarget::A => self.registers.a = source_value,
                        LoadByteTarget::HLI => {
                            self.bus.write_byte(self.registers.get_hl(), source_value)
                        }
                        _ => panic!("TODO: implement other targets"),
                    };
                    match source {
                        LoadByteSource::D8 => self.pc.wrapping_add(2),
                        _ => self.pc.wrapping_add(1),
                    }
                }
            },
            Instruction::PUSH(target) => {
                let value = match target {
                    StackTarget::BC => self.registers.get_bc(),
                    StackTarget::DE => self.registers.get_de(),
                    StackTarget::HL => self.registers.get_hl(),
                    StackTarget::AF => self.registers.get_af(),
                };
                self.push(value);
                self.pc.wrapping_add(1)
            }
            Instruction::POP(target) => {
                let result = self.pop();
                match target {
                    StackTarget::BC => self.registers.set_bc(result),
                    StackTarget::DE => self.registers.set_de(result),
                    StackTarget::HL => self.registers.set_hl(result),
                    StackTarget::AF => self.registers.set_af(result),
                };
                self.pc.wrapping_add(1)
            }
            Instruction::CALL(test) => {
                let jump_condition = self.should_jump(test);
                self.call(jump_condition)
            }
            Instruction::RET(test) => {
                let jump_condition = self.should_jump(test);
                self.return_(jump_condition)
            }
            Instruction::NOP => self.pc.wrapping_add(1),
            Instruction::HALT => {
                self.is_halted = true;
                self.pc
            }
            _ => self.pc,
        }
    }

    pub fn read_next_byte(&self) -> u8 {
        self.bus.read_byte(self.pc + 1)
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

    pub fn should_jump(&self, test: JumpTest) -> bool {
        match test {
            JumpTest::NotZero => !self.registers.f.zero,
            JumpTest::NotCarry => !self.registers.f.carry,
            JumpTest::Zero => self.registers.f.zero,
            JumpTest::Carry => self.registers.f.carry,
            JumpTest::Always => true,
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

    pub fn push(&mut self, value: u16) {
        self.sp = self.sp.wrapping_sub(1);
        self.bus.write_byte(self.sp, ((value & 0xFF00) >> 8) as u8);

        self.sp = self.sp.wrapping_sub(1);
        self.bus.write_byte(self.sp, (value & 0x00FF) as u8);
    }

    pub fn pop(&mut self) -> u16 {
        let lsb = self.bus.read_byte(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);

        let msb = self.bus.read_byte(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);

        (msb << 8) | lsb
    }

    pub fn read_next_word(&mut self) -> u16 {
        // Gameboy is little endian, so pc+2->MSB & pc+1->LSB
        ((self.bus.read_byte(self.pc + 2) as u16) << 8) | (self.bus.read_byte(self.pc + 1) as u16)
    }

    pub fn call(&mut self, should_jump: bool) -> u16 {
        let next_pc = self.pc.wrapping_add(3);
        if should_jump {
            self.push(next_pc);
            self.read_next_word()
        } else {
            next_pc
        }
    }

    pub fn return_(&mut self, should_jump: bool) -> u16 {
        if should_jump {
            self.pop()
        } else {
            self.pc.wrapping_add(1)
        }
    }
}
