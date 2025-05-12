struct Register {
        register: u8
}

struct General_Purpose_Registers {
        registers: [Register; 16]
}

struct Instruction_Register {
        register: Register
}

struct Flag_Register {
        register: Register
}

struct Delay_Timer {
        timer: Register
}

struct Sound_Timer {
        timer: Register
}

struct Program_Counter {
        counter: u16
}

struct Stack_Pointer {
        pointer: Register
}
