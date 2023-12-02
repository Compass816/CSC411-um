use std::{
    io::{self, Write, Read},
    process::{self, exit},
};

use crate::memory::{Memory, Registers};


/// Conditional move: $r[C] != 0 then $r[A] := $r[B]
///
/// # Arguments:
/// * `&mut Registers`: a Registers object, which is a vec of u32s that act as registers
/// * `a: u32` : index a in Registers.data
/// * `b: u32` : index b in Registers.data
/// * `c: u32` : index b in Registers.data
pub fn cmov(registers: &mut Registers, a: u32, b: u32, c: u32) {
    //let ra = registers.data[a as usize];
    let rb = registers.data[b as usize];
    let rc = registers.data[c as usize];

    //println!("a={}, b={}, c={}", a, b, c);
    //println!("$r[a]={} $r[b]={}, $r[c]={}", ra, rb, rc);

    if rc != 0 {
        registers.data[a as usize] = rb;

        //println!("Moved $r[{}]={} to $r[{}]={}", a, ra, b, rb);
    }
}


/// Segmented Load $r[A] := $m[$r[B]][$r[C]]
///
/// # Arguments:
/// * `&mut Registers`: a Registers object, which is a vec of u32s that act as registers
/// * `&mut Memory` : a Memory object, which is a hashmap<u32, Vec<u32>, representing segmented memory given an ID and an offset in memory.
/// * `a: u32` : index a in Registers.data
/// * `b: u32` : index b in Registers.data
/// * `c: u32` : index b in Registers.data
pub fn load(registers: &mut Registers, memory: &mut Memory, a: u32, b: u32, c: u32) {
    //let ra = registers.data[a as usize];
    let rb = registers.data[b as usize];
    let rc = registers.data[c as usize];

    //println!("a={}, b={}, c={}", a, b, c);
    //println!("$r[a]={} $r[b]={}, $r[c]={}", ra, rb, rc);

    //println!("Loading {} from $m[{}][{}]", ra, rb, rc);

    if let Some(segment) = memory.get(&rb) {
        if let Some(value) = segment.get(rc as usize) {
            registers.data[a as usize] = *value;
        }
    }
}


/// Segmented Store $m[$r[A]][$r[B]] := $r[C]
///
/// # Arguments:
/// * `&mut Registers`: a Registers object, which is a vec of u32s that act as registers
/// * `&mut Memory` : a Memory object, which is a hashmap<u32, Vec<u32>, representing segmented memory given an ID and an offset in memory.
/// * `a: u32` : index a in Registers.data
/// * `b: u32` : index b in Registers.data
/// * `c: u32` : index b in Registers.data
pub fn store(registers: &mut Registers, memory: &mut Memory, a: u32, b: u32, c: u32) {
    let ra = registers.data[a as usize];
    let rb = registers.data[b as usize];
    let rc = registers.data[c as usize];

    //println!("a={}, b={}, c={}", a, b, c);
    //println!("$r[a]={} $r[b]={}, $r[c]={}", ra, rb, rc);

    //println!("Storing {} to $m[{}][{}]", rc, ra, rb);

    // Get the vec for the correspodning ID, $r[a]
    if let Some(segment) = memory.get(&ra) {
        // Clone the vec and update it at the index $r[b] with $r[c]
        let mut new_vec = Some(segment).unwrap().clone();
        new_vec[rb as usize] = rc;

        memory.set(ra, new_vec);
    }
}


/// Addition $r[A] := ($r[B] + $r[C]) mod 2^32
///
/// # Arguments:
/// * `&mut Registers`: a Registers object, which is a vec of u32s that act as registers
/// * `a: u32` : index a in Registers.data
/// * `b: u32` : index b in Registers.data
/// * `c: u32` : index b in Registers.data
pub fn add(registers: &mut Registers, a: u32, b: u32, c: u32) {
    let rb = registers.data[b as usize];
    let rc = registers.data[c as usize];

    //println!("b={}, c={}", b, c);
    //println!("$r[b]={}, $r[c]={}", rb, rc);

    //println!("Adding $r[a]={} + {} -> {:032b}", rb, rc, rc);

    registers.data[a as usize] = match rb.checked_add(rc) {
        Some(result) => result,
        None => {
            exit(1);
        }
    }
}


/// Multiplication $r[A] := ($r[B] × $r[C]) mod 2^32
///
/// # Arguments:
/// * `&mut Registers`: a Registers object, which is a vec of u32s that act as registers
/// * `a: u32` : index a in Registers.data
/// * `b: u32` : index b in Registers.data
/// * `c: u32` : index b in Registers.data
pub fn mult(registers: &mut Registers, a: u32, b: u32, c: u32) {
    let rb = registers.data[b as usize];
    let rc = registers.data[c as usize];

    //println!("b={}, c={}", b, c);
    //println!("$r[b]={}, $r[c]={}", rb, rc);

    //println!("Multiplying $r[a]={} * {}", rb, rc);

    registers.data[a as usize] = match rb.checked_mul(rc) {
        Some(result) => result,
        None => {
            exit(1);
        }
    }
}


/// Division $r[A] := ($r[B] ÷ $r[C]) (integer division)
///
/// # Arguments:
/// * `&mut Registers`: a Registers object, which is a vec of u32s that act as registers
/// * `a: u32` : index a in Registers.data
/// * `b: u32` : index b in Registers.data
/// * `c: u32` : index b in Registers.data
pub fn div(registers: &mut Registers, a: u32, b: u32, c: u32) {
    let rb = registers.data[b as usize];
    let rc = registers.data[c as usize];

    //println!("b={}, c={}", b, c);
    //println!("$r[b]={}, $r[c]={}", rb, rc);

    //println!("Dividing $r[a]={} / {}", rb, rc);

    registers.data[a as usize] = rb / rc;
}


/// Bitwise NAND $r[A] :=¬($r[B]∧$r[C])
///
/// # Arguments:
/// * `&mut Registers`: a Registers object, which is a vec of u32s that act as registers
/// * `a: u32` : index a in Registers.data
/// * `b: u32` : index b in Registers.data
/// * `c: u32` : index b in Registers.data
pub fn nand(registers: &mut Registers, a: u32, b: u32, c: u32) {
    let rb = registers.data[b as usize];
    let rc = registers.data[c as usize];

    registers.data[a as usize] = !(rb & rc);

    //let ra = registers.data[a as usize];

    println!("{:032b}", rb);
    println!("{:032b}", rc);
    println!("{:032b}", rb & rc);
}

/// Ends the UM
pub fn halt() {
    process::exit(0);
}


/// Map segment A new segment is created with a number of words equal to the value in $r[C]. Each word in the new segment is initialized to zero. A bit pattern that is not all zeroes and does not identify any currently mapped segment is placed in $r[B]. The new segment is mapped as $m[$r[B]].
///
/// # Arguments:
/// * `&mut Registers`: a Registers object, which is a vec of u32s that act as registers
/// * `&mut Memory` : a Memory object, which is a hashmap<u32, Vec<u32>, representing segmented memory given an ID and an offset in memory.
/// * `b: u32` : index b in Registers.data
/// * `c: u32` : index b in Registers.data
pub fn map(registers: &mut Registers, memory: &mut Memory, b: u32, c: u32) {
    if c != 0 {
        //let rb = registers.data[b as usize];
        let rc = registers.data[c as usize];

        let seg = vec![0_u32; rc as usize];

        // Grab the highest unused id if there are no previously unmapped ids available
        let id = match memory.mem_ids.is_empty() {
            true => {
                let id = memory.latest_id + 1;
                memory.latest_id = id;
                id
            }
            false => {
                let id = memory.mem_ids[0];
                memory.mem_ids.remove(0);
                id
            }
        };

        // The id is placed in register b
        registers.data[b as usize] = id;

        //println!("b={}, c={}", b, c);
        //println!("$r[b]={}, $r[c]={}", rb, rc);

        //println!("Mapping segment at id={}, length={}", rb, rc);

        memory.add(id, seg);
    }
}


/// Unmap segment The segment $m[$r[C]] is unmapped. Future Map Segment instructions may reuse the identifier $r[C].
/// # Arguments:
/// * `&mut Registers`: a Registers object, which is a vec of u32s that act as registers
/// * `&mut Memory` : a Memory object, which is a hashmap<u32, Vec<u32>, representing segmented memory given an ID and an offset in memory.
/// * `c: u32` : index b in Registers.data
pub fn unmap(registers: &mut Registers, memory: &mut Memory, c: u32) {
    // Add the unmapped id to the vec of unmapped ids
    let rc = registers.data[c as usize];
    memory.mem_ids.push(rc);

    //println!("c={}", c);
    //println!("$r[c]={}",rc);

    //println!("Unmapping segment at id={}", rc);

    memory.remove(&rc);
}


/// Output The value in $r[C] is displayed on the I/O device immediately. Only values from 0 to 255 are allowed.
/// # Returns:
/// * `io::Result<()>` Result type if the value was printed to stdout.
/// # Arguments:
/// * `&mut Registers`: a Registers object, which is a vec of u32s that act as registers
/// * `c: u32` : index b in Registers.data
pub fn output(registers: &mut Registers, c: u32) -> io::Result<()> {
    let c = registers.data[c as usize];

    let mut stdout = io::stdout().lock();
    let c_bytes: &[u8] = &c.to_ne_bytes(); // Convert integer to bytes
    stdout.write(&[c_bytes[0]])?;
    stdout.flush()?;

    Ok(())
}


/// Input The UM waits for input on the I/O device. When input arrives, $r[c] is loaded with the input, which must be a value from 0 to 255. If the end of input has been signaled, then $r[C] is loaded with a full 32-bit word in which every bit is 1.
/// # Arguments:
/// * `&mut Registers`: a Registers object, which is a vec of u32s that act as registers
/// * `c: u32` : index b in Registers.data
pub fn input(registers: &mut Registers, c: u32) {
    if let Some(Ok(byte)) = io::stdin().bytes().next() {
        registers.data[c as usize] = u32::from(byte);
    } else {
        registers.data[c as usize] = !0;
    }
}


/// Load Program Segment $m[$r[B]] is duplicated, and the duplicate replaces $m[0], which is abandoned. The program counter is set to point to $m[0][$r[C]]. If $r[B]=0, the load program operation should be extremely quick, as this is effectively a jump.
/// # Arguments:
/// * `&mut Registers`: a Registers object, which is a vec of u32s that act as registers
/// * `&mut Memory` : a Memory object, which is a hashmap<u32, Vec<u32>, representing segmented memory given an ID and an offset in memory.
/// * `b: u32` : index b in Registers.data
/// * `c: u32` : index b in Registers.data
pub fn loadp(registers: &mut Registers, memory: &mut Memory, b: u32, c: u32) {
    let rb = registers.data[b as usize];
    let rc = registers.data[c as usize];

    //println!("Load Program:");

    //println!("b={}, c={}", b, c);
    //println!("$r[b]={}, $r[c]={}", rb, rc);

    // Duplicate the value at $m[$r[b]] and replace $m[0]
    let segment = memory.get(&rb).unwrap().clone();
    //println!("Segment: $m[$r[0]] -> $m[$r[{}]]", b);
    memory.set(0, segment);


    // Update the program counter to point to $m[0][$r[c]]
    if let Some(segment) = memory.get(&0) {
        if let Some(value) = segment.get(rc as usize) {
            //println!("pc={}", value);
            memory.program_counter = *value;
            //println!("Program counter={}", memory.program_counter);
        }
    }
}


/// Load Value the value `vl` is loaded into register $r[`a`]
/// * `&mut Registers`: a Registers object, which is a vec of u32s that act as registers
/// * `a: u32` : index a in Registers.data
/// * `vl: u32` : the value to be loaded into $r[a]
pub fn loadv(registers: &mut Registers, a: u32, vl: u32) {
    registers.data[a as usize] = vl;
}
