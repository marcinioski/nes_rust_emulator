use std::rc::Rc;
use std::cell::RefCell;
use std::marker::PhantomData;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RegisterValue {
    R8(u8),
    R16(u16),
    R32(u32),
    RUnitialized,
}

pub struct Register<T> {
    register_value: RegisterValue,
    register_name: String,
    value: T,
    phantom: PhantomData<T>,
}

pub trait RegisterStore {
    fn store(&mut self, value: RegisterValue);
}

pub trait RegisterLoad {
    fn load(&self, value: &mut RegisterValue);
}

impl<T> RegisterStore for Register<T> {
    fn store(&mut self, value: RegisterValue) {
        self.register_value = value;
    }
}

impl<T> RegisterLoad for Register<T> {
    fn load(&self, value: &mut RegisterValue) {
        *value = self.register_value;
    }
}

impl<'a, T> Register<T>
    where T: Copy + PartialOrd + Default {

    pub fn get_name(&self) -> &str {
        &self.register_name
    }

    pub fn new(name: &str) -> Register<T> {
        Register {
            register_value: RegisterValue::RUnitialized,
            register_name: String::from(name),
            value: std::default::Default::default(),
            phantom: PhantomData,
        }
    }
}

pub enum CpuRegister {
    R8(Register<u8>),
    R16(Register<u16>),
    Unknown,
}

impl CpuRegister {
    fn get_name(&self) -> & str {
        match self {
            CpuRegister::R8(register) => register.get_name(),
            CpuRegister::R16(register) => register.get_name(),
            _ => "Unknown"
        }
    }
}

impl RegisterStore for CpuRegister {
    fn store(&mut self, value: RegisterValue) {
        match self {
            CpuRegister::R8(register) => register.store(value),
            CpuRegister::R16(register) => register.store(value),
            _ => (),
        }
    }
}

impl RegisterLoad for CpuRegister {
    fn load(&self, value: &mut RegisterValue) {
        match self {
            CpuRegister::R8(register) => register.load(value),
            CpuRegister::R16(register) => register.load(value),
            _ => (),
        }
    }
}

pub struct CpuRegisters {
    registers: Vec<Rc<RefCell<CpuRegister>>>,
}

impl CpuRegisters {
    pub fn new(registers: Vec<Rc<RefCell<CpuRegister>>>) -> CpuRegisters {
        CpuRegisters {
            registers,
        }
    }

    fn get_register(&mut self, register_name: &str) -> Rc<RefCell<CpuRegister>> {
        for register in self.registers.iter() {
            let register_borrow = register.borrow();
            let register_iter_name = register_borrow.get_name();
            if register_name.contains(register_iter_name) {
                return register.clone();
            }
        }
        return Rc::new(RefCell::new(CpuRegister::Unknown));
    }

    pub fn write_register(&mut self, name: &str, register_value: RegisterValue) {
        self.get_register(name).borrow_mut().store(register_value);
    }

    pub fn read_register(&mut self, name: &str, register_value: &mut RegisterValue) {
        self.get_register(name).borrow_mut().load(register_value);
    }
}


