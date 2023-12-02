use rumasm::rumasm::asm;
use rumasm::rumasm::halt;
use rumasm::rumasm::loadv;
use rumasm::rumasm::output;
use rumasm::rumasm::load;
use rumasm::rumasm::store;
use rumasm::rumasm::map;
use rumasm::rumasm::unmap;
use rumasm::rumasm::cmov;

pub fn main() {
    asm(loadv(7, 10)); // linefeed
    asm(loadv(0, 53)); // 5
    asm(output(0));
    asm(output(7));
    asm(loadv(1, 1)); // Load the length of 1 into $r[1]
    asm(loadv(2, 1)); // Load the ID=1 into $r[2]
    asm(map(2, 1)); // Map a new segment of length 1. Id stored in $r[2]
    asm(loadv(3, 0)); // Load the offset into $r[3]
    asm(store(2, 3, 0)); // Store the value  $r[0] into $m[1][0] into, which should be 5
    asm(load(4, 2, 3)); // Load the value from $m[1][0] into $r[0]
    asm(unmap(1));
    asm(output(4)); // Should be 5
    asm(output(7));
    asm(cmov(4, 0, 2)); // if $r[2] != 0 then move $r[4] to $r[0]
    asm(output(4)); // Should be 5
    asm(output(7));
    asm(halt())
}
