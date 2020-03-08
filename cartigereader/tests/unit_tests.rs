use cartigereader;
use cartigereader::Cartidge;
use cartigereader::CartidgeErr;
use hal::interface::RomReader;
use hal::interface::RomReaderErr;
use hal::interface::Rom;
use std::sync::Arc;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn read_wrong_rom() {
        let mut mem_controller = hal::create_mem_controller();
        let mut mem_controller = Arc::new(&mut mem_controller);

        let mut rom_controller = hal::create_rom_controller();

        if let Ok(rom) = rom_controller.read_rom("tests/static/wrong_rom.rom", &mut mem_controller){
            if let Err(CartidgeErr::WrongHeader) = Cartidge::new(&rom) {
                ()
            }
            else {
                assert!(false);
            }
        }
        else {
            assert!(false);
        }
    }

    #[test]
    fn read_proper_header() {
    }
}
