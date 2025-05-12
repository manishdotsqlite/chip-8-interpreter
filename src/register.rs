pub struct Register {
        pub register: u8
}

pub struct General_Purpose_Registers {
        registers: [Register; 16]
}

impl General_Purpose_Registers {

        pub fn new() -> Self {
                General_Purpose_Registers {
                        registers: core::array::from_fn(|_| Register { register: 0 })
                }
        }

        pub fn fetch_register(&self, index: usize) -> u8 {
                self.registers[index].register
        }

        pub fn set_register(&mut self, index: usize, value: u8) {
                self.registers[index].register = value;
        }

        pub fn display_register(&self, index: usize) {
                println!("V{:X} = {:#X}", index, self.registers[index].register);
        }
    
}

pub struct Instruction_Register {
        pub register: u16
}

impl Instruction_Register {
        pub fn new() -> Self {
                Instruction_Register {
                        register: 0
                }
        }

        pub fn set_register(&mut self, value: u16) {
                self.register = value;
        }

        pub fn fetch_register(&self) -> u16 {
                self.register
        }
}

pub struct Program_Counter {
        pub counter: usize 
}

impl Program_Counter {
        pub fn increment(&mut self) {
                self.counter += 2;
        }

        pub fn goto(&mut self, location: usize) {
                self.counter = location;
        }
}


