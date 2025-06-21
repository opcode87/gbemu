pub struct Emulator {
    pub cpu: CPU,
    pub memory: Memory,
    pub ppu: PPU,
    pub timer: Timer,
    pub input: InputController,
    pub cycles: u64,
}

