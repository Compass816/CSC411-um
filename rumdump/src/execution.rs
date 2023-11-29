use crate::memory::{Memory, Registers, self};

pub fn cmov(registers: &mut Registers, a: u32, b: u32, c: u32) {
    let a = registers.data[a as usize];
    let b = registers.data[b as usize];
    let c = registers.data[c as usize];

    if registers.data[c as usize] != 0 {
        registers.data[a as usize] = registers.data[b as usize];
    }
}

pub fn load(registers: &mut Registers, memory: &Memory, a: u32, b: u32, c: u32) {
    let a = registers.data[a as usize];
    let b = registers.data[b as usize];
    let c = registers.data[c as usize];

    if let Some(segment) = memory.get(&(registers.data[b as usize])) {
        if let Some(value) = segment.get(registers.data[c as usize] as usize) {
            registers.data[a as usize] = *value;
        }
    }
}

pub fn store(registers: &mut Registers, memory: &Memory, a: u32, b: u32, c: u32) {
    let a = registers.data[a as usize];
    let b = registers.data[b as usize];
    let c = registers.data[c as usize];

    if let Some(segment) = memory.get(&(registers.data[a as usize])) {
        if let Some(mut value) = segment.get(registers.data[b as usize] as usize) {
            value = &registers.data[c as usize];
        }
    }
}

pub fn add(registers: &mut Registers, a: u32, b: u32, c: u32) {
    let a = registers.data[a as usize];
    let b = registers.data[b as usize];
    let c = registers.data[c as usize];
    registers.data[a as usize] = (registers.data[b as usize] + registers.data[c as usize]) % (2_u32.pow(32));
}

pub fn mult(registers: &mut Registers, a: u32, b: u32, c: u32) {
    let a = registers.data[a as usize];
    let b = registers.data[b as usize];
    let c = registers.data[c as usize];
    registers.data[a as usize] = (registers.data[b as usize] * registers.data[c as usize]) % (2_u32.pow(32));
}

pub fn div(registers: &mut Registers, a: u32, b: u32, c: u32) {
    let a = registers.data[a as usize];
    let b = registers.data[b as usize];
    let c = registers.data[c as usize];
    registers.data[a as usize] = registers.data[b as usize] / registers.data[c as usize];
}

pub fn nand(registers: &mut Registers, a: u32, b: u32, c: u32) {
    let a = registers.data[a as usize];
    let b = registers.data[b as usize];
    let c = registers.data[c as usize];
    registers.data[a as usize] = !(registers.data[b as usize] & registers.data[c as usize]);
}

pub fn halt() {
    println!("Halt");
}

pub fn map(registers: &mut Registers, memory: &Memory, b: u32, c: u32) {
    let seg = vec![0_u32; registers.data[c as usize] as usize];

    // Grab the highest unused id if there are no previously unmapped ids available
    let mut id = 0_u32;

    if memory.mem_ids.is_empty() {
        id = memory.latest_id;
    } else {
        id = memory.mem_ids[0];
    }

    memory.add(id, seg);
}

pub fn unmap(registers: &mut Registers, memory: &Memory, c: u32) {
    // Add the unmapped id to the vec of unmapped ids
    memory.mem_ids.push(registers.data[c as usize]);

    memory.remove(&registers.data[c as usize]);
}

pub fn output() {

}

pub fn input() {

}

pub fn loadp(registers: &mut Registers, memory: &Memory, b: u32, c: u32) {
    let Some(segment) = memory.get(&(registers.data[b as usize]));
    let a = memory.get(&0).unwrap();
    
     Some(segment);
}

pub fn loadv(registers: &mut Registers, rv: u32, vl: u32) {
    let a = registers.data[rv as usize];

    registers.data[a as usize] = vl as u32;
}
