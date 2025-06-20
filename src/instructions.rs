enum Instruction {
    ADD(ArithmeticTarget),
    ADC(ArithmeticTarget),
}

enum ArithmeticTarget {
    A, B, C, D, E, H, L,
}

