use crate::memory::{Memory, Registers};

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

use crate::execution;
type Umi = u32;

pub struct Field {
    width: u32,
    lsb: u32,
}

static RA: Field = Field { width: 3, lsb: 6 };
static RB: Field = Field { width: 3, lsb: 3 };
static RC: Field = Field { width: 3, lsb: 0 };
static RL: Field = Field { width: 3, lsb: 25 };
static VL: Field = Field { width: 25, lsb: 0 };
static OP: Field = Field { width: 4, lsb: 28 };

fn mask(bits: u32) -> u32 {
    (1 << bits) - 1
}

/// Given a `field` and `instruction`, extract
/// that field from the instruction as a u32
pub fn get(field: &Field, instruction: Umi) -> u32 {
    (instruction >> field.lsb) & mask(field.width)
}

/// Given an instruction word, extract the opcode
fn op(instruction: Umi) -> Option<Opcode> {
    FromPrimitive::from_u32((instruction >> OP.lsb) & mask(OP.width))
}

pub fn disassemble(registers: &mut Registers, memory: &mut Memory, inst: Umi) {
    // match FromPrimitive::from_u32(get(&OP, inst)) {
    match op(inst) {
        Some(Opcode::CMov) => {
            // format!(
            //     "if (r{} != 0) r{} := r{};",
            //     get(&RC, inst),
            //     get(&RA, inst),
            //     get(&RB, inst)
            // )
        }
        Some(Opcode::Add) => {
            let a = get(&RA, inst);
            let b = get(&RB, inst);
            let c = get(&RC, inst);

            execution::add(registers, a, b, c)

            // format!(
            //     "r{} := r{} + r{};",
            //     get(&RA, inst),
            //     get(&RB, inst),
            //     get(&RC, inst)
            // )
        }

        Some(Opcode::Mult) => {
            let a = get(&RA, inst);
            let b = get(&RB, inst);
            let c = get(&RC, inst);

            execution::mult(registers, a, b, c)

            // format!(
            //     "r{} := r{} * r{};",
            //     get(&RA, inst),
            //     get(&RB, inst),
            //     get(&RC, inst)
            // )
        }

        Some(Opcode::Div) => {
            let a = get(&RA, inst);
            let b = get(&RB, inst);
            let c = get(&RC, inst);

            execution::div(registers, a, b, c)

            // format!(
            //     "r{} := r{} / r{};",
            //     get(&RA, inst),
            //     get(&RB, inst),
            //     get(&RC, inst)
            // )
        }

        Some(Opcode::Halt) => {
            execution::halt()
        }

        Some(Opcode::Loadv) => {
            let rl = get(&RL, inst);
            let vl = get(&VL, inst);

            execution::loadv(registers, rl, vl)

            // format!(
            //     "r{} := v{};",
            //     get(&RL, inst),
            //     get(&VL, inst),
            // )
        }

        Some(Opcode::Nand) => {
            let a = get(&RA, inst);
            let b = get(&RB, inst);
            let c = get(&RC, inst);

            execution::nand(registers, a, b, c)

            // format!(
            //     "r{} := r{} ∧ r{};",
            //     get(&RA, inst),
            //     get(&RB, inst),
            //     get(&RC, inst)
            // )
        }

        Some(Opcode::Store) => {
            let a = get(&RA, inst);
            let b = get(&RB, inst);
            let c = get(&RC, inst);

            execution::store(registers, memory, a, b, c)

            // format!(
            //     "m[r{}][r{}] := r{};",
            //     get(&RA, inst),
            //     get(&RB, inst),
            //     get(&RC, inst)
            // )
        }

        Some(Opcode::Load) => {
            let a = get(&RA, inst);
            let b = get(&RB, inst);
            let c = get(&RC, inst);

            execution::load(registers, memory, a, b, c)

            // format!(
            //     "r{} := m[r{}][r{}];",
            //     get(&RA, inst),
            //     get(&RB, inst),
            //     get(&RC, inst)
            // )
        }

        
        Some(Opcode::Map) => {
            let b = get(&RB, inst);
            let c = get(&RC, inst);

            execution::map(registers, memory, b, c)

            // format!(
            //     "m[r{}];",
            //     get(&RB, inst),
                
            // )
        }
          
        Some(Opcode::Unmap) => {
            let c = get(&RC, inst);

            execution::unmap(registers, memory, c)

            // format!(
            //     "m[r{}];",
            //     get(&RC, inst),
                
            // )
        }

        Some(Opcode::Output) => {
            let c = get(&RC, inst);

            execution::output(registers, c)

            // format!(
            //     "r{} := r{};",
            //     get(&RC, inst),
            //     get(&RC, inst),
                
            // )
        }

        Some(Opcode::Input) => {
            let c = get(&RC, inst);

            execution::input(registers, c)

            // format!(
            //     "r{} := r{};",
            //     get(&RC, inst),
            //     get(&RC, inst),

                
            // )
        }

        Some(Opcode::Loadp) => {
            let b = get(&RB, inst);
            let c = get(&RC, inst);

            execution::loadp(registers, memory, b, c)

            // format!(
            //     "m[0] := m[r{}];",
            //     get(&RB, inst),
            //  //   get(&RC, inst),
                
            // )
        }

        None => todo!()
    }
}
