mod x86;

use x86::emulator::Emulator;
use wasm_bindgen::prelude::*;
use crate::memory::Memory;

/*const REGISTER_COUNT: usize = 8;

struct Instruction {
    prefix: u32,
    opcode: u32,
    modrm: u8,
    sib: u8,
    displacement: u32,
    immediate: u32,
}

#[wasm_bindgen]
pub struct Cpu {
    pc: u32,
    eflags: u32,
    registers: [u32; REGISTER_COUNT],
}

impl Cpu {
    pub fn decode_instruction(&mut self, memory: &mut Memory, loc: u32) -> Instruction {

    }
}*/

/*[wasm_bindgen]
impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            pc: 0,
            sp: 0
        }
    }

    pub fn start_executing(&mut self, memory: &mut Memory, entrypoint: u32) {
        self.pc = entrypoint;
    }
}*/