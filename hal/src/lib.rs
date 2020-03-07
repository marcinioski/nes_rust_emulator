pub mod interface;
pub use crate::interface::Memory;

pub fn create_mem_controller() -> interface::MemoryController {
    interface::MemoryController::new()
}

pub fn create_rom_controller() -> interface::RomController {
    interface::RomController::new()
}
