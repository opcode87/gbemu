enum Instruction {
    ADD(ArithmeticTarget),
    ADC(ArithmeticTarget),
    LD(LoadType),
}

enum ArithmeticTarget {
    A, B, C, D, E, H, L,
}

enum LoadType {
    Byte(LoadByteTarget, LoadByteSource),
    Word(LoadWordTarget),
    AFromIndirect(Indirect),
    IndirectFromA(Indirect),
    AFromByteAddress,
    ByteAddressFromA,
    SPFromHL,
    HLFromSPN,
    IndirectFromSP,
}
