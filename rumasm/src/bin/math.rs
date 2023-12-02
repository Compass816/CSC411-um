use rumasm::rumasm::asm;
use rumasm::rumasm::halt;
use rumasm::rumasm::loadv;
use rumasm::rumasm::map;
use rumasm::rumasm::output;
use rumasm::rumasm::add;
use rumasm::rumasm::mult;
use rumasm::rumasm::div;
use rumasm::rumasm::load;
use rumasm::rumasm::store;
use rumasm::rumasm::unmap;

/*
Used:
load X
store X
add
mult
div
nand
halt X
map X
unmap 
output X
input
load program
load value X
*/

pub fn main() {
    asm(loadv(7, 10)); // nl
    asm(loadv(0, decimal_to_ascii(5).unwrap()));
    asm(output(0));
    asm(output(7));
    asm(loadv(1, decimal_to_ascii(3).unwrap()));
    asm(output(1));
    asm(output(7));
    asm(add(2, 0, 1));
    asm(output(2));
    asm(output(7));
    asm(loadv(0, decimal_to_ascii(9).unwrap()));
    asm(output(0));
    asm(output(7));
    asm(mult(1, 0, 2));
    asm(output(1));
    asm(output(7));

    // Load the value 6 into register 1 (0-indexed)
    asm(loadv(1, 54)); // 5 in decimal

    // Map a new segment in memory whose length is 6
    asm(map(6, 1));

    // Load the value in memory segment with ID 1 at offset 0 into register 2.
    // We know it has an ID of 1 because this was the first mapped memory segment besides the initial program at 0.
    asm(load(2, 1, 0));

    // Output the value at register 2, which will be 0, which we just loaded.
    asm(output(2));

    // Load the value 3 into register 3
    asm(loadv(3, 51)); // 3 in decimal

    // Store the value from register 3 into the memory segmeent with ID 1 into the vec at offset 0
    asm(store(1, 0, 3));

    // Add the values at registers 1 and 3, which will result in 6 + 3 = 9
    asm(add(4, 1, 3));
    asm(output(4));

    // Multiply the values at registers 1 and 3, which will result in 6 * 3 = 18
    asm(add(4, 1, 3));
    asm(output(4));

    // Divide the values at registers 1 and 3, which will result in 6 / 3 = 2
    asm(add(4, 1, 3));
    asm(output(4));

    // Unmap the segment in memory at ID 1
    // This removes the key, value from the memory hashmap and adds the ID to a vector of unmapped IDs
    asm(unmap(1));

    // Map a new segment in memory whose length is 2
    // It chooses the ID 1 again because that ID was freed up. It prioritizes choosing one of the free IDs 
    // in the vector before incrementing the ID, which would be 2 if a new segment was created
    asm(map(4, 1));

    // Halt, ending the program
    asm(halt())
}


// Function to convert u32 digit to its ASCII equivalent as u32
fn decimal_to_ascii(digit: u32) -> Option<u32> {
    if digit < 10 {
        Some(digit + '0' as u32)
    } else {
        None // Return None for non-digit values
    }
}

// Function to convert ASCII digit as u32 to its equivalent u32 value
fn ascii_to_decimal(ascii_digit: u32) -> Option<u32> {
    if ascii_digit >= '0' as u32 && ascii_digit <= '9' as u32 {
        Some(ascii_digit - '0' as u32)
    } else {
        None // Return None for non-digit values
    }
}