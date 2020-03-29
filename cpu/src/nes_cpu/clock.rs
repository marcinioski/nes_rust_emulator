use crate::{HalMemController, Cpu, MemoryMap, Clock};
use std::sync::Arc;
use std::rc::Rc;
use std::cell::RefCell;

fn _tick(cpu: &mut Cpu, mem_map: &mut MemoryMap, cache_line: &[u8]) {
    (cpu.fetch)(cache_line, mem_map);
    (cpu.execute)(mem_map);
}

pub fn create_nes_clock() -> Rc<RefCell<Clock>> {
    Rc::new(RefCell::new(Clock  {tick: Box::new(_tick)} ))
}
