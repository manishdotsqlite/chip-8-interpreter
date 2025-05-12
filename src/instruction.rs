

pub enum Operation {
        CLS, // 00E0
        RET, // 00EE
        JMP(u16), // 1nnn - Jump to location nnn
        CALL(u16), // 2nnn - Call subroutine at address nnn

        SE(i32, u8), // 3xkk - (SE Vx, byte): Skip the next instruction if Vx == kk
        SNE(i32, u8), //  4xkk - Skip the next instruction if Vx != kk
        SRE(i32, i32), // 5xy0 - Skip the next instruction if Vx == Vy

        LD(i32, u8), // 6xkk - Load kk into Vx
        AD(i32, u8), // 7xkk - Add kk to Vx
        LDXY(i32, i32), // 8xy0 - Load Vy into Vx
        OR(i32, i32), // 8xy1 - Vx = Vx OR Vy
        AND(i32, i32), // 8xy2 - Vx = Vx AND Vy
        XOR(i32, i32), // 8xy3 - Vx = Vx XOR Vy
        ADD(i32, i32), // 8xy4 - Vx = Vx + Vy
        SUB(i32, i32), // 8xy5 - Vx = Vx - Vy
        SHR(i32), // 8xy6 - Shift Vx right by 1
        SHL(i32), // 8xy7 - Shift Vx left by 1

        LDI(u16), // Annn - loads address value to Instruction Register

}

pub struct Instruction {
        pub operation: Operation,
}

impl Instruction {
    pub fn decode_instruction(instruction_bytes: u16) -> Instruction {
        let hex_string = format!("{:X}", instruction_bytes);
        let instruction: Instruction = match hex_string.chars().next() {
                Some(x) => match x {
                        '0' => Instruction { operation: Operation::CLS },
                        '1' => {
                                let nnn = instruction_bytes & 0x0FFF;
                                Instruction { operation: Operation::JMP(nnn) }
                        },
                        '2' => {
                                let nnn = instruction_bytes & 0x0FFF;
                                Instruction { operation: Operation::CALL(nnn) }
                        },
                        '3' => {
                                let x = (instruction_bytes >> 8) & 0x0F;
                                let kk = instruction_bytes & 0x00FF;
                                Instruction { operation: Operation::SE(x as i32, kk as u8) }
                        },
                        '4' => {
                                let x = (instruction_bytes >> 8) & 0x0F;
                                let kk = instruction_bytes & 0x00FF;
                                Instruction { operation: Operation::SNE(x as i32, kk as u8) }
                        },
                        '5' => {
                                let x = (instruction_bytes >> 8) & 0x0F;
                                let y = (instruction_bytes >> 4) & 0x0F;
                                Instruction { operation: Operation::SRE(x as i32, y as i32) }
                        },
                        '6' => {
                                let x = (instruction_bytes >> 8) & 0x0F;
                                let kk = instruction_bytes & 0x00FF;
                                Instruction { operation: Operation::LD(x as i32, kk as u8) }
                        },
                        '7' => {
                                let x = (instruction_bytes >> 8) & 0x0F;
                                let kk = instruction_bytes & 0x00FF;
                                Instruction { operation: Operation::AD(x as i32, kk as u8) }
                        },
                        '8' => {
                                let x = (instruction_bytes >> 8) & 0x0F;
                                let y = (instruction_bytes >> 4) & 0x0F;
                                match instruction_bytes & 0x000F {
                                        0 => Instruction { operation: Operation::LDXY(x as i32, y as i32) },
                                        1 => Instruction { operation: Operation::OR(x as i32, y as i32) },
                                        2 => Instruction { operation: Operation::AND(x as i32, y as i32) },
                                        3 => Instruction { operation: Operation::XOR(x as i32, y as i32) },
                                        4 => Instruction { operation: Operation::ADD(x as i32, y as i32) },
                                        5 => Instruction { operation: Operation::SUB(x as i32, y as i32) },
                                        6 => Instruction { operation: Operation::SHR(x as i32) },
                                        7 => Instruction { operation: Operation::SHL(x as i32) },
                                        _ => panic!("Invalid instruction"),
                                }
                        },
                        'A' => {
                                let nnn = instruction_bytes & 0x0FFF;
                                Instruction { operation: Operation::LDI(nnn) }
                        },
                        _ => panic!("Invalid instruction"),
                        
                }
                None => panic!("Invalid instruction"),
        };

        instruction
    }


}
