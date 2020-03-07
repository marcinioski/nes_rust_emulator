use crate::interface::{Memory, MemoryController, RawPtr};
use crate::interface::{Rom, RomReader, RomReaderErr};
use std::sync::Arc;
use std::fs;
use std::error;

pub struct X64MemoryController;
pub struct X64RomController;

impl X64MemoryController {
    pub fn new() -> X64MemoryController {
        X64MemoryController {}
    }
}

impl X64RomController {
    pub fn new() -> X64RomController {
        X64RomController {}
    }
}

impl Memory for X64MemoryController {
    fn ctrlmalloc(&mut self, size: usize) -> Option<RawPtr> {
        if size == 0 {
            None
        }
        else {
            Some(Arc::new(vec![0; size].into_boxed_slice()))
        }
    }

    #[warn(unused_variables)]
    fn ctrlfree(&mut self, _ptr: RawPtr) {
    }

    fn ctrl_from_ptr(&mut self, ptr: *const u8, size: usize) -> Option<RawPtr> {
        if size == 0 {
            None
        }
        else {
            unsafe {
                let ptr = ptr as *mut u8;

                Some(Arc::new(Vec::from_raw_parts(ptr, size, size).into_boxed_slice()))
            }
        }
    }
}


impl Rom {
    fn read_file(path: &str) -> Result<Vec<u8>, Box<dyn error::Error + 'static>> {
        let bytes = fs::read(path)?;
        Ok(bytes)
    }
    fn new(memory: RawPtr, rom_size: usize) -> Rom {
        Rom {memory, rom_size}
    }
}

impl RomReader for X64RomController {
    fn read_rom(&mut self, path: &str, mem_controller: &mut Arc<&mut MemoryController>) -> Result<Rom, RomReaderErr> {
        //let mem_controller = mem_controller.clone();
        if let Some(mem_controller) = Arc::get_mut(mem_controller) {

            if path.len() <= 0 {
                return Err(RomReaderErr::NotExist)
            }

            if let Ok(bytes) = Rom::read_file(path) {
                if let Some(ptr) = mem_controller.ctrl_from_ptr(bytes.as_ptr(), bytes.len()) {
                    return Ok(Rom::new(ptr, bytes.len()));
                }
                else {
                    return Err(RomReaderErr::CannotCreate);
                }
            }
            else {
                Err(RomReaderErr::CannotRead)
            } 
        }
        else {
            Err(RomReaderErr::WrongArgs)
        }
    }
}
