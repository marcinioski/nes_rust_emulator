use crate::{CartidgeFlags, HalRom};

const ROM_BANKS_Q_SHIFT: usize = 4;
const ROM_BANKS_Q_NAME: &'static str = "QRomBanks";

const VROM_BANKS_Q_SHIFT: usize = 5;
const VROM_BANKS_Q_NAME: &'static str = "QVromBanks";

const NES_FLAG6_SHIFT: usize = 6;
const NES_FLAG6_NAME: &'static str = "Flag6";

const NES_FLAG6_B0_HV_MIRRORING: u8 = 0;
const NES_FLAG6_B1_BATT_PACKED: u8 = 1<<1;
const NES_FLAG6_B2_TRAINER: u8 = 1<<2;
const NES_FLAG6_B3_4_VRAM_LAYOUT: u8 = 1<<3;
const NES_FLAG6_B4_4_ROM_MAPPER: u8 = 1<<4;
const NES_FLAG6_B4_5_ROM_MAPPER: u8 = 1<<5;
const NES_FLAG6_B4_6_ROM_MAPPER: u8 = 1<<6;
const NES_FLAG6_B4_7_ROM_MAPPER: u8 = 1<<7;

const NES_FLAG7_SHIFT: usize = 7;
const NES_FLAG7_NAME: &'static str = "Flag7";

const NES_FLAG8_Q_RAM_BANKS: usize = 8;
const NES_FLAG8_NAME: &'static str = "Flag8";

const NES_FLAG9_PAL_CARTIDGES: usize = 9;
const NES_FLAG9_NAME: &'static str = "Flag9";

const NES_FLAG10_RESERVED: usize = 10;
const NES_FLAG15_RESERVED: usize = 15;

fn print_flags_info(flags: &std::collections::BTreeMap<&'static str, u8>) {
}

pub fn read_flags(rom: &HalRom) -> Option<CartidgeFlags> {
    if rom.rom_size > ((NES_FLAG15_RESERVED as usize)*std::mem::size_of::<u8>()) {
        let mut rom_flags = std::collections::BTreeMap::new();

        // copying all flags into map
        rom_flags.insert(ROM_BANKS_Q_NAME, rom.memory[ROM_BANKS_Q_SHIFT]);
        rom_flags.insert(VROM_BANKS_Q_NAME, rom.memory[VROM_BANKS_Q_SHIFT]);
        rom_flags.insert(NES_FLAG6_NAME, rom.memory[NES_FLAG6_SHIFT]);
        rom_flags.insert(NES_FLAG7_NAME, rom.memory[NES_FLAG7_SHIFT]);
        rom_flags.insert(NES_FLAG8_NAME, rom.memory[NES_FLAG8_Q_RAM_BANKS]);
        rom_flags.insert(NES_FLAG9_NAME, rom.memory[NES_FLAG9_PAL_CARTIDGES]);

        #[cfg(Debug)]
        print_flags_info(&rom_flags);
        return Some(CartidgeFlags {flags : Some(rom_flags)});
    }
    None
}
