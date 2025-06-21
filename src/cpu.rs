mod registers;
mod instructions;

use registers::Registers;
use instructions::Instruction;

pub struct CPU {
    pub registers: Registers,
    pc: u16,
    bus: MemoryBus,
}

struct MemoryBus {
    memory: [u8; 0xFFFF]
}

impl MemoryBus {
    fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }
}

impl CPU {

    fn step(&mut self) {
        let mut instruction_byte = self.bus.read_byte(self.pc);

        /* if it is prefixed the instructions r different is dependent on the byte after (pc+1) */
        let prefixed = instruction_byte == 0xCB;

        if prefixed {
            instruction_byte = self.bus.read_byte(self.pc + 1);
        }

        let next_pc = if let Some(instruction) = Instruction::from_byte(instruction_byte, prefixed) {
            self.execute(instruction)
        } else {
            panic!("unknown instruction found:", instruction_byte)
        };

        self.pc = next_pc;
    }

    fn execute(&mut self, instruction: Instruction) -> u16 {
        match instruction {

            // 0x80
            Instruction::ADD(target) => { 
                let value = match target {
                    ArithmeticTarget::A => self.registers.a,
                    ArithmeticTarget::B => self.registers.b,
                    ArithmeticTarget::C => self.registers.c,
                    ArithmeticTarget::D => self.registers.d,
                    ArithmeticTarget::E => self.registers.e,
                    ArithmeticTarget::H => self.registers.h,
                    ArithmeticTarget::L => self.registers.l,
                };
                self.registers.a = self.add(value);
                self.pc.wrapping_add(1)
            } 
            
            // 0x88
            Instruction::ADC(target) => {
                let value = match target {
                    ArithmeticTarget::A => self.registers.a,
                    ArithmeticTarget::B => self.registers.b,
                    ArithmeticTarget::C => self.registers.c,
                    ArithmeticTarget::D => self.registers.d,
                    ArithmeticTarget::E => self.registers.e,
                    ArithmeticTarget::H => self.registers.h,
                    ArithmeticTarget::L => self.registers.l,
                };
                self.registers.a = self.adc(value);
                self.pc.wrapping_add(1)
            }

            // 0x41
            Instruction::LD(load_type) => {
                match load_type {
                    LoadType::Byte(target, source) => {
                        let source_value = match source {
                            LoadSource::A => self.registers.a = source_value,
                            LoadTarget::HLI => self.bus.write_byte(self.registers.get_hl(), source_value),
                        }
                    }
                }
            }

        }
    }


    fn adc(&mut self, value: u8) -> u8 {
        self.add_internal(value, true)
    }

    fn add(&mut self, value: u8) -> u8 {
        self.add_internal(value, false)
    }
    
    fn add_internal(&mut self, value: u8, add_carry: bool) -> u8 {
        
        let additional_carry = if add_carry && self.registers.f.carry {
            1
        } else {
            0
        };
        
        let (add, carry) = self.registers.a.overflowing_add(value);
        let (add2, carry2) = add.overflowing_add(additional_carry);

        self.registers.f.zero = add2 == 0;
        self.registers.f.subtract = false;
        self.registers.f.carry = carry || carry2;

        self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;

        add2
    }
}
