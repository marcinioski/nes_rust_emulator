use std::sync::Arc;
pub type RawPtr = Arc<Box<[u8]>>;

pub trait Memory {
    fn ctrlmalloc(&mut self, size: usize) -> Option<RawPtr>;
    fn ctrlfree(&mut self, ptr: RawPtr);
    fn ctrl_from_ptr(&mut self, ptr: *const u8, size: usize) -> Option<RawPtr>;
}

#[cfg(target_arch = "x86_64")]
pub mod x64;

pub struct MemoryController {
    #[cfg(target_arch = "x86_64")]
    controller: x64::X64MemoryController,
    #[cfg(target_arch = "arm")]
    controller: stm::STMMemoryController,
}

pub struct RomController {
    #[cfg(target_arch = "x86_64")]
    controller: x64::X64RomController,
}

impl MemoryController {
    pub fn new() -> MemoryController {
        MemoryController { controller: x64::X64MemoryController {} } 
    }
}

impl RomController {
    pub fn new() -> RomController {
        RomController {controller: x64::X64RomController {} }
    }
}

impl Memory for MemoryController {
    fn ctrlmalloc(&mut self, size: usize) -> Option<RawPtr> {
        self.controller.ctrlmalloc(size)
    }

    fn ctrlfree(&mut self, ptr: RawPtr) {
        self.controller.ctrlfree(ptr);
    }

    fn ctrl_from_ptr(&mut self, ptr: *const u8, size: usize) -> Option<RawPtr> {
        self.controller.ctrl_from_ptr(ptr, size)
    }
}

pub enum RomReaderErr {
    NotExist,
    CannotRead,
    CannotCreate,
    WrongArgs,
}

pub struct Rom {
    pub memory: RawPtr,
    pub rom_size: usize,
}

pub trait RomReader {
    fn read_rom(&mut self, path: &str, mem_controller: &mut Arc<&mut MemoryController>) -> Result<Rom, RomReaderErr>;
}

impl RomReader for RomController {
    fn read_rom(&mut self, path: &str, mem_controller: &mut Arc<&mut MemoryController>) -> Result<Rom, RomReaderErr> {
        self.controller.read_rom(path, mem_controller)
    }
}
