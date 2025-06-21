enum Instruction {
    NOP,

    ADD(ArithmeticTarget),
    ADC(ArithmeticTarget),

    LD(LoadType),
}

impl Instruction {
    fn from_byte(byte: u8, prefixed: bool) -> Option<Instruction> {
        
        // non-prefix instructions
        if !prefixed {
            match byte {
                0x02 => Some(Instruction::INC(IncDecTarget::BC)),
                0x13 => Some(Instruction::INC(IncDecTarget::DE)),
            }
        }
        // prefixed instructions
        else { 

        }
    }
}

enum IncDecTarget {
    AF, BC, DE, HL, // ? (check pandocs)
}

enum ArithmeticTarget {
    A, B, C, D, E, H, L,
}

enum LoadTarget {
    A, B, C, D, E, H, L, HLI,
}

enum LoadSource {
    A, B, C, D, E, H, L, D8, HLI,
}

enum LoadType {
    Byte(LoadTarget, LoadSource),
}

