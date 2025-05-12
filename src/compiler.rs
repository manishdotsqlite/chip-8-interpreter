use std::fs::File;
use std::io::{ BufReader, BufRead};

use crate::instruction::{Instruction, Operation};

fn parse_line(line: &str) -> Vec<&str> {
        let words: Vec<&str> = line.split_whitespace().collect();
        words
}

fn words_to_instruction(words: Vec<&str>) -> Instruction {
        
        let instruction: Instruction = match words[0] {
                "CLS" => Instruction { operation: Operation::CLS },
                "RET" => Instruction { operation: Operation::RET },
                "JMP" => {
                        let nnn = u16::from_str_radix(words[1], 16).unwrap();
                        Instruction { operation: Operation::JMP(nnn) }
                },
                "CALL" => {
                        let nnn = u16::from_str_radix(words[1], 16).unwrap();
                        Instruction { operation: Operation::CALL(nnn) }
                },
                "SE" => {
                        let x = u8::from_str_radix(words[1], 16).unwrap();
                        let kk = u8::from_str_radix(words[2], 16).unwrap();
                        Instruction { operation: Operation::SE(x as i32, kk) }
                },
                "SNE" => {
                        let x = u8::from_str_radix(words[1], 16).unwrap();
                        let kk = u8::from_str_radix(words[2], 16).unwrap();
                        Instruction { operation: Operation::SNE(x as i32, kk) }
                },
                "SRE" => {
                        let x = u8::from_str_radix(words[1], 16).unwrap();
                        let y = u8::from_str_radix(words[2], 16).unwrap();
                        Instruction { operation: Operation::SRE(x as i32, y as i32) }
                },
                "LD" => {
                        let x = u8::from_str_radix(words[1], 16).unwrap();
                        let kk = u8::from_str_radix(words[2], 16).unwrap();
                        Instruction { operation: Operation::LD(x as i32, kk) }
                },
                "AD" => {
                        let x = u8::from_str_radix(words[1], 16).unwrap();
                        let kk = u8::from_str_radix(words[2], 16).unwrap();
                        Instruction { operation: Operation::AD(x as i32, kk) }
                },
                "LDXY" => {
                        let x = u8::from_str_radix(words[1], 16).unwrap();
                        let y = u8::from_str_radix(words[2], 16).unwrap();
                        Instruction { operation: Operation::LDXY(x as i32, y as i32) }
                },
                "OR" => {
                        let x = u8::from_str_radix(words[1], 16).unwrap();
                        let y = u8::from_str_radix(words[2], 16).unwrap();
                        Instruction { operation: Operation::OR(x as i32, y as i32) }
                },
                "AND" => {
                        let x = u8::from_str_radix(words[1], 16).unwrap();
                        let y = u8::from_str_radix(words[2], 16).unwrap();
                        Instruction { operation: Operation::AND(x as i32, y as i32) }
                },
                "XOR" => {
                        let x = u8::from_str_radix(words[1], 16).unwrap();
                        let y = u8::from_str_radix(words[2], 16).unwrap();
                        Instruction { operation: Operation::XOR(x as i32, y as i32) }
                },
                "ADD" => {
                        let x = u8::from_str_radix(words[1], 16).unwrap();
                        let y = u8::from_str_radix(words[2], 16).unwrap();
                        Instruction { operation: Operation::ADD(x as i32, y as i32) }
                },
                "SUB" => {
                        let x = u8::from_str_radix(words[1], 16).unwrap();
                        let y = u8::from_str_radix(words[2], 16).unwrap();
                        Instruction { operation: Operation::SUB(x as i32, y as i32) }
                },
                "SHR" => {
                        let x = u8::from_str_radix(words[1], 16).unwrap();
                        Instruction { operation: Operation::SHR(x as i32) }
                },
                "SHL" => {
                        let x = u8::from_str_radix(words[1], 16).unwrap();
                        Instruction { operation: Operation::SHL(x as i32) }
                },
                "LDI" => {
                        let nnn = u16::from_str_radix(words[1], 16).unwrap();
                        Instruction { operation: Operation::LDI(nnn) }
                },
                _ => panic!("Invalid instruction"),
        };
        instruction
}

pub fn compiler(filename: &str) -> Vec<Instruction> {
        let file = File::open(filename).expect("Unable to open file");
        let reader = BufReader::new(file);
        let mut instructions: Vec<Instruction> = Vec::new();
        for line in reader.lines() {
                match  line {
                        Ok(line) => {
                                let words: Vec<&str> = parse_line(&line);
                                if words.len() > 0 {
                                        let instruction = words_to_instruction(words);
                                        instructions.push(instruction);
                                }
                        },
                        Err(e) => {
                                eprintln!("Error reading line: {}", e);
                        }
                }
        }

        instructions
}