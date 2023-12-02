use rum::rumdis;
use rum::rumload;
use rum::memory::{Memory, Registers};
use std::env;
use std::process::exit;
//use std::time::Instant;

fn main() {
    //let start_time = Instant::now();

    if env::args().len() == 0 || env::args().len() >= 2 {
        let input = env::args().nth(1);

        let instructions = rumload::load(input.as_deref());
        //println!("{} instructions", instructions.len());
    
        let mut registers = Registers::default();
    
        let mut memory = Memory::new(0, instructions);
    
        if let Some(instruction_vec) = memory.get(&0).cloned() {
            for instruction in instruction_vec {
    
                // Run rumdis, match the op code, run one of the functions below
                rumdis::disassemble(&mut registers, &mut memory, instruction);
                //println!("{}", memory.program_counter);
                //println!("{:?}", registers.data);

                //let end_time = Instant::now();
                //let elapsed_time = end_time - start_time;
                //println!("Program execution time: {:.2} seconds", elapsed_time.as_secs_f64());
            }
        }
    } else {
        eprintln!("Invalid number of arguments");
        exit(0)
    }
}
