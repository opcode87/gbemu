mod registers;
mod instructions;

use registers::Registers;
use instructions::Instruction;

impl CPU {
    fn execute(&mut self, instruction: Instruction) {
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
            }

            // 0x41
            Instruction::LD(target) => {   
            }
        }
    }


     // LD B, C
     // B = C
    fn ld() -> u8 {

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
