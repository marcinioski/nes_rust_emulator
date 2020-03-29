use cpu;
use cpu::Cpu;
use cpu::ConsoleCpu;
use hal::interface;
use hal::interface::RomType;
use std::sync::Arc;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn create_empty_cpu() {
        let mut mem_controller = hal::create_mem_controller();
        let mut mem_controller = Arc::new(&mut mem_controller);

        let ccpu = cpu::create_console_cpu(RomType::Unknown, &mut mem_controller);
        match ccpu {
            None => (),
            _ => assert!(false),
        }
    }

    #[test]
    fn create_nes_cpu () {
        let mut mem_controller = hal::create_mem_controller();
        let mut mem_controller = Arc::new(&mut mem_controller);

        let ncpu = cpu::create_console_cpu(RomType::NesRom, &mut mem_controller);

        match ncpu {
            Some(_) => (),
            _ => assert!(false),
        }
    }

    #[test]
    fn create_nes_cpu_opcodes () {
        let mut mem_controller = hal::create_mem_controller();
        let mut mem_controller = Arc::new(&mut mem_controller);

        let mut ncpu = cpu::create_console_cpu(RomType::NesRom, &mut mem_controller).unwrap();
        let cache_line = vec![1, 2, 3, 4];
        ncpu.run(&cache_line);
        ncpu.run(&cache_line);
    }

    #[test]
    fn create_nes_cpu_registers () {
        let mut mem_controller = hal::create_mem_controller();
        let mut mem_controller = Arc::new(&mut mem_controller);

        let mut ncpu = cpu::create_console_cpu(RomType::NesRom, &mut mem_controller).unwrap();

        let accu_val = cpu::RegisterValue::R8(0x10);
        let mut accu_mut_val = cpu::RegisterValue::RUnitialized;
        let accu_name = "accumulator";

        ncpu.write_register(accu_name, accu_val);
        ncpu.read_register(accu_name, &mut accu_mut_val);
        assert_eq!(accu_val, accu_mut_val);
    }

    #[test]
    fn cpu_nes_memory_map() {
    }
}
