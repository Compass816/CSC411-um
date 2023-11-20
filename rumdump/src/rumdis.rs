#[derive(Debug, PartialEq, Copy, Clone, FromPrimitive)]
#[repr(u32)]
enum Opcode {
CMov,
Load,
Store,
Add,
Mult, 
Div, 
Nand,
Halt, 
Map,
Unmap,
Output,
Input,
Loadp,
Loadv,
}

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
type Umi = u32;
pub struct Field {
width: u32,
lsb: u32,
}
static RA: Field = Field {width: 3, lsb: 6};
static RB: Field = Field {width: 3, lsb: 3};
static RC: Field = Field {width: 3, lsb: 0};
static RL: Field = Field {width: 3, lsb: 25};
static VL: Field = Field {width: 25, lsb: 0};
static OP: Field = Field {width: 4, lsb: 28};
fn mask(bits: u32) -> u32 { (1 << bits) - 1 }
/// Given a `field` and `instruction`, extract
/// that field from the instruction as a u32
pub fn get(field: &Field, instruction: Umi) -> u32 {
(instruction >> field.lsb) & mask(field.width)
}
/// Given an instruction word, extract the opcode
fn op(instruction: Umi) -> Option<Opcode> {
FromPrimitive::from_u32((instruction >> OP.lsb) & mask(OP.width))
}
pub fn disassemble(inst: Umi) -> String {
// match FromPrimitive::from_u32(get(&OP, inst)) {
match op(inst) {
Some(Opcode::CMov) => {
format!(
"if (r{} != 0) r{} := r{};",
get(&RC, inst),
get(&RA, inst),
get(&RB, inst)
)
}
}
}



