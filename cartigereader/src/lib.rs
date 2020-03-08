use std::sync::Arc;
use std::result::Result;
pub use hal::interface::RomType as HalRomType;
pub use hal::interface::Rom as HalRom;

mod nes_cartidge;

pub enum CartidgeErr {
    WrongArgs,
    UnknownRomType,
    WrongHeader,
    WrongHeaderLength,
    WrongHeaderFlags,
    WrongChecksum,
}

pub trait CheckHeader {
    fn check_header(rom: &HalRom) -> Result<(), CartidgeErr>;
}

pub struct CartidgeFlags {
    pub flags: Option<std::collections::BTreeMap<&'static str, u8>>,
}

pub trait ParseFlags {
    fn parse_flags(rom: &HalRom) -> Option<CartidgeFlags>;
}

pub struct Cartidge<'a> {
    pub rom: &'a HalRom,
    pub flags: Option<CartidgeFlags>,
}

impl<'a> Cartidge<'a> {
    pub fn new(rom: &HalRom) -> Result<Arc<Cartidge>, CartidgeErr> {
        if let Ok(_) = Cartidge::check_header(rom) {
            Ok(Arc::new(Cartidge {rom,
                                  flags: Cartidge::parse_flags(rom)} ))
        }
        else {
            Err(CartidgeErr::WrongHeader)
        }
    }
}

impl<'a> CheckHeader for Cartidge<'a> {
    fn check_header(rom: &HalRom) -> Result<(), CartidgeErr> {
        match rom.rom_type {
            HalRomType::NesRom => return nes_cartidge::CartidgeNes::check_header(rom),
            _ => return Err(CartidgeErr::UnknownRomType),
        }
    }
}

impl<'a> ParseFlags for Cartidge<'a> {
    fn parse_flags(rom: &HalRom) -> Option<CartidgeFlags> {
        match rom.rom_type {
            HalRomType::NesRom => return nes_cartidge::CartidgeNes::parse_flags(rom),
            _ => return None,
        }
    }
}
