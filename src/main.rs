mod memory;
mod register;
mod stack;
mod cpu;
mod instruction;
mod compiler;

fn main() {
        let mut cpu = cpu::CPU::new();
        cpu.execute_from_file("src/program.txt");

}