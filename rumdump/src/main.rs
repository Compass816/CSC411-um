use rumdump::rumdis;
use rumdump::rumload;
use rumdump::memory::{Memory, Registers};
use std::env;

fn main() {
    let input = env::args().nth(1);
    let instructions = rumload::load(input.as_deref());
    println!("{} instructions", instructions.len());

    let mut registers = Registers::default();

    let mut memory = Memory::new(0, instructions);

    if let Some(instruction_vec) = memory.get(&0).cloned() {
        for instruction in instruction_vec {
            // Run rumdis, match the op code, run one of the functions below
            rumdis::disassemble(&mut registers, &mut memory, instruction)
        }
    }
}
