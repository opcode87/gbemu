const ZERO_FLAG_POS: u8 = 7;
const SUBTRACT_FLAG_POS: u8 = 6;
const HALF_CARRY_FLAG_POS: u8 = 5;
const CARRY_FLAG_POS: u8 = 4;

impl std::convert::From<Flags> for u8 {
    fn from(flag: Flags) -> u8 {
        (if flag.zero { 1 } else { 0 }) << ZERO_FLAG_POS |
        (if flag.subtract { 1 } else { 0 }) << SUBTRACT_FLAG_POS |
        (if flag.half_carry { 1 } else { 0 }) << HALF_CARRY_FLAG_POS |
        (if flag.carry { 1 } else { 0 }) << CARRY_FLAG_POS
    }
}

impl std::convert::From<u8> for Flags {
    fn from(byte: u8) -> Self {
        let zero = ((byte >> ZERO_FLAG_POS) & 0b1) != 0;
        let subtract = ((byte >> SUBTRACT_FLAG_POS) & 0b1) != 0;
        let half_carry = ((byte >> HALF_CARRY_FLAG_POS) & 0b1) != 0;
        let carry = ((byte >> CARRY_FLAG_POS) & 0b1) != 0;

        Flags {
            zero,
            subtract,
            half_carry,
            carry
        }
    }
}

struct Registers 
{

    // af, bc, de, hl

    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
    h: u8,
    l: u8,
}

// flags (f register)
struct Flags {
    zero: bool,
    subtract: bool,
    half_carry: bool,
    carry: bool,
}

// registers can be accessed as either one 16-bit register or as two 8-bit registers.

fn split_u16(value: u16) -> (u8, u8) {
    (((value >> 8) & 0xFF) as u8, (value & 0xFF) as u8)
}
fn combine_u16(hi: u8, lo: u8) -> u16 {
    ((hi as u16) << 8) | (lo as u16)
}

impl Registers {
    
    // af register
    fn get_af(&self) -> u16 {
        return combine_u16(self.a, self.f);
    }

    fn set_af(&mut self, value: u16) {
        let (hi, lo) = split_u16(value)
        self.a = hi;
        self.f = lo;
    }

    fn flags()

    // bc register
    fn get_bc(&self) -> u16 {
        return combine_u16(self.b, self.c);
    }

    fn set_bc(&mut self, value: u16) {
        let (hi, lo) = split_u16(value)
        self.b = hi;
        self.c = lo;
    }
    
    // de register
    fn get_de(&self) -> u16 {
        return combine_u16(self.ad, self.e);
    }

    fn set_de(&mut self, value: u16) {
        let (hi, lo) = split_u16(value)
        self.d = hi;
        self.e = lo;
        
    }

    // hl register
    fn get_hl(&self) -> u16 {
        return combine_u16(self.h, self.l);
    }

    fn set_hl(&mut self, value: u16) {
        let (hi, lo) = split_u16(value)
        self.h = hi;
        self.l = lo;
        
    }
}