use crate::{HalMemController, Cpu, CpuRegisters, MemoryMap, ConsoleCpu};
use crate::register::{CpuRegister, Register};
use std::sync::Arc;
use std::rc::Rc;
use std::cell::RefCell;

mod clock;
mod mem_map;

fn _fetch(cache_line: &[u8], memory_map: &MemoryMap) {

}

fn _execute(memory_map: &MemoryMap) {
}

fn _create_nes_cpu_registers() -> CpuRegisters {
    let mut registers = Vec::new();
    registers.push(Rc::new(RefCell::new(CpuRegister::R8(Register::new("accumulator")))));
    CpuRegisters::new(registers)
}

fn create_nes_cpu_unit() -> Rc<RefCell<Cpu>> {
    Rc::new(RefCell::new(Cpu {
        registers: _create_nes_cpu_registers(),
        fetch: Box::new(_fetch),
        execute: Box::new(_execute)
    }))
}

pub fn create_nes_cpu(mem_controller: &mut Arc<&mut HalMemController>) -> ConsoleCpu {
    ConsoleCpu {
        cpu_unit: create_nes_cpu_unit(),
        mem_map: mem_map::create_nes_mem_map(mem_controller),
        clock_unit: clock::create_nes_clock(),
    }
}
