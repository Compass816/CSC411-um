use crate::memory::{Memory, Registers};

pub fn cmov(registers: &mut Registers, a: u32, b: u32, c: u32) {
    let a = registers.data[a as usize];
    let b = registers.data[b as usize];
    let c = registers.data[c as usize];

    if registers.data[c as usize] != 0 {
        registers.data[a as usize] = registers.data[b as usize];
    }
}

pub fn load() {

}

pub fn store() {

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

pub fn map() {

}

pub fn unmap() {

}

pub fn output() {

}

pub fn input() {

}

pub fn loadp() {

}

pub fn loadv(registers: &mut Registers, rv: u32, vl: u32) {
    let a = registers.data[rv as usize];

    registers.data[a as usize] = vl as u32;
}
