pub struct NesConfig {
    
}

impl NesConfig {
    pub fn new_empty() -> Option<NesConfig> {
        None
    }

    pub fn new_read_config() -> Option<NesConfig> {
        Some( NesConfig {} )
    }
}
