use crate::{HalMemController, HalMemory, HalPtr, MemoryMap, MemoryRegion};
use std::sync::Arc;
use std::rc;
use std::cell::RefCell;

const NES_RAM_SIZE: usize = 0x10000;
const NES_RAM_NAME: &'static str = "NES_RAM";

const NES_CARTIDGE_ROM_UPPER_BANK: usize = 0x10000;
const NES_CARTIDGE_ROM_UPPER_BANK_NAME: &'static str = "UPPER_CARTIDGE_BANK";

const NES_CARTIDGE_ROM_LOWER_BANK: usize = 0xC000;
const NES_CARTIDGE_ROM_LOWER_BANK_NAME: &'static str = "LOWER_CARTIDGE_BANK";

const NES_CARTIDGE_RAM: usize = 0x8000;
const NES_CARTIDGE_RAM_NAME: &'static str = "CARTIDGE_RAM";

const NES_EXPANSION_MODULE: usize = 0x6000;
const NES_EXPNASION_MODULE_NAME: &'static str = "EXPANSION_MODULE";

const NES_INPUT_OUTPUT: usize = 0x5000;
const NES_INPUT_OUTPUT_NAME: &'static str = "INPUT_OUTPUT";

const NES_INTERNAL_RAM: usize = 0x2000;
const NES_INTERNAL_RAM_NAME: &'static str = "INTERNAL_RAM";

fn _create_nes_ram_region_names(mem_controller: &mut Arc<&mut HalMemController>, ram_begin: rc::Rc<HalPtr>) ->
Option<std::collections::LinkedList<MemoryRegion>>
{
//    let mut result = std::collections::LinkedList::new();

    return None;
}

fn create_nes_ram_map(mem_controller: &mut Arc<&mut HalMemController>) -> MemoryRegion {
    let mem_controller_unwrapped = Arc::get_mut(mem_controller).unwrap();
    let region_begin = rc::Rc::new(mem_controller_unwrapped.ctrlmalloc(NES_RAM_SIZE).unwrap());
    MemoryRegion {
        region_begin: region_begin.clone(),
        region_size: NES_RAM_SIZE,
        region_name: NES_RAM_NAME,
        region_names: _create_nes_ram_region_names(mem_controller, region_begin.clone())}
}

const NES_PPU_SIZE: usize = 0x4000;
const NES_PPU_NAME: &'static str = "PPU_VRAM";

const NES_PPU_SPRITE_PALETTE: usize = 0x3F20;
const NES_PPU_SPRITE_PALETTE_NAME: &'static str = "PPU_SPRITE_PALLETE";

const NES_PPU_IMAGE_PALETTE: usize = 0x3F10;
const NES_PPU_IMAGE_PALETTE_NAME: &'static str = "PPU_IMAGE_PALETTE";

const NES_PPU_EMPTY: usize = 0x3F00;
const NES_PPU_EMPTY_NAME: &'static str = "PPU_EMPTY";

const NES_PPU_ATTRIBUTE_TABLE_3: usize = 0x3000;
const NES_PPU_ATTRIBUTE_TABLE_3_NAME: &'static str = "PPU_ATTRIBUTE_TABLE_3";

const NES_PPU_NAME_TABLE_3: usize = 0x2FC0;
const NES_PPU_NAME_TABLE_3_NAME: &'static str = "PPU_NAME_TABLE_2";

const NES_PPU_ATTRIBUTE_TABLE_2: usize = 0x2C00;
const NES_PPU_ATTRIBUTE_TABLE_2_NAME: &'static str = "PPU_ATTRIBUTE_TABLE_2";

const NES_PPU_NAME_TABLE_2: usize = 0x2BC0;
const NES_PPU_NAME_TABLE_2_NAME: &'static str = "PPU_NAME_TABLE_2";

const NES_PPU_ATTRIBUTE_TABLE_1: usize = 0x2800;
const NES_PPU_ATTRIBUTE_TABLE_1_NAME: &'static str = "PPU_ATTRIBUTE_TABLE_1";

const NES_PPU_NAME_TABLE_1: usize = 0x27C0;
const NES_PPU_NAME_TABLE_1_NAME: &'static str = "PPU_NAME_TABLE_1";

const NES_PPU_ATTRIBUTE_TABLE_0: usize = 0x2400;
const NES_PPU_ATTRIBUTE_TABLE_0_NAME: &'static str = "PPU_ATTRIBUTE_TABLE_0";

const NES_PPU_NAME_TABLE_0: usize = 0x23C0;
const NES_PPU_NAME_TABLE_0_NAME: &'static str = "PPU_NAME_TABLE_0";

const NES_PPU_PATTERN_TABLE_1: usize = 0x2000;
const NES_PPU_PATTERN_TABLE_1_NAME: &'static str = "PPU_PATTERN_TABLE_1";

const NES_PPU_PATTERN_TABLE_0: usize = 0x1000;
const NES_PPU_PATTERN_TABLE_0_NAME: &'static str = "PPU_PATTERN_TABLE_0";

fn _create_nes_vram_region_names(mem_controller: &mut Arc<&mut HalMemController>, region_begin: rc::Rc<HalPtr>) ->
    Option<std::collections::LinkedList<MemoryRegion>>
{
//    let result = std::collections::LinkedList::new();

//    return Some(result);
    None
}

fn create_nes_vram_map(mem_controller: &mut Arc<&mut HalMemController>) -> MemoryRegion {
    let mem_controller_unwrapped = Arc::get_mut(mem_controller).unwrap();
    let region_begin = rc::Rc::new(mem_controller_unwrapped.ctrlmalloc(NES_PPU_SIZE).unwrap());
    MemoryRegion {
        region_begin: region_begin.clone(),
        region_name: NES_PPU_NAME,
        region_size: NES_PPU_SIZE,
        region_names: _create_nes_vram_region_names(mem_controller, region_begin.clone()),
    }
}

pub fn create_nes_mem_map(mem_controller: &mut Arc<&mut HalMemController>) -> rc::Rc<RefCell<MemoryMap>> {
   rc::Rc::new(RefCell::new(MemoryMap { 
        ram: create_nes_ram_map(mem_controller),
        vram: create_nes_vram_map(mem_controller),
    }))
}

