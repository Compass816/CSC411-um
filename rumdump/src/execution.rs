use std::{process, io::{self, Write}};

use crate::memory::{Memory, Registers, self};

pub fn cmov(registers: &mut Registers, a: u32, b: u32, c: u32) {
    let a = registers.data[a as usize];
    let b = registers.data[b as usize];
    let c = registers.data[c as usize];

    if registers.data[c as usize] != 0 {
        registers.data[a as usize] = registers.data[b as usize];
    }
}

pub fn load(registers: &mut Registers, memory: &mut Memory, a: u32, b: u32, c: u32) {
    let ra = registers.data[a as usize];
    let rb = registers.data[b as usize];
    let rc = registers.data[c as usize];

    println!("a={}, b={}, c={}", a, b, c);
    println!("$r[a]={} $r[b]={}, $r[c]={}", ra, rb, rc);

    println!("Loading {} from $m[{}][{}]", ra, rb, rc);

    if let Some(segment) = memory.get(&rb) {
        if let Some(value) = segment.get(rc as usize) {
            registers.data[a as usize] = *value;
        }
    }
}

pub fn store(registers: &mut Registers, memory: &mut Memory, a: u32, b: u32, c: u32) {
    let ra = registers.data[a as usize];
    let rb = registers.data[b as usize];
    let rc = registers.data[c as usize];

    println!("a={}, b={}, c={}", a, b, c);
    println!("$r[a]={} $r[b]={}, $r[c]={}", ra, rb, rc);

    println!("Storing {} to $m[{}][{}]", rc, ra, rb);

    // Get the vec for the correspodning ID, $r[a]
    if let Some(segment) = memory.get(&ra) {
        // Clone the vec and update it at the index $r[b] with $r[c]
        let mut new_vec = Some(segment).unwrap().clone();
        new_vec[rb as usize] = rc;

        memory.set(ra, new_vec);
    }
}

pub fn add(registers: &mut Registers, a: u32, b: u32, c: u32) {
    let b = registers.data[b as usize];
    let c = registers.data[c as usize];
    // % (2_u32.pow(32));
    registers.data[a as usize] = (b + c);
}

pub fn mult(registers: &mut Registers, a: u32, b: u32, c: u32) {
    let b = registers.data[b as usize];
    let c = registers.data[c as usize];
    // % (2_u32.pow(32));
    registers.data[a as usize] = (b * c);
}

pub fn div(registers: &mut Registers, a: u32, b: u32, c: u32) {
    let b = registers.data[b as usize];
    let c = registers.data[c as usize];
    registers.data[a as usize] = (b / c);
}

pub fn nand(registers: &mut Registers, a: u32, b: u32, c: u32) {
    let a = registers.data[a as usize];
    let b = registers.data[b as usize];
    let c = registers.data[c as usize];
    registers.data[a as usize] = !(registers.data[b as usize] & registers.data[c as usize]);
}

pub fn halt() {
    process::exit(0);
}

pub fn map(registers: &mut Registers, memory: &mut Memory, b: u32, c: u32) {

    if c != 0 {
        let rb = registers.data[b as usize];
        let rc = registers.data[c as usize];

        let seg = vec![0_u32; rc as usize];

        // Grab the highest unused id if there are no previously unmapped ids available
        let mut id = 0_u32;
    
        if memory.mem_ids.is_empty() {
            id = memory.latest_id;
        } else {
            id = memory.mem_ids[0];
        }
    
        // The id is placed in register b
        registers.data[b as usize] = id;

        println!("b={}, c={}", b, c);
        println!("$r[b]={}, $r[c]={}", rb, rc);    
    
        memory.add(id, seg);
    }
}

pub fn unmap(registers: &mut Registers, memory: &mut Memory, c: u32) {
    // Add the unmapped id to the vec of unmapped ids
    memory.mem_ids.push(registers.data[c as usize]);

    memory.remove(&registers.data[c as usize]);
}

pub fn output(registers: &mut Registers, c: u32) -> io::Result<()> {
    let c = registers.data[c as usize];

    let mut stdout = io::stdout().lock();
    let c_bytes: &[u8] = &c.to_ne_bytes(); // Convert integer to bytes

    stdout.write_all(c_bytes)?;
    stdout.flush()?;

    Ok(())
}

pub fn input(registers: &mut Registers, c: u32) {

}

pub fn loadp(registers: &mut Registers, memory: &mut Memory, b: u32, c: u32) {
    // Duplicate the value at $m[$r[B]] and replace $m[0]
    let segment = memory.get(&(registers.data[b as usize])).unwrap().clone();
    memory.set(0, segment);

    // Update the program counter to point to $m[0][$r[C]]
    if let Some(segment) = memory.get(&(registers.data[0])) {
        if let Some(value) = segment.get(registers.data[c as usize] as usize) {
            memory.program_counter = *value;
        }
    }
}

pub fn loadv(registers: &mut Registers, a: u32, vl: u32) {
    registers.data[a as usize] = vl;
}
