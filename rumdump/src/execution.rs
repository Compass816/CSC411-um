use crate::memory::{Memory, Registers};

pub fn cmov() {

}

pub fn load() {

}

pub fn store() {

}

pub fn add(registers: &mut Registers, a: u32, b: u32, c: u32) {
    let a = registers.data[a as usize];
    let b = registers.data[b as usize];
    let c = registers.data[c as usize];

    // Then do the acutal addition
}

pub fn mult() {

}

pub fn div() {

}

pub fn nand() {

}

pub fn halt() {

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

pub fn loadv() {

}
