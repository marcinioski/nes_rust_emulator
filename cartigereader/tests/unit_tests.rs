use cartigereader;
use cartigereader::Cartidge;
use cartigereader::CartidgeErr;
use hal::interface::RomReader;
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
                println!("Could not read wrong header!");
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
        let mut mem_controller = hal::create_mem_controller();
        let mut mem_controller = Arc::new(&mut mem_controller);
        let mut rom_controller = hal::create_rom_controller();

        if let Ok(rom) = rom_controller.read_rom("tests/static/contra.nes", &mut mem_controller) {
            if let Ok(_) = Cartidge::new(&rom) {
                println!("Proper header!");
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
    fn parsing_rom_flags() {
        let mut mem_controller = hal::create_mem_controller();
        let mut mem_controller = Arc::new(&mut mem_controller);
        let mut rom_controller = hal::create_rom_controller();

        if let Ok(rom) = rom_controller.read_rom("tests/static/contra.nes", &mut mem_controller) {
            if let Ok(cartidge) = Cartidge::new(&rom) {
                assert_eq!(cartidge.flags.is_some(), true);
                let flags = cartidge.flags.as_ref();
                if let Some(flags) = flags {
                    if let Some(flags) = &flags.flags {
                        for (k, v) in flags.iter( ){
                            println!("{} {}", k, v);
                        }
                    }
                }
            }
            else {
                assert!(false);
            }
        }
        else {
            assert!(false);
        }
    }
}
