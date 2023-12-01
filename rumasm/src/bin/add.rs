use rumasm::rumasm::asm;
use rumasm::rumasm::halt;
use rumasm::rumasm::loadv;
use rumasm::rumasm::output;
use rumasm::rumasm::add;

pub fn main() {
    asm(loadv(7, 10)); // linefeed
    asm(loadv(0, 53)); // 5
    asm(loadv(1, 51)); // 3
    asm(output(0));
    asm(output(7));
    asm(output(1));
    asm(output(7));
    asm(add(2, 0, 1));
    asm(output(2));
    asm(output(7));
    asm(halt())
}
