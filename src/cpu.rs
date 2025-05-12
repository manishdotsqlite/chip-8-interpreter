use crate::{compiler::compiler, instruction::{ self, Instruction, Operation}, memory::Memory, register::{ General_Purpose_Registers, Instruction_Register, Program_Counter}, stack::Stack };
use std::{thread::sleep, time::Duration};

pub struct CPU {

        program_counter: Program_Counter,
        memory: Memory,

        general_purpose_registers: General_Purpose_Registers,
        instruction_register: Instruction_Register,

        stack: Stack,
}

impl CPU {

        pub fn new() -> Self {
                CPU {
                        program_counter: Program_Counter { counter: 0x200 },
                        memory: Memory::new(),
                        general_purpose_registers: General_Purpose_Registers::new(),
                        instruction_register: Instruction_Register::new(),
                        stack: Stack::new(),
                }
        }

        pub fn execute_instruction(&mut self, instruction: Instruction) {
                match instruction.operation {
                        Operation::CLS => {
                                println!("Clearing the screen");
                                sleep(Duration::from_millis(1000));
                                print!("\x1B[2J\x1B[H");
                        },
                        Operation::RET => {
                                println!("Returning from subroutine");
                                sleep(Duration::from_millis(1000));
                                self.program_counter.goto(self.stack.pointer.pointer as usize);
                        },
                        Operation::JMP(nnn) => {
                                println!("Jumping to address: {:#X}", nnn);
                                sleep(Duration::from_millis(1000));
                                self.program_counter.goto(nnn as usize);
                        }, 
                        Operation::CALL(nnn) => {
                                println!("Calling subroutine at address: {:#X}", nnn);
                                sleep(Duration::from_millis(1000));
                                self.stack.push(self.program_counter.counter as u16);
                                self.program_counter.goto(nnn as usize);
                        },
                        Operation::SE(x, kk) => {
                                println!("Skipping next instruction if V{:X} == {:#X}", x, kk);
                                sleep(Duration::from_millis(1000));
                                let value = self.general_purpose_registers.fetch_register(x as usize);
                                if value == kk {
                                        self.program_counter.increment();
                                }
                        },
                        Operation::SNE(x, kk) => {
                                println!("Skipping next instruction if V{:X} != {:#X}", x, kk);
                                sleep(Duration::from_millis(1000));
                                let value = self.general_purpose_registers.fetch_register(x as usize);
                                if value != kk {
                                        self.program_counter.increment();
                                }
                        },
                        Operation::SRE(x, y) => {
                                println!("Skipping next instruction if V{:X} == V{:X}", x, y);
                                sleep(Duration::from_millis(1000));
                                let value_x = self.general_purpose_registers.fetch_register(x as usize);
                                let value_y = self.general_purpose_registers.fetch_register(y as usize);
                                if value_x == value_y {
                                        self.program_counter.increment();
                                }
                        },
                        Operation::LD(x, kk) => {
                                println!("Loading {:#X} into V{:X}", kk, x);
                                sleep(Duration::from_millis(1000));
                                self.general_purpose_registers.set_register(x as usize, kk);
                        },
                        Operation::AD(x, kk) => {
                                println!("Adding {:#X} to V{:X}", kk, x);
                                sleep(Duration::from_millis(1000));
                                let value = self.general_purpose_registers.fetch_register(x as usize);
                                self.general_purpose_registers.set_register(x as usize, value.wrapping_add(kk));
                        },
                        Operation::LDXY(x, y) => {
                                println!("Loading V{:X} into V{:X}", x, y);
                                sleep(Duration::from_millis(1000));
                                let value = self.general_purpose_registers.fetch_register(x as usize);
                                self.general_purpose_registers.set_register(y as usize, value);
                        },
                        Operation::OR(x, y) => {
                                println!("ORing V{:X} with V{:X}", x, y);
                                sleep(Duration::from_millis(1000));
                                let value_x = self.general_purpose_registers.fetch_register(x as usize);
                                let value_y = self.general_purpose_registers.fetch_register(y as usize);
                                self.general_purpose_registers.set_register(x as usize, value_x | value_y);
                        },
                        Operation::AND(x, y) => {
                                println!("ANDing V{:X} with V{:X}", x, y);
                                sleep(Duration::from_millis(1000));
                                let value_x = self.general_purpose_registers.fetch_register(x as usize);
                                let value_y = self.general_purpose_registers.fetch_register(y as usize);
                                self.general_purpose_registers.set_register(x as usize, value_x & value_y);
                        },
                        Operation::XOR(x, y) => {
                                println!("XORing V{:X} with V{:X}", x, y);
                                sleep(Duration::from_millis(1000));
                                let value_x = self.general_purpose_registers.fetch_register(x as usize);
                                let value_y = self.general_purpose_registers.fetch_register(y as usize);
                                self.general_purpose_registers.set_register(x as usize, value_x ^ value_y);
                        },
                        Operation::ADD(x, y) => {
                                println!("Adding V{:X} to V{:X}", x, y);
                                sleep(Duration::from_millis(1000));
                                let value_x = self.general_purpose_registers.fetch_register(x as usize);
                                let value_y = self.general_purpose_registers.fetch_register(y as usize);
                                self.general_purpose_registers.set_register(x as usize, value_x.wrapping_add(value_y));
                                self.general_purpose_registers.display_register(x as usize);
                        },
                        Operation::SUB(x, y) => {
                                println!("Subtracting V{:X} from V{:X}", x, y);
                                sleep(Duration::from_millis(1000));
                                let value_x = self.general_purpose_registers.fetch_register(x as usize);
                                let value_y = self.general_purpose_registers.fetch_register(y as usize);
                                self.general_purpose_registers.set_register(x as usize, value_x.wrapping_sub(value_y));
                        },
                        Operation::SHR(x) => {
                                println!("Shifting V{:X} right", x);
                                sleep(Duration::from_millis(1000));
                                let value = self.general_purpose_registers.fetch_register(x as usize);
                                self.general_purpose_registers.set_register(x as usize, value >> 1);
                        },
                        Operation::SHL(x) => {
                                println!("Shifting V{:X} left", x);
                                sleep(Duration::from_millis(1000));
                                let value = self.general_purpose_registers.fetch_register(x as usize);
                                self.general_purpose_registers.set_register(x as usize, value << 1);
                        },

                        Operation::LDI(nnn) => {
                                println!("Loading {:#X} into Instruction Register", nnn);
                                sleep(Duration::from_millis(1000));
                                self.instruction_register.register = nnn as u16;
                        },
                };
        }

    pub fn execute_from_memory(&mut self, location: usize, count: usize) {
        loop {
            let instruction_bytes = self.memory.fetch_instruction(location);
            let instruction = Instruction::decode_instruction(instruction_bytes);
            self.execute_instruction(instruction);
            self.program_counter.increment();
            if self.program_counter.counter >= location + count {
                break;
            }
        }
    }

    pub fn execute_from_file(&mut self, filename: &str) {
        let instructions = compiler(filename);
        for instruction in instructions {
            self.execute_instruction(instruction);
        }
    }


    pub fn inject_instructions(&mut self, data: &[u8], location: usize) {
        self.memory.inject_instructions(data, location);
    }

}