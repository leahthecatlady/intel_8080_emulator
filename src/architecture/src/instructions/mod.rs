use crate::opcodes::OpCode;
use crate::state::State;
use crate::instruction::Instruction;

pub fn not_implemented(state: &State, instruction: &Instruction) -> bool {
    println!("not implemented!");
    true
}

pub mod call;
pub mod carry_bit;
pub mod data;
pub mod direct_addressing;
pub mod double_register;
pub mod hlt;
pub mod immediate;
pub mod interrupt;
pub mod io;
pub mod jump;
pub mod nop;
pub mod returns;
pub mod rotate_accumulator;
pub mod rst;
pub mod single_register;
pub mod to_accumulator;