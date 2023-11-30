use rumasm::rumasm::asm;
use rumasm::rumasm::halt;
use rumasm::rumasm::loadv;
use rumasm::rumasm::output;
use rumasm::rumasm::add;
use rumasm::rumasm::mult;
use rumasm::rumasm::div;


pub fn main() {
    asm(loadv(0, decimal_to_ascii(5).unwrap()));
    asm(output(0));
    asm(loadv(1, decimal_to_ascii(3).unwrap()));
    asm(output(1));
    asm(add(2, 0, 1));
    asm(output(2));

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