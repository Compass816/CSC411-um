use rumasm::rumasm::asm;
use rumasm::rumasm::halt;
use rumasm::rumasm::loadv;
use rumasm::rumasm::output;

pub fn main() {
    asm(loadv(0, char_to_ascii('A'))); // A
    asm(output(0));
    //asm(loadv(0, 10)); // linefeed
    //asm(output(0));
    asm(halt())
}

fn char_to_ascii(character: char) -> u32 {
    character as u32
}