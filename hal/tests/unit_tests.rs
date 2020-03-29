use hal;
use hal::interface::Memory;
use hal::interface::RomReader;
use hal::interface::RomReaderErr;
use hal::interface::Rom;
use std::sync::Arc;
use std::mem;
use std::env;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn malloc_test() {
        let mut mem_controller = hal::create_mem_controller();
        if let Some(_) = mem_controller.ctrlmalloc(0) {
            assert!(false);
        }

        if let Some(mut data) = mem_controller.ctrlmalloc(20) {
            assert_eq!(Arc::get_mut(&mut data).unwrap().len(), 20);
            Arc::get_mut(&mut data).unwrap()[0] = 10;
            assert_eq!(Arc::get_mut(&mut data).unwrap()[0], 10);
        }
        else {
            assert!(false);
        }
    }
    #[test]
    fn malloc_from_ptr_test() {
        let mut controller = hal::create_mem_controller();

        let test_vec: Vec<u8> = vec![0xAA; 512];

        // manually drop destructor
        let test_vec = mem::ManuallyDrop::new(test_vec);

        let tst_ptr = test_vec.as_ptr();
        let tst_len = test_vec.len();

        if let Some(mut data) = controller.ctrl_from_ptr(tst_ptr, tst_len) {
            assert_eq!(Arc::get_mut(&mut data).unwrap().len(), test_vec.len());
            assert_eq!(Arc::get_mut(&mut data).unwrap()[0], 0xAA);
            assert_eq!(Arc::get_mut(&mut data).unwrap()[test_vec.len() - 1], 0xAA);
        }
    }

    #[test]
    fn malloc_region_from_slice() {
        let mut controller = hal::create_mem_controller();

        let mut test_vec: Vec<u8> = vec![0xAA; 512];
        let test_shift: usize = 20;
        let test_len: usize = 20;

        test_vec[test_shift] = 0xCC;
        test_vec[test_shift + test_len] = 0xFF;

        // manually drop destructor
        let test_vec = mem::ManuallyDrop::new(test_vec);

        let tst_ptr = test_vec.as_ptr();
        let tst_len = test_vec.len();
        let tst_ptr_val = tst_ptr as usize;

        if let Some(mut data) = controller.ctrl_from_ptr(tst_ptr, tst_len) {
//            let mut _from_slice = controller.ctrl_ptr_from_slice(&mut data, test_len, test_shift).unwrap();
//            assert_eq!(Arc::get_mut(&mut data).unwrap()[test_shift], Arc::get_mut(&mut from_slice).unwrap()[0]);
//            assert_eq!(Arc::get_mut(&mut data).unwrap()[test_shift + test_len], Arc::get_mut(&mut from_slice).unwrap()[test_len - 1]);
        }
    }

    #[test]
    fn rom_reader_test() {
        let mut mem_controller = hal::create_mem_controller();
        let mut mem_controller = Arc::new(&mut mem_controller);

        let mut rom_controller = hal::create_rom_controller();
        if let Err(RomReaderErr::NotExist) = rom_controller.read_rom("", &mut mem_controller) {
            ()
        }
        else {
            assert!(false);
        }

        if let Err(RomReaderErr::CannotRead) = rom_controller.read_rom("unexist_rom", &mut mem_controller) {
            ()
        }
        else {
            assert!(false);
        }

        if let Ok(_rom) = rom_controller.read_rom("tests/static/wrong_input.rom", &mut mem_controller) {
            ()
        }
        else {
            assert!(false);
        }
    }
}
