pub mod nesconfig;

use nesconfig::NesConfig;

pub struct Nes<'a> {
    config: &'a NesConfig,
}

impl<'a> Nes<'a> {
    pub fn new(config: &mut Option<NesConfig>) -> Result<Nes, &'static str> {
        if let Some(config) = config {
            Ok(Nes {config})
        }
        else
        {
            Err("improper config")
        }
    }
}
