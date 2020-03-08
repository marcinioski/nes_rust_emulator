use crate::{CheckHeader, ParseFlags, CartidgeFlags, CartidgeErr, HalRom};
mod flags_reader;

const NH_B1: u8 = 0x4E;
const NH_B2: u8 = 0x45;
const NH_B3: u8 = 0x53;
const NH_B4: u8 = 0x1A;

const NES_HEADER: [u8; 4] = [NH_B1, NH_B2, NH_B3, NH_B4];

pub struct CartidgeNes;

impl CheckHeader for CartidgeNes {
    fn check_header(rom: &HalRom) -> Result<(), CartidgeErr> {
        let memory = &rom.memory;
        let size = rom.rom_size;

        if size < 4 {
            return Err(CartidgeErr::WrongHeaderLength);
        }

        if memory[0] == NES_HEADER[0] &&
           memory[1] == NES_HEADER[1] &&
           memory[2] == NES_HEADER[2] &&
           memory[3] == NES_HEADER[3] {
            return Ok(());
        }

        Err(CartidgeErr::WrongHeader)
    }
}

impl ParseFlags for CartidgeNes {
    fn parse_flags(rom: &HalRom) -> Option<CartidgeFlags> {
        return flags_reader::read_flags(&rom);
    }
}
