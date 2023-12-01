use rumasm::rumasm::asm;
use rumasm::rumasm::halt;
use rumasm::rumasm::loadv;
use rumasm::rumasm::output;
use rumasm::rumasm::div;

pub fn main() {
    asm(loadv(7, 10)); // linefeed
    asm(loadv(0, 99));
    asm(loadv(1, 33));
    asm(output(0));
    asm(output(7));
    asm(output(1));
    asm(output(7));
    asm(div(2, 0, 1)); // Should be 3 in ASCII
    asm(output(2));
    asm(output(7));
    asm(halt())
}
