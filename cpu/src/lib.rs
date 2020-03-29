use std::sync::Arc;
use std::rc::Rc;
use std::cell::RefCell;
use std::convert::AsRef;
pub use hal::interface::MemoryController as HalMemController;
pub use hal::interface::Memory as HalMemory;
pub use hal::interface::RawPtr as HalPtr;
pub use hal::interface::RomType as HalRomType;

mod nes_cpu;
mod register;
use register::CpuRegisters;
pub use register::RegisterValue;

pub struct MemoryRegion {
    region_begin: Rc<HalPtr>,
    region_size: usize,
    region_name: &'static str,
    region_names: Option<std::collections::LinkedList<MemoryRegion>>,
}

pub struct MemoryMap {
    ram: MemoryRegion,
    vram: MemoryRegion,
}

impl AsRef<MemoryMap> for MemoryMap {
    fn as_ref(&self) -> &MemoryMap {
        self
    }
}

pub struct Cpu {
    registers: CpuRegisters,
    pub fetch: Box<dyn Fn(&[u8], &MemoryMap)>,
    pub execute: Box<dyn Fn(&MemoryMap)>,
}

impl AsRef<Cpu> for Cpu {
    fn as_ref(&self) -> &Cpu {
        self
    }
}

pub struct Clock {
    pub tick: Box<dyn Fn(&mut Cpu, &mut MemoryMap, &[u8])>,
}

pub struct ConsoleCpu {
    mem_map: Rc<RefCell<MemoryMap>>,
    cpu_unit: Rc<RefCell<Cpu>>,
    clock_unit: Rc<RefCell<Clock>>,
}

impl AsRef<ConsoleCpu> for ConsoleCpu {
    fn as_ref(&self) -> &ConsoleCpu {
        self
    }
}

impl ConsoleCpu {
    pub fn run(&mut self, cache_line: &[u8]) {
        let mut mem_map = self.mem_map.borrow_mut();
        let mut cpu_unit = self.cpu_unit.borrow_mut();
        let clock_unit = self.clock_unit.borrow_mut();

        (clock_unit.tick)(&mut cpu_unit, &mut mem_map, cache_line);
    }

    pub fn write_register(&mut self, name: &str, register_value: RegisterValue) {
        let mut cpu_unit = self.cpu_unit.borrow_mut();
        cpu_unit.registers.write_register(name, register_value);
    }

    pub fn read_register(&mut self, name: &str, register_value: &mut RegisterValue) {
        let mut cpu_unit = self.cpu_unit.borrow_mut();
        cpu_unit.registers.read_register(name, register_value);
    }
}

    pub fn create_console_cpu(rom_type: HalRomType, mem_controller: &mut Arc<&mut HalMemController>) -> Option<ConsoleCpu> {
    match rom_type {
        HalRomType::NesRom => return Some(
            nes_cpu::create_nes_cpu(mem_controller)
        ),
        _ => return None,
    }
    None
}
